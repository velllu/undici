//! Rewrite of https://github.com/mackstann/tinywm

use undici::x11::{
    common::{MouseButton, Vector2},
    display::Display,
    events::{button::MouseEventData, event::EventType},
    window::{Modifier, WindowData},
};

fn main() {
    // This sets the wm to open at the 90th display, so you can put a virtual screen using
    // the Xephyr linux command. Beware that this breaks if you have 90 monitors 😀
    // Example of command: `Xephyr -br -ac -noreset -screen 800x800 :90`
    #[cfg(debug_assertions)] // this line makes it so it only does this in debug mode
    std::env::set_var("DISPLAY", ":90");

    let display = Display::new().expect("could not open display");
    let root_window = display.get_root_window();

    root_window.grab_key("l", Modifier::Alt); // Press Alt + L to put the window on the top
    root_window.grab_key("r", Modifier::Alt); // Press Alt + R to put the window on the bottom

    // Drag while pressing Alt + Left Mouse Button to drag window around
    root_window.grab_mouse_button(MouseButton::Left, Modifier::Alt);

    // Drag while pressing Alt + Right Mouse Button to resize window from the right bottom corner
    root_window.grab_mouse_button(MouseButton::Right, Modifier::Alt);

    // To hold the the mouse/window data once dragging starts
    let mut start: Option<MouseEventData> = None;
    let mut attributes: Option<WindowData> = None;

    loop {
        let event = display.get_event();

        // Later we will use `event.subwindow`, which is None if the user interacts outside of
        // a window, but is Some(Window) if the user is inside a window

        match event.type_ {
            EventType::KeyPress(key_event) => {
                if let Some(window) = event.subwindow {
                    // This will only happen if the key is pressed inside a Window, and
                    // not the root window in general
                    match key_event.key.as_str() {
                        "l" => window.lower(),
                        "r" => window.raise(),
                        _ => {}
                    }
                }
            }

            EventType::MouseButtonPress(mouse_event) => {
                if let Some(window) = event.subwindow {
                    attributes = Some(window.get_data());
                }

                start = Some(mouse_event)
            }

            EventType::MouseButtonRelease(_mouse_event) => {
                start = None;
            }

            EventType::MotionNotify(motion_event) => {
                if let (Some(start), Some(window), Some(attributes)) =
                    (&start, &event.subwindow, &attributes)
                {
                    let x_diff = motion_event.root_position.x - start.root_position.x;
                    let y_diff = motion_event.root_position.y - start.root_position.y;

                    match start.button {
                        MouseButton::Left => window.set_position(Vector2::new(
                            attributes.position.x + x_diff,
                            attributes.position.y + y_diff,
                        )),

                        MouseButton::Right => window.set_scale(Vector2::new(
                            (attributes.scale.x + x_diff) as u32,
                            (attributes.scale.y + y_diff) as u32,
                        )),

                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}
