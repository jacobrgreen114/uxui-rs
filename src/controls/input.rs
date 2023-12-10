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

// use crate::component::*;
// use crate::drawing::*;
// use crate::*;
// use input_handling::*;

// pub struct InputBuilder {
//     hint: Option<String>,
//     text: Option<BindableString>,
// }
//
// impl InputBuilder {
//     pub fn with_hint(mut self, text: &str) -> Self {
//         self.hint.replace(text.into());
//         self
//     }
//
//     pub fn with_text(mut self, text: &str) -> Self {
//         self.text.replace(BindableString::Static(text.into()));
//         self
//     }
//
//     // pub fn with_binding(mut self, binding: StringPropertyBinding) -> Self {
//     //     self.text.replace(BindableString::Binding(binding));
//     //     self
//     // }
//
//     pub fn build(self) -> Box<Input> {
//         Box::new(Input {
//             hint: self.hint,
//             text: self.text.unwrap_or_default(),
//         })
//     }
// }
//
// pub struct Input {
//     hint: Option<String>,
//     text: BindableString,
// }
//
// impl Input {
//     pub fn builder() -> InputBuilder {
//         InputBuilder {
//             hint: None,
//             text: None,
//         }
//     }
// }
//
// impl InputHandler for Input {}
//
// impl PreviewInputHandler for Input {}
//
// impl ComponentController for Input {
//     fn measure(&mut self, available_size: Size, children: &[Component]) -> Size {
//         todo!()
//     }
//
//     fn arrange(&mut self, final_rect: Rect, children: &[Component]) -> Rect {
//         todo!()
//     }
//
//     fn draw(&self, context: &mut DrawingContext) {
//         todo!()
//     }
// }
