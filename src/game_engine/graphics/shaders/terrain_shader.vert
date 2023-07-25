#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

uniform mat4 transform;

smooth out vec4 pixelColor;

void main() {
    gl_Position = transform * vec4(pos, 1.0);

    pixelColor = vec4(color, 1.0);
}