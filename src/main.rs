#[macro_use]
extern crate glium;
extern crate image;

mod teapot;

use glium::{DisplayBuild, Surface};
use std::f32::consts::PI;
use std::io::Cursor;

fn main() {
    let mut builder = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24);
    builder.opengl.vsync = true;

    let display = builder.build_glium()
        .expect("failed to build glium window");

    let image = image::load(Cursor::new(&include_bytes!("/home/jimb/rust/tut-glium/src/mandel.png")[..]),
                            image::PNG)
        .expect("loading image")
        .to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                   image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image)
        .expect("creating texture");

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES)
        .expect("building positions");
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS)
        .expect("building normals");
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES)
        .expect("building indices");

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              &vertex_shader_src,
                                              &fragment_shader_src,
                                              None)
        .expect("building program");

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [ 0.01, 0.0, 0.0, 0.0 ],
            [ 0.0, 0.01, 0.0, 0.0 ],
            [ 0.0, 0.0, 0.01, 0.0 ],
            [ 0.0, 0.0, 0.0, 1.0f32 ]
        ];
        let light = [-1.0, 0.4, 0.9f32];
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { matrix: matrix, u_light: light },
                    &params)
            .expect("target.draw");
        target.finish().expect("target.finish");

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
