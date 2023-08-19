use crate::x11::common::Vector2;
use x11::xlib::{XEvent, XMotionEvent};

#[derive(PartialEq)]
pub struct MotionData {
    pub root_position: Vector2<i32>,
}

impl From<XEvent> for MotionData {
    fn from(xevent: XEvent) -> Self {
        let xmotion: XMotionEvent = xevent.into();

        Self {
            root_position: Vector2::new(xmotion.x_root, xmotion.y_root),
        }
    }
}
