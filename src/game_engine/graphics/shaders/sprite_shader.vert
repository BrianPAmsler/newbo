#version 330 core
#define SPRITES_IN_SHEET $sheet_size
#define SPRITE_COUNT $instance_count

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

uniform vec4 sprite_info[SPRITES_IN_SHEET];
uniform vec4 sprites[SPRITE_COUNT];
uniform int sprite_id[SPRITE_COUNT];

smooth out vec2 tex_coord;

void main() {
    vec2 offset = sprites[gl_InstanceID].xy;
    vec2 size = sprites[gl_InstanceID].zw;
    vec3 p = pos * vec3(size, 1);

    gl_Position = vec4(p.xy + offset, p.z, 1.0);

    int s_id = sprite_id[gl_InstanceID];
    vec2 uv_bl = sprite_info[s_id].xy;
    vec2 uv_w = sprite_info[s_id].zw;

    tex_coord = uv_bl + uv * uv_w;
}