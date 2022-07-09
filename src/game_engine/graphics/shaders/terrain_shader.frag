#version 330 core

smooth in vec4 pixelColor;

out vec4 outputColor;

void main() {
    outputColor = pixelColor;
}