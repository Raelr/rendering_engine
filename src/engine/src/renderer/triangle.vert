#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoordinates;

uniform vec2 Offset;
uniform mat4 Transform;
uniform vec4 Color;

out VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} OUT;

void main() {

    gl_Position = Transform * vec4(Position.xy, 0.0, 1.0);

    OUT.Color = Color;

    OUT.TexCoord = TexCoordinates;
}