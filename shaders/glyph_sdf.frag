/*
  Copyright 2023 Jacob Green

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

#version 450

#define FRAGMENT

#include "glyph_bindings.glsl"

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

const float SHARPNESS = 2.71;

void main() {
    const float dist = texture(sampler2D(glyph_texture, default_sampler), in_uv).r;
    const float anti_alias = fwidth(dist) / SHARPNESS;
    const float alpha = smoothstep(0.5f - anti_alias, 0.5f + anti_alias, dist);

    out_color = glyph_info.fill_color;
    out_color.a *= alpha;
}