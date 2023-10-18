#version 450

#define VERTEX

#include "rect_bindings.glsl"

layout(location = 0) out vec4 frag_color;

vec2 verticies[6] = vec2[](
vec2(0.0, 0.0),
vec2(0.0, 1.0),
vec2(1.0, 0.0),
vec2(1.0, 0.0),
vec2(0.0, 1.0),
vec2(1.0, 1.0)
);



void main() {
    frag_color = rect_info.color_fill;
    gl_Position = render_info.projection * rect_info.transform * vec4(verticies[gl_VertexIndex], 0.0f, 1.0f);
    // gl_Position = vec4(positions[gl_VertexIndex], 0.0f, 0.5f);
}