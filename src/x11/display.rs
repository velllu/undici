use std::{env, process::Command, ptr::null, time::Duration};

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
    /// Connects to the Display, this is meant to be used when `--release` is active, for
    /// dev porpuses use the `new_virtual()` function
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

    /// Connects to a new Display on a virtual X11 session using the Xephyr command (which
    /// should be already installed on every linux distro), this is made to be used for
    /// fast debugging, when using `--release`, I suggest to use the standard `new()`
    /// function
    /// # Examples
    /// ```no_run
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new_virtual(800, 800)
    ///     .expect("Hopefully there are no errors!");
    /// ```
    pub fn new_virtual(width: u32, height: u32) -> Result<Self, DisplayError> {
        std::thread::spawn(move || {
            Command::new("Xephyr")
                .args([
                    "-br",
                    "-ac",
                    "-noreset",
                    "-screen",
                    &format!("{}x{}", width, height),
                    ":90",
                ])
                .spawn()
                .unwrap();
        });

        // This does not wait to look like I am doing something much more complex, we need
        // to wait because DISPLAY must be set after xephyr starts
        std::thread::sleep(Duration::from_millis(100));
        env::set_var("DISPLAY", ":90");

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
    /// WARNING: This **cannot** be chained with `.new()` and `.new_virtual()` methods!
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
        unsafe {
            XCloseDisplay(self.display);
        }
    }
}
