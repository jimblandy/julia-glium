#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    position: (f32, f32)
}

implement_vertex!(Point, position);

/// The four corners of OpenGL screen space.
const VERTICES: [Point; 4] = [
    Point { position: (  1.0,  1.0 ) },
    Point { position: ( -1.0,  1.0 ) },
    Point { position: ( -1.0, -1.0 ) },
    Point { position: (  1.0, -1.0 ) },
];

/// Two triangles arranged as a rectangle covering OpenGL screen space.
/// All the real work happens in the fragment shader, so we just want to
/// run the shader on every pixel.
const INDICES: [u16; 6] = [
    0, 1, 2,
    0, 2, 3
];

fn main() {
    let mut builder = glium::glutin::WindowBuilder::new();
    builder.opengl.vsync = true;

    let display = builder.build_glium()
        .expect("failed to build glium window");

    let positions = glium::VertexBuffer::new(&display, &VERTICES)
        .expect("building positions");
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &INDICES)
        .expect("building indices");

    let vertex_shader_src = r#"
        #version 150

        uniform vec2 screen_to_complex;
        in vec2 position;
        out vec2 fragment_z;

        void main() {
            fragment_z = screen_to_complex * position;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        uniform vec2 c;
        in vec2 fragment_z;
        out vec4 color;

        // Complex multiplication.
        vec2 cmul(vec2 a, vec2 b) {
            return vec2(a[0] * b[0] - a[1] * b[1],
                        a[0] * b[1] + a[1] * b[0]);
        }

        void main() {
            vec2 z = fragment_z;
            int it = 0;
            const int limit = 150;
            for (it = 0; it < limit; it++) {
                z = cmul(z, z) + c;
                z[0] += dot(z, z);
                if (dot(z, z) > 4.0)
                    break;
            }

            // Map the iteration count to value between 0 and 1.
            float gray;
            if (it >= limit) {
                gray = 1.0;
            } else {
                gray = float(it) / float(limit);
            }

            // Brighten things up a bit: invert, cube to push it towards zero,
            // and revert.
            gray = 1.0 - gray;
            gray = gray * gray * gray;

            color = vec4(gray, gray, gray, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              &vertex_shader_src,
                                              &fragment_shader_src,
                                              None)
        .expect("building program");

    let mut dimensions = display.get_framebuffer_dimensions();
    let mut aspect = dimensions.0 as f32 / dimensions.1 as f32;
    let mut c = [ 0.0, 0.0f32 ];

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let params = Default::default();
        target.draw(&positions, &indices, &program,
                    &uniform! {
                        screen_to_complex: if aspect < 1.0 {
                            [ 2.0, 2.0 / aspect ]
                        } else {
                            [ 2.0 * aspect, 2.0 ]
                        },
                        c: c },
                    &params)
            .expect("target.draw");
        target.finish().expect("target.finish");

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(w, h) => {
                    dimensions = (w, h);
                    aspect = dimensions.0 as f32 / dimensions.1 as f32;
                }
                glium::glutin::Event::MouseMoved(x,y) => {
                    // Map pixels to complex coordinates, keeping the circle of radius
                    // two centered at the origin in the middle of the image.
                    if dimensions.0 > dimensions.1 {
                        // Window is wider than it is high.
                        c[0] = (x as f32 / dimensions.0 as f32 - 0.5) * 2.0 * aspect;
                        c[1] = (y as f32 / dimensions.1 as f32 - 0.5) * 2.0;
                    } else {
                        // Window is higher than it is wide.
                        c[0] = (x as f32 / dimensions.0 as f32 - 0.5) * 2.0;
                        c[1] = (y as f32 / dimensions.1 as f32 - 0.5) * 2.0 / aspect;
                    }
                },
                _ => ()
            }
        }
    }
}
