#[macro_use]
extern crate glium;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use glium::{Surface, Program};
use glium::backend::Facade;
use glium::glutin::{Event, WindowEvent};

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
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Julia");
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop)
        .expect("failed to build glium window");

    let positions = glium::VertexBuffer::new(&display, &VERTICES)
        .expect("building positions");
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &INDICES)
        .expect("building indices");

    let mut program = Program::from_source(&display,
                                           &include_str!("julia.vert"),
                                           &include_str!("julia.frag"),
                                           None)
        .expect("building program");

    let mut dimensions = display.get_framebuffer_dimensions();
    let mut aspect = dimensions.0 as f32 / dimensions.1 as f32;
    let mut c = [ 0.0, 0.0f32 ];

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

        let mut should_return = false;
        event_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                    should_return = true;
                }
                Event::WindowEvent { event: WindowEvent::Resized(w, h), .. } => {
                    dimensions = (w, h);
                    aspect = dimensions.0 as f32 / dimensions.1 as f32;
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseMoved {
                        position: (x,y), ..
                    }, ..
                } => {
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
        });

        if should_return {
            return;
        }
    }
}

#[derive(Debug)]
struct ShaderError;

impl From<std::io::Error> for ShaderError {
    fn from(_e: io::Error) -> ShaderError {
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
