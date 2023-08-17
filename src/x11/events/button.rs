use crate::x11::common::MouseButton;
use x11::xlib::{XButtonEvent, XEvent};

#[derive(PartialEq)]
pub struct MouseEventData {
    pub button: MouseButton,
}

impl From<XEvent> for MouseEventData {
    fn from(xevent: XEvent) -> Self {
        let xbutton: XButtonEvent = xevent.into();

        let button = match xbutton.button {
            1 => MouseButton::Left,
            2 => MouseButton::Middle,
            3 => MouseButton::Right,
            _ => unimplemented!(),
        };

        Self { button }
    }
}
