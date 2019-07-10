#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform vec2 Offset;
uniform vec3 VertexColor;
uniform uint UsePosition;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main() {

    // Takes in the position of the input vertices and places them in a vector4

    gl_Position = vec4(Position.xy + Offset.xy, Position.z, 1.0);

    if (UsePosition == 1) {
        OUT.Color = gl_Position;
    } else {

        OUT.Color = vec4(Color, 1.0);
    }
}
