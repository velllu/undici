use super::window::Window;
use crate::color::RGB;
use x11::xlib::{XAllPlanes, XDestroyImage, XGetImage, XGetPixel, XImage, ZPixmap};

/// Can be created from a `Window`'s `.get_image()` method
pub struct Image {
    pub(crate) image: *mut XImage,
    pub width: u32,
    pub height: u32,
}

impl Image {
    /// Get a specifc pixel's RGB data
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new().unwrap();
    /// let root_window = display.get_root_window();
    ///
    /// let screenshot = root_window.get_image();
    /// let first_pixel = screenshot.get_pixel(0, 0);
    ///
    /// println!(
    ///     "First pixel color: #{:x}{:x}{:x}",
    ///     first_pixel.r,
    ///     first_pixel.b,
    ///     first_pixel.g
    /// );
    /// ```
    pub fn get_pixel(&self, x: i32, y: i32) -> RGB {
        let pixel = unsafe { XGetPixel(self.image, x, y) };

        RGB {
            r: ((pixel >> 16) & 0xFF) as u8,
            g: ((pixel >> 8) & 0xFF) as u8,
            b: (pixel & 0xFF) as u8,
        }
    }
}

impl Window {
    /// Takes a "picture" of the window, examples uses include:
    /// - Getting a specific pixel color
    /// ... and for now, that's it. I just used it for my `qmk-ambience` project
    /// # Examples
    /// ```
    /// use undici::x11::display::Display;
    ///
    /// let display = Display::new().expect("could not open display");
    /// let root_window = display.get_root_window();
    ///
    /// let screenshot = root_window.get_image();
    /// ```
    pub fn get_image(&self) -> Image {
        let window_data = self.get_data();
        let image = unsafe {
            XGetImage(
                self.display,
                self.id,
                0,
                0,
                window_data.width as u32,
                window_data.height as u32,
                XAllPlanes(),
                ZPixmap,
            )
        };

        Image {
            image,
            width: window_data.width as u32,
            height: window_data.height as u32,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { XDestroyImage(self.image) };
    }
}
