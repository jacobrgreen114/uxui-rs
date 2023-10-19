#version 450

#define FRAGMENT

#include "glyph_bindings.glsl"

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

const float SHARPNESS = 1.5;

void main() {
    const float dist = texture(sampler2D(glyph_texture, default_sampler), in_uv).r;
    const float anti_alias = fwidth(dist) / SHARPNESS;
    const float alpha = smoothstep(0.5f - anti_alias, 0.5f + anti_alias, dist);

    out_color = glyph_info.fill_color;
    out_color.a *= alpha;
}