#version 450

layout (set = 0, binding = 0) uniform RenderInfo {
    mat4 Projection;
//mat4 View;
} renderInfo;

layout (set = 1, binding = 0) uniform Transform {
    mat4 Model;
    vec4 Color;
} transform;

layout(location = 0) out vec4 fragColor;

vec2 positions[6] = vec2[](
vec2(0.0, 0.0),
vec2(0.0, 1.0),
vec2(1.0, 0.0),
vec2(1.0, 0.0),
vec2(0.0, 1.0),
vec2(1.0, 1.0)
);



void main() {
    fragColor = transform.Color;
    gl_Position = renderInfo.Projection * transform.Model * vec4(positions[gl_VertexIndex], 0.0f, 1.0f);
    // gl_Position = vec4(positions[gl_VertexIndex], 0.0f, 0.5f);
}