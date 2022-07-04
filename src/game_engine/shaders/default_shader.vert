#version 330 core

layout (location = 0) in vec3 pos;

uniform vec3 offset;

void main() {
    gl_Position = vec4(pos.x + offset.x, pos.y + offset.y, pos.z + offset.z, 1.0);
}