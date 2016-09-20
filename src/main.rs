#[macro_use]
extern crate glium;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use glium::{DisplayBuild, Surface, Program};
use glium::backend::Facade;

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

    let mut dimensions = display.get_framebuffer_dimensions();
    let mut aspect = dimensions.0 as f32 / dimensions.1 as f32;
    let mut c = [ 0.0, 0.0f32 ];

    let mut program = load_shader_program(&display).unwrap();

    loop {
        if let Ok(p) = load_shader_program(&display) {
            program = p;
        }

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

#[derive(Debug)]
struct ShaderError;

impl From<std::io::Error> for ShaderError {
    fn from(e: io::Error) -> ShaderError {
        ShaderError
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn load_shader_program<F: Facade>(display: &F) -> Result<Program, ShaderError> {
    let vert_shader = try!(read_file("src/julia.vert"));
    let frag_shader = try!(read_file("src/julia.frag"));
    if let Ok(p) = glium::Program::from_source(display, &vert_shader, &frag_shader, None) {
        Ok(p)
    } else {
        Err(ShaderError)
    }
}
