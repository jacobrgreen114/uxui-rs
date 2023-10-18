
#include "global_bindings.glsl"

layout (set = 1, binding = 0) uniform GlyphInfo {
    mat4 transform;
    vec4 fill_color;
} glyph_info;


#ifdef FRAGMENT
layout (set = 1, binding = 1) uniform texture2D glyph_texture;
#endif