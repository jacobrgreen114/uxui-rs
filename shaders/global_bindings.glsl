
layout (set = 0, binding = 0) uniform RenderInfo {
    mat4 projection;
} render_info;

#ifdef FRAGMENT
layout(set = 0, binding = 1) uniform sampler default_sampler;
#endif