#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;

void main() {

    // Takes in the position of the input vertices and places them in a vector4
    gl_Position = vec4(Position, 1.0);
}
