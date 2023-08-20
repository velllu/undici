use super::{common::Vector2, window::Window};
use x11::xlib::*;

impl Window {
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    /// use undici::x11::common::Vector2;
    ///
    /// let display = Display::new().unwrap();
    /// let root_window = display.get_root_window();
    ///
    /// root_window.set_cursor_position(Vector2::new(0, 0)); // Sets cursor on top left
    ///
    /// let position = root_window.get_cursor_position();
    /// assert_eq!(position.x, 0);
    /// assert_eq!(position.y, 0);
    /// ```
    pub fn set_cursor_position(&self, position: Vector2<i32>) {
        unsafe {
            XWarpPointer(self.display, 0, self.id, 0, 0, 0, 0, position.x, position.y);
        };
    }

    /// Check the `set_cursor_position` to have an example
    pub fn get_cursor_position(&self) -> Vector2<i32> {
        // We don't need all of this but it will segfault if we use a null pointer
        let mut x = 0;
        let mut y = 0;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut mask = 0;
        let mut root = 0u64;
        let mut child = 0u64;

        unsafe {
            XQueryPointer(
                self.display,
                self.id,
                &mut root,
                &mut child,
                &mut root_x,
                &mut root_y,
                &mut x,
                &mut y,
                &mut mask,
            );
        }

        Vector2 { x, y }
    }
}
