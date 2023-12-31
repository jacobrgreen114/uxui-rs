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

#include "image_bindings.glsl"

layout(location = 0) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

void main() {
    const vec4 fill = texture(sampler2D(image_texture, default_sampler), in_uv);
    out_color = fill;
}