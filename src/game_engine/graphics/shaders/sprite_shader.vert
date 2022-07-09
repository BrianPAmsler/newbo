#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

uniform vec2 pos;

smooth out vec2 texCoord;

void main() {
    gl_Position = vec4(pos.x + offset.x, pos.y + offset.y, pos.z + offset.z, 1.0);

    texCoord = uv;
}