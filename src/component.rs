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

use crate::*;
use drawing::*;
use input_handling::*;

use std::cell::Cell;
use std::fmt::Debug;

pub trait Component:
    Layout + Draw + PreviewInputHandler + InputHandler + DispatchInput + Debug
{
}

// pub trait ComponentController: Layout + Draw + InputHandler {}

pub trait Draw {
    // fn visually_dirty(&self) -> bool { false }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);
}

pub trait Layout {
    // fn layout_dirty(&self) -> bool { false }

    fn measure(&mut self, available_size: Size) -> Size;
    fn arrange(&mut self, final_rect: Rect) -> Rect;
}

/*
   Base Component
*/

pub struct BaseComponent {
    sizing: Sizing,
    final_size: Cell<Size>,
    final_rect: Cell<Rect>,
}

/*
   Sizing Extensions
*/

pub(crate) trait ComponentSizingExt {
    fn calc_available_size(&self, available_size: Size) -> Size;
    fn calc_final_size(&self, available_size: Size, required_size: Size) -> Size;
}

// todo : adjust fit / fill logic
impl ComponentSizingExt for Sizing {
    #[inline]
    fn calc_available_size(&self, available_size: Size) -> Size {
        Size {
            width: match self.width.desired {
                Length::Fit | Length::Fill => {
                    available_size.width.max(self.width.min).min(self.width.max)
                }
                Length::Fixed(pixels) => pixels,
            },
            height: match self.height.desired {
                Length::Fit | Length::Fill => available_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fixed(pixels) => pixels,
            },
        }
    }

    #[inline]
    fn calc_final_size(&self, available_size: Size, required_size: Size) -> Size {
        Size {
            width: match self.width.desired {
                Length::Fit => required_size.width.max(self.width.min).min(self.width.max),
                Length::Fill => available_size.width.max(self.width.min).min(self.width.max),
                Length::Fixed(pixels) => pixels,
            },
            height: match self.height.desired {
                Length::Fit => required_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fill => available_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fixed(pixels) => pixels,
            },
        }
    }
}
