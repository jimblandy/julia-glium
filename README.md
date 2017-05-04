# julia-glium: A GPU-based Julia set plotter in Rust

This program plots the Julia set for the current mouse location, using the GPU
(or, an OpenGL fragment shader, at least) for the per-pixel calculations. It's
written in Rust, using the [glium][1] OpenGL bindings.

In Rust 1.17, you should be able to just type:

    $ cargo run

and have it open up a window. Move the mouse around in the window.

[1]: https://crates.io/crates/glium
