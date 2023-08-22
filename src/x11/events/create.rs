use super::event::EventType;
use crate::x11::window::Window;
use x11::xlib::{XCreateWindowEvent, XEvent};

#[derive(PartialEq)]
pub struct WindowCreateData {
    pub window: Window,
    pub parent_window: Window,
}

/// CreateNotify gets fired 4 times per window opened for some reason, we need to check
/// if the window name is `None` to remove the excess CreateNotify calls
pub fn xevent_to_window_create_data(xevent: XEvent) -> EventType {
    let xcreate: XCreateWindowEvent = xevent.into();

    let parent_window = Window {
        id: xcreate.parent,
        display: xcreate.display,
    };

    let window = Window {
        display: xcreate.display,
        id: xcreate.window,
    };

    if window.get_name().is_none() {
        return EventType::Unimplemented;
    }

    EventType::WindowCreated(WindowCreateData {
        window,
        parent_window,
    })
}
