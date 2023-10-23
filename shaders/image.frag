#version 450

#define FRAGMENT

#include "image_bindings.glsl"

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

void main() {
    const vec4 fill = texture(sampler2D(image_texture, default_sampler), in_uv);
    out_color = fill;
}