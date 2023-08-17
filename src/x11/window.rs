use super::common::MouseButton;
use std::{
    ffi::{c_char, c_uint, CString},
    ptr::null_mut,
};
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
    /// let display = Display::new()
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

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { XDestroyWindow(self.display, self.id) };
    }
}
