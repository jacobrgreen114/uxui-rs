use component::ComponentSizingExt;
use drawing::DrawingContext;
use std::any::Any;
use std::ops::Deref;
use Sizing;
use {Rect, Size};

pub struct KeyEventArgs {
    key: u32,
    state: u32,
}

pub trait Component: Layout + Draw + InputHandler {}

pub trait ComponentController: Layout + Draw + InputHandler {}

pub trait Draw {
    // fn visually_dirty(&self) -> bool { false }

    fn draw<'a>(&'a self, context: &'a ::drawing::DrawingContext<'a>);
}

pub trait Container {
    // fn children(&self) -> &[Box<dyn Component>];
    // fn children_mut(&mut self) -> &mut [Box<dyn Component>];
}

pub trait Layout {
    // fn layout_dirty(&self) -> bool { false }

    fn measure(&self, available_size: Size) -> Size;
    fn arrange(&self, final_rect: Rect) -> Rect;
}

pub trait PreviewInputHandler {
    fn on_key_event_preview(&self, _args: &KeyEventArgs) -> bool;
}

pub trait InputHandler: PreviewInputHandler {
    fn on_key_event(&self, _args: &KeyEventArgs) -> bool;
}

/*
   BaseButton
*/

pub struct BaseButtonCreateInfo {
    sizing: Sizing,
}

pub struct BaseButton {
    sizing: Sizing,
}

impl BaseButton {
    pub const fn new(create_info: &BaseButtonCreateInfo) -> Self {
        Self {
            sizing: create_info.sizing,
        }
    }
}

impl Layout for BaseButton {
    fn measure(&self, available_size: Size) -> Size {
        todo!()
    }

    fn arrange(&self, final_rect: Rect) -> Rect {
        todo!()
    }
}

impl Draw for BaseButton {
    fn draw<'a>(&'a self, context: &'a DrawingContext<'a>) {
        todo!()
    }
}

impl InputHandler for BaseButton {
    fn on_key_event(&self, _args: &KeyEventArgs) -> bool {
        false
    }
}

impl PreviewInputHandler for BaseButton {
    fn on_key_event_preview(&self, _args: &KeyEventArgs) -> bool {
        false
    }
}

impl Component for BaseButton {}

/*
   Button
*/
pub struct Button {
    base: BaseButton,
}

impl Layout for Button {
    fn measure(&self, available_size: Size) -> Size {
        self.base.measure(available_size)
    }

    fn arrange(&self, final_rect: Rect) -> Rect {
        self.base.arrange(final_rect)
    }
}

impl Draw for Button {
    fn draw<'a>(&'a self, context: &'a ::drawing::DrawingContext<'a>) {
        self.base.draw(context);
    }
}

impl InputHandler for Button {
    fn on_key_event(&self, _args: &KeyEventArgs) -> bool {
        self.base.on_key_event_preview(_args)
    }
}

impl PreviewInputHandler for Button {
    fn on_key_event_preview(&self, _args: &KeyEventArgs) -> bool {
        self.base.on_key_event_preview(_args)
    }
}

impl Component for Button {}

/*
   Input Dipatch Extension
*/

// trait InputDispatcher {
//     fn dispatch_key_event(&self, args: &KeyEventArgs) -> bool;
// }
//
// impl<T: Component + ?Sized> InputDispatcher for T {
//     fn dispatch_key_event(&self, args: &KeyEventArgs) -> bool {
//         // todo: validate shortcircuit of or on all platforms
//         self.on_key_event_preview(args)
//             || self
//                 .children()
//                 .iter()
//                 .any(|child| child.dispatch_key_event(args))
//             || self.on_key_event(args)
//     }
// }

/*

*/

// pub struct BaseComponent {}
//
// pub struct SizedComponent {
//     base: BaseComponent,
//     sizing: Sizing,
// }
//
// impl SizedComponent {
//     #[inline(always)]
//     pub fn measure_with_sizing<F>(&self, available_size: Size, f: F) -> Size
//     where
//         F: FnOnce(Size) -> Size,
//     {
//         let available = self.sizing.calc_available_size(available_size);
//         let required = f(available);
//         let final_size = self.sizing.calc_final_size(available, required);
//         final_size
//     }
// }
//
// #[inline(always)]
// fn draw_with_children<'a, It, F>(context: &'a ::drawing::DrawingContext<'a>, children: It, func: F)
// where
//     It: Iterator<Item = &'a dyn Component>,
//     F: FnOnce(&'a ::drawing::DrawingContext<'a>),
// {
//     func(context);
//     for child in children {
//         child.draw(context);
//     }
// }
