#version 450

layout(set = 0, binding = 1) uniform sampler default_sampler;

layout (set = 1, binding = 0) uniform ModelInfo {
    mat4 transform;
    vec4 fill_color;
} model_info;

layout (set = 1, binding = 1) uniform texture2D glyph_texture;

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

// #define epsilon float(1e-37)
// #define scalar (128.0f / 255.0f)

void main() {
    vec4 fill = model_info.fill_color;
    float d = texture(sampler2D(glyph_texture, default_sampler), in_uv).r;
    float aaf = fwidth(d) / 2;

    float alpha = smoothstep(0.5f - aaf, 0.5f + aaf, d);
    //float alpha = step(0.5f, d);

    fill.a *= alpha;

    out_color = fill;
    // out_color = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}