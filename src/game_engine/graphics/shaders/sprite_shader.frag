#version 330 core

uniform sampler2D MainTex;

smooth in vec2 tex_coord;

out vec4 outputColor;

void main() {
    outputColor = texture(MainTex, tex_coord);
}