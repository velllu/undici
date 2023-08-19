use std::ffi::{c_char, c_uint, CString};

use super::{button::MouseEventData, key::KeyEventData, motion::MotionData};
use crate::x11::{
    common::MouseButton,
    display::Display,
    window::{Modifier, Window},
};
use x11::xlib::*;

#[derive(PartialEq)]
pub enum EventType {
    KeyPress(KeyEventData),
    KeyRelease(KeyEventData),
    MouseButtonPress(MouseEventData),
    MouseButtonRelease(MouseEventData),
    MotionNotify(MotionData),
    Unimplemented,
}

pub(crate) struct Event {
    pub(crate) event: XEvent,
}

pub struct EventData {
    pub type_: EventType,
    pub subwindow: Option<Window>,
}

impl Event {
    pub(crate) fn new() -> Self {
        Self {
            event: unsafe { std::mem::zeroed() },
        }
    }

    #[allow(non_upper_case_globals)]
    pub(crate) fn get_data(&self) -> EventData {
        let subwindow = unsafe {
            if self.event.key.subwindow == 0 {
                None
            } else {
                Some(Window {
                    id: self.event.key.subwindow,
                    display: self.event.key.display,
                })
            }
        };

        let type_ = match unsafe { self.event.type_ } {
            KeyPress => EventType::KeyPress(self.event.into()),
            KeyRelease => EventType::KeyRelease(self.event.into()),
            ButtonPress => EventType::MouseButtonPress(self.event.into()),
            ButtonRelease => EventType::MouseButtonRelease(self.event.into()),
            _ => EventType::Unimplemented,
        };

        EventData { type_, subwindow }
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

fn modifier_to_xlib_mod(modifier: Modifier) -> c_uint {
    match modifier {
        Modifier::Shift => ShiftMask,
        Modifier::Lock => LockMask,
        Modifier::Control => ControlMask,
        Modifier::Alt => Mod1Mask,
        Modifier::Num => Mod2Mask,
        Modifier::Super => Mod4Mask,
        Modifier::ScrollLock => Mod5Mask,
    }
}

fn mouse_button_to_number(mouse_button: MouseButton) -> c_uint {
    match mouse_button {
        MouseButton::Left => 1,
        MouseButton::Middle => 2,
        MouseButton::Right => 3,
    }
}

impl Window {
    /// Filters X11 key events to a specific key & modifier
    /// # Examples
    /// ```
    /// use undici::x11::{display::Display, window::Modifier};
    ///
    /// let display = Display::new().expect("could not open display");
    /// let root_window = display.get_root_window();
    ///
    /// root_window.grab_key("a", Modifier::Alt);
    /// ```
    pub fn grab_key(&self, key: &str, modifier: Modifier) {
        // what c does to a mf
        let key_c = CString::new(key).unwrap();
        let key_c_p: *const c_char = key_c.as_ptr();

        unsafe {
            XGrabKey(
                self.display,
                XKeysymToKeycode(self.display, XStringToKeysym(key_c_p)) as i32,
                modifier_to_xlib_mod(modifier),
                self.id,
                true.into(),
                1,
                1,
            )
        };
    }

    /// Filters X11 mouse buttons events to a specific mouse button & modifier
    /// # Examples
    /// This only makes X11 look for mouse events with the left mouse key, while pressing
    /// alt
    /// ```
    /// use undici::x11::{display::Display, window::Modifier, common::MouseButton};
    ///
    /// let display = Display::new().expect("could not open display");
    /// let root_window = display.get_root_window();
    ///
    /// root_window.grab_mouse_button(MouseButton::Left, Modifier::Alt);
    /// ```
    pub fn grab_mouse_button(&self, mouse_button: MouseButton, modifier: Modifier) {
        unsafe {
            XGrabButton(
                self.display,
                mouse_button_to_number(mouse_button),
                modifier_to_xlib_mod(modifier),
                self.id,
                true.into(),
                (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
                1,
                1,
                0,
                0,
            )
        };
    }
}
