use crate::x11::common::Vector2;
use x11::xlib::{XEvent, XMotionEvent};

#[derive(PartialEq)]
pub struct MotionData {
    pub root_position: Vector2,
}

impl From<XEvent> for MotionData {
    fn from(xevent: XEvent) -> Self {
        let xbutton: XMotionEvent = xevent.into();

        Self {
            root_position: Vector2 {
                x: xbutton.x_root,
                y: xbutton.y_root,
            },
        }

        // let button = match xbutton.button {
        //     1 => MouseButton::Left,
        //     2 => MouseButton::Middle,
        //     3 => MouseButton::Right,
        //     _ => unimplemented!(),
        // };

        // Self {
        //     button,
        //     root_position: Vector2 {
        //         x: xbutton.x_root,
        //         y: xbutton.y_root,
        //     },
        // }
    }
}
