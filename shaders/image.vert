#version 450

#define VERTEX

#include "image_bindings.glsl"

layout(location = 0) out vec2 out_uv;

vec2 verticies[6] = vec2[](
    vec2(0.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 1.0)
);

vec2 uvs[6] = vec2[](
    vec2(0.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 1.0)
);


void main() {
    vec2 vtx = verticies[gl_VertexIndex];
    vec2 uv = uvs[gl_VertexIndex];

    out_uv = uv;

    gl_Position = render_info.projection * image_info.transform * vec4(vtx, 0.0f, 1.0f);
}