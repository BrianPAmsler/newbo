#version 330 core
#define SPRITE_COUNT $instance_count

uniform sampler2D MainTex;

smooth in vec2 tex_coord;
flat in int id;

out vec4 outputColor;

void main() {
    outputColor = texture(MainTex, tex_coord) * float(id > 0);
}