use super::{button::MouseEventData, key::KeyEventData};
use crate::x11::display::Display;
use x11::xlib::{ButtonPress, ButtonRelease, KeyPress, KeyRelease, XEvent, XNextEvent};

#[derive(PartialEq)]
pub enum EventType {
    KeyPress(KeyEventData),
    KeyRelease(KeyEventData),
    MouseButtonPress(MouseEventData),
    MouseButtonRelease(MouseEventData),
    Unimplemented,
}

pub(crate) struct Event {
    pub(crate) event: XEvent,
}

pub struct EventData {
    pub type_: EventType,
}

impl Event {
    pub(crate) fn new() -> Self {
        Self {
            event: unsafe { std::mem::zeroed() },
        }
    }

    #[allow(non_upper_case_globals)]
    pub(crate) fn get_data(&self) -> EventData {
        let type_ = match unsafe { self.event.type_ } {
            KeyPress => EventType::KeyPress(self.event.into()),
            KeyRelease => EventType::KeyRelease(self.event.into()),
            ButtonPress => EventType::MouseButtonPress(self.event.into()),
            ButtonRelease => EventType::MouseButtonRelease(self.event.into()),
            _ => EventType::Unimplemented,
        };

        EventData { type_ }
    }
}

impl Display {
    /// Listen to any X11 event, this is meant to be used in a `loop`
    /// # Examples
    /// ```no_run
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new().expect("could not open display");
    ///
    /// loop {
    ///     let event = display.get_event();
    ///
    ///     match event.type_ {
    ///         _ => todo!()
    ///     }
    /// }
    /// ```
    pub fn get_event(&self) -> EventData {
        let mut event = Event::new();
        unsafe { XNextEvent(self.display, &mut event.event as *mut XEvent) };

        event.get_data()
    }
}
