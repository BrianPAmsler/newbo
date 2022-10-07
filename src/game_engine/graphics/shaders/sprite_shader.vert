#version 330 core
#define SPRITES_IN_SHEET %v1%
#define SPRITE_COUNT %v2%

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

uniform vec4 sprite_info[SPRITES_IN_SHEET];
uniform vec4 sprites[SPRITE_COUNT];
uniform int sprite_id[SPRITE_COUNT];

smooth out vec2 tex_coord;

void main() {
    vec2 offset = sprites[gl_InstanceID].xy;
    vec2 size = sprites[gl_InstanceID].zw;
    vec3 p = pos * size;

    gl_Position = vec4(p.xy + offset, p.z, 1.0);

    vec2 uv_tl = sprite_info[gl_InstanceID].xy;
    vec2 uv_w = sprite_info[gl_InstanceID].zw;

    tex_coord = uv_tl + uv * uv_w;
}