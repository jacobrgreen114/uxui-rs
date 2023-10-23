
#include "global_bindings.glsl"

layout (set = 1, binding = 0) uniform GlyphInfo {
    mat4 transform;
} image_info;


#ifdef FRAGMENT
layout (set = 1, binding = 1) uniform texture2D image_texture;
#endif