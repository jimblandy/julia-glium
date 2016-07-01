# `julia-glium`: A GPU-based Julia set plotter in Rust

This program plots the Julia set for the current mouse location, using the GPU
(or, an OpenGL fragment shader, at least) for the per-pixel calculations. It's
written in Rust, using the [glium][1] OpenGL bindings.

[1]: https://crates.io/crates/glium
