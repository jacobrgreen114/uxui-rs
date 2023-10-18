#version 450

#define FRAGMENT

#include "glyph_bindings.glsl"

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

float SHARPNESS = 1.5;

void main() {
    vec4 fill = glyph_info.fill_color;
    float d = texture(sampler2D(glyph_texture, default_sampler), in_uv).r;
    float aaf = fwidth(d) / SHARPNESS;

    float alpha = smoothstep(0.5f - aaf, 0.5f + aaf, d);
    //float alpha = step(0.5f, d);

    fill.a *= alpha;

    out_color = fill;
}