#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TexCoordinates;

uniform vec2 Offset;
uniform vec4 VertexColor;

uniform uint UsePosition;
uniform uint UseVertexColors;
uniform uint ReverseShape;

out VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} OUT;

void main() {

    // Takes in the position of the input vertices and places them in a vector4
    gl_Position = ReverseShape == 1 ? vec4(-Position.xy + Offset.xy, -Position.z, 1.0) : vec4(Position.xy + Offset.xy, Position.z, 1.0);

    OUT.Color = UsePosition == 0 ? (UseVertexColors == 1 ? vec4(Color, 1.0) : VertexColor) : gl_Position;

    OUT.TexCoord = TexCoordinates;
}