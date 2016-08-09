#version 150

uniform vec2 c;
in vec2 fragment_z;
out vec4 color;

// Complex multiplication.
vec2 cmul(vec2 a, vec2 b) {
    return vec2(a[0] * b[0] - a[1] * b[1],
                a[0] * b[1] + a[1] * b[0]);
}

// Complex reciprocal.
vec2 cinv(vec2 a) {
    float norm = dot(a, a);
    return vec2(a[0] / norm, -a[1] / norm);
}

void main() {
    vec2 z = cinv(fragment_z);
    int it = 0;
    const int limit = 100;
    for (it = 0; it < limit; it++) {
        z = cmul(z, z) + c;
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
