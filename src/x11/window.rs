use std::ptr::null_mut;
use x11::xlib::*;

pub enum Modifier {
    Shift,
    Lock,
    Control,
    Alt,
    Num,
    Super,
    ScrollLock,
}

pub struct WindowData {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// We need a default window attributes const because the XGetWindowAttributes requires a
/// "blank" XWindowAttributes variable to begin with
const NEW_WINDOW_ATTRIBUTES: XWindowAttributes = XWindowAttributes {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    border_width: 0,
    depth: 0,
    visual: null_mut(),
    root: 0,
    class: 0,
    bit_gravity: 0,
    win_gravity: 0,
    backing_store: 0,
    backing_planes: 0,
    backing_pixel: 0,
    save_under: 0,
    colormap: 0,
    map_installed: 0,
    map_state: 0,
    all_event_masks: 0,
    your_event_mask: 0,
    do_not_propagate_mask: 0,
    override_redirect: 0,
    screen: null_mut(),
};

/// Autoexplicative enough right?
pub fn kill_window(window: &mut Window) {
    unsafe { XDestroyWindow(window.display, window.id) };
}

/// # Warning
/// If this is not used in a loop (for example, a wm), you need to manually kill the
/// window with the `kill_window(&mut window)` function
pub struct Window {
    // To identify a `Window` we need but it's `id` and the `display` where the window
    // lives
    pub(crate) id: u64,
    pub(crate) display: *mut _XDisplay,
}

impl Window {
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new()tar
    ///     .expect("Hopefully there are no errors!");
    ///
    /// let data = display.get_root_window().get_data();
    ///
    /// println!("Your monitor resolution is {}x{}", data.width, data.height);
    /// assert_eq!(0, data.x); // The root window should always be fixed on the top right
    /// assert_eq!(0, data.y);
    /// ```
    pub fn get_data(&self) -> WindowData {
        let mut attributes = NEW_WINDOW_ATTRIBUTES;
        unsafe { XGetWindowAttributes(self.display, self.id, &mut attributes) };

        attributes.into()
    }

    /// Makes window go on top of all other windows. If you are searching for the opposite
    /// thing, see the `lower()` function
    pub fn raise(&self) {
        unsafe { XRaiseWindow(self.display, self.id) };
    }

    /// Makes window go on the bottom of all other windows. If you are searching for the
    /// opposite thing, see the `raise()` function
    pub fn lower(&self) {
        unsafe { XLowerWindow(self.display, self.id) };
    }
}

impl From<XWindowAttributes> for WindowData {
    fn from(attributes: XWindowAttributes) -> Self {
        WindowData {
            x: attributes.x,
            y: attributes.y,
            width: attributes.width,
            height: attributes.height,
        }
    }
}
