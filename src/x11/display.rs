use std::ptr::null;

use super::{errors::DisplayError, window::Window};
use x11::xlib::{XCloseDisplay, XDefaultRootWindow, XOpenDisplay, _XDisplay};

pub struct Display {
    pub(crate) display: *mut _XDisplay,
}

fn new_display() -> Result<Display, DisplayError> {
    let display = unsafe { XOpenDisplay(null()) };

    if display.is_null() {
        Err(DisplayError::CouldNotCreate)
    } else {
        Ok(Display { display })
    }
}

impl Display {
    /// Connects to the Display.
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new()
    ///     .expect("Hopefully there are no errors!");
    /// ```
    pub fn new() -> Result<Self, DisplayError> {
        new_display()
    }

    /// The "screen" in X11 is just a window that contains everything, it's called the
    /// root window, and every display has one, so it can't fail
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new()
    ///     .expect("Hopefully there are no errors!");
    ///
    /// let root_window = display.get_root_window();
    /// ```
    /// # Panics
    /// WARNING: This **cannot** be chained with `.new()`
    /// Because it needs the display to be alive!
    pub fn get_root_window(&self) -> Window {
        let root_window_id = unsafe { XDefaultRootWindow(self.display) };

        Window {
            id: root_window_id,
            display: self.display,
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.display) };
    }
}
