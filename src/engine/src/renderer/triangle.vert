#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main() {

    // Takes in the position of the input vertices and places them in a vector4
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}
