#version 410 core

in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;

uniform sampler2D Texture1;
uniform sampler2D Texture2;
uniform float opacity;
uniform uint usingTextures;

out vec4 Color;

void main() {

    Color = usingTextures == 1 ? (mix(texture(Texture1, IN.TexCoord), texture(Texture2, IN.TexCoord), opacity) * IN.Color) : IN.Color;
}