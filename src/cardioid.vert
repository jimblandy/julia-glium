#version 150

uniform vec2 screen_to_complex;
in vec2 position;
out vec2 fragment_pt;

void main() {
    fragment_pt = screen_to_complex * position;
    gl_Position = vec4(position, 0.0, 1.0);
}
