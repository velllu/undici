use std::ffi::{c_char, CStr};
use x11::xlib::{XEvent, XKeyEvent, XKeycodeToKeysym, XKeysymToString};

#[derive(PartialEq)]
pub struct KeyEventData {
    pub key: String,
}

impl From<XEvent> for KeyEventData {
    fn from(xevent: XEvent) -> Self {
        let xkey: XKeyEvent = xevent.into();

        let key_sym = unsafe { XKeycodeToKeysym(xkey.display, xkey.keycode as u8, 0) };

        // Converting C's char* to rust's String
        let key = unsafe {
            let key_c: *mut c_char = XKeysymToString(key_sym);
            let key_cstring = CStr::from_ptr(key_c);

            key_cstring
                .to_str()
                .expect("could not convert C keysm to a Rust string")
                .to_string()
        };

        Self { key }
    }
}
