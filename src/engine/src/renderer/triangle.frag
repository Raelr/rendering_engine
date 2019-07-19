#version 410 core

in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;

uniform sampler2D ourTexture;

out vec4 Color;

void main() {

    Color = texture(ourTexture, IN.TexCoord);
}