use crate::drawing::*;
use crate::*;
use std::cell::RefCell;

pub trait Component {
    fn is_layout_dirty(&self) -> bool;

    fn is_visually_dirty(&self) -> bool;

    fn measure(&mut self, available_size: Size) -> Size;
    fn arrange(&mut self, final_rect: Rect) -> Rect;

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);
}

pub trait ComponentController: Sized + 'static {
    fn measure(&mut self, component: &BaseComponent<Self>, available_size: Size) -> Size;
    fn arrange(&mut self, component: &BaseComponent<Self>, final_rect: Rect) -> Rect;
}

pub struct BaseComponent<C>
where
    C: ComponentController,
{
    controller: RefCell<C>,
    visually_dirty: bool,
    layout_dirty: bool,
    final_size: Option<Size>,
    final_rect: Option<Rect>,
}

impl<C> BaseComponent<C>
where
    C: ComponentController,
{
    pub fn new(controller: C) -> Self {
        Self {
            controller: RefCell::new(controller),
            visually_dirty: true,
            layout_dirty: true,
            final_size: None,
            final_rect: None,
        }
    }
}

impl<C> Component for BaseComponent<C>
where
    C: ComponentController,
{
    fn is_layout_dirty(&self) -> bool {
        todo!()
    }

    fn is_visually_dirty(&self) -> bool {
        todo!()
    }

    fn measure(&mut self, available_size: Size) -> Size {
        self.final_size
            .replace(self.controller.borrow_mut().measure(self, available_size));
        self.final_size.unwrap()
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        self.final_rect
            .replace(self.controller.borrow_mut().arrange(self, final_rect));
        self.visually_dirty = false;
        self.final_rect.unwrap()
    }

    fn draw(&self, context: &mut DrawingContext) {
        todo!()
    }
}

pub trait ComponentSizingExt {
    fn calc_available_size(&self, available_size: Size) -> Size;
    fn calc_final_size(&self, available_size: Size, required_size: Size) -> Size;
}

impl ComponentSizingExt for Sizing {
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

//#[inline]
//pub fn calculate_available_size(sizing: &Sizing, available_size: Size) -> Size {
//    Size {
//        width: match sizing.width.desired {
//            Length::Fit | Length::Fill => available_size
//                .width
//                .max(sizing.width.min)
//                .min(sizing.width.max),
//            Length::Fixed(pixels) => pixels,
//        },
//        height: match sizing.height.desired {
//            Length::Fit | Length::Fill => available_size
//                .height
//                .max(sizing.height.min)
//                .min(sizing.height.max),
//            Length::Fixed(pixels) => pixels,
//        },
//    }
//}
//
//#[inline]
//pub fn calculate_final_size(sizing: &Sizing, available_size: Size, required_size: Size) -> Size {
//    Size {
//        width: match sizing.width.desired {
//            Length::Fit => required_size
//                .width
//                .max(sizing.width.min)
//                .min(sizing.width.max),
//            Length::Fill => available_size
//                .width
//                .max(sizing.width.min)
//                .min(sizing.width.max),
//            Length::Fixed(pixels) => pixels,
//        },
//        height: match sizing.height.desired {
//            Length::Fit => required_size
//                .height
//                .max(sizing.height.min)
//                .min(sizing.height.max),
//            Length::Fill => available_size
//                .height
//                .max(sizing.height.min)
//                .min(sizing.height.max),
//            Length::Fixed(pixels) => pixels,
//        },
//    }
//}
