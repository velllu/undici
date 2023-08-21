use x11::xlib::{XCreateWindowEvent, XEvent};

use crate::x11::window::Window;

#[derive(PartialEq)]
pub struct WindowCreateData {
    pub window: Window,
}

impl From<XEvent> for WindowCreateData {
    fn from(xevent: XEvent) -> Self {
        let xcreate: XCreateWindowEvent = xevent.into();

        let window = Window {
            display: xcreate.display,
            id: xcreate.window,
        };

        Self { window }
    }
}
