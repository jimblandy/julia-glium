#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    position: (f32, f32)
}

implement_vertex!(Point, position);

const VERTICES: [Point; 4] = [
    Point { position: (  1.0,  1.0 ) },
    Point { position: ( -1.0,  1.0 ) },
    Point { position: ( -1.0, -1.0 ) },
    Point { position: (  1.0, -1.0 ) },
];

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

        in vec2 position;
        out vec2 v_position;

        void main() {
            v_position = position;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec2 v_position;
        uniform vec2 c;
        out vec4 color;

        vec2 cmul(vec2 a, vec2 b) {
            return vec2(a[0] * b[0] - a[1] * b[1],
                        a[0] * b[1] + a[1] * b[0]);
        }

        void main() {
            vec2 z = v_position * 2;

            int it = 0;
            const int limit = 100;
            for (it = 0; it < limit; it++) {
                z = cmul(z, z) + c;
                if (dot(z, z) > 4.0)
                    break;
            }

            float gray;
            if (it >= limit) {
                gray = 0.0;
            } else {
                gray = float(it) / float(limit);
            }

            color = vec4(gray, gray, gray, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              &vertex_shader_src,
                                              &fragment_shader_src,
                                              None)
        .expect("building program");

    let mut dimensions = display.get_framebuffer_dimensions();
    let mut c = [ 0.0, 0.0f32 ];
    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let params = Default::default();
        target.draw(&positions, &indices, &program,
                    &uniform! { c: c },
                    &params)
            .expect("target.draw");
        target.finish().expect("target.finish");

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(w, h) => {
                    dimensions = (w, h);
                }
                glium::glutin::Event::MouseMoved(x,y) => {
                    c[0] = -2.0 + (x as f32 / dimensions.0 as f32) * 4.0;
                    c[1] =  2.0 - (y as f32 / dimensions.1 as f32) * 4.0;
                },
                _ => ()
            }
        }
    }
}
