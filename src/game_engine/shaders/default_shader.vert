#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

uniform vec3 offset;

smooth out vec4 pixelColor;

void main() {
    gl_Position = vec4(pos.x + offset.x, pos.y + offset.y, pos.z + offset.z, 1.0);

    pixelColor = vec4(color.r, color.g, color.b, 1.0);
}