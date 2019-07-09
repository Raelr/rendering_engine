#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

uniform float HorizontalOffset;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main() {

    // Takes in the position of the input vertices and places them in a vector4
    gl_Position = vec4(Position.x + HorizontalOffset, Position.yz, 1.0);
    OUT.Color = gl_Position;
}
