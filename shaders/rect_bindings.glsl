
#include "global_bindings.glsl"

layout (set = 1, binding = 0) uniform RectInfo {
    mat4 transform;
    vec4 color_fill;
} rect_info;