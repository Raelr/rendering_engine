#version 410 core

// Takes in the input vectors and translates them into a vector3 coordinate.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoordinates;

uniform vec2 Offset;
uniform mat4 Model;
uniform mat4 Projection;
uniform mat4 View;
uniform vec4 Color;

out VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} OUT;

void main() {

    gl_Position = Projection * View * Model * vec4(Position.xy, 0.0, 1.0);

    OUT.Color = Color;

    OUT.TexCoord = TexCoordinates;
}