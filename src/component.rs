use crate::*;
use drawing::*;
use input_handling::*;
use std::cell::Cell;

pub trait Component: Layout + Draw + PreviewInputHandler + InputHandler + DispatchInput {}

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
