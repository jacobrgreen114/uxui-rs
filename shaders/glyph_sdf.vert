#version 450

layout (set = 0, binding = 0) uniform RenderInfo {
    mat4 projection;
} render_info;

layout (set = 1, binding = 0) uniform ModelInfo {
    mat4 transform;
    vec4 color_fill;
} model_info;

layout(location = 0) out vec2 out_uv;


vec2 positions[6] = vec2[](
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
    vec2 vertexPos = positions[gl_VertexIndex];
    vec2 uv = uvs[gl_VertexIndex];

    out_uv = uv;

    gl_Position = render_info.projection * model_info.transform * vec4(vertexPos, 0.0f, 1.0f);
}