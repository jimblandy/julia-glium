#version 150

uniform vec2 c;
in vec2 fragment_pt;
out vec4 color;

float color_distance(float d) {
  float inv = 0.1 / abs(d);
  if (inv > 1.0) {
    return 0.0;
  } else {
    return 1.0 - inv;
  }
}

float sq(float x) { return x * x; }

void main() {
    vec2 pt = fragment_pt;
    float a = c.y;
    float dist = sq(sq(pt.x) + sq(pt.y) - sq(a)) - c.x * sq(a) * (sq(pt.x-a) + sq(pt.y));
    if (dist <= 6.5) {
      dist = fract(5 * dist);
    }
    color = vec4(vec3(color_distance(dist)), 1.0);
}
