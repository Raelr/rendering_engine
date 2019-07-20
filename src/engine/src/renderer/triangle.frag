#version 410 core

in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;

uniform sampler2D Texture1;
uniform sampler2D Texture2;

out vec4 Color;

void main() {

    Color = mix(texture(Texture1, IN.TexCoord), texture(Texture2, IN.TexCoord), 0.2) * IN.Color;
}