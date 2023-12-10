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