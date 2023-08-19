use undici::x11::{
    common::MouseButton, display::Display, events::event::EventType, window::Modifier,
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

    root_window.grab_mouse_button(MouseButton::Left, Modifier::Alt);
    root_window.grab_mouse_button(MouseButton::Right, Modifier::Alt);

    loop {
        let event = display.get_event();

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

            EventType::KeyRelease(key_event) => {
                println!("Released key: {}", key_event.key);
            }

            EventType::MouseButtonPress(mouse_event) => match mouse_event.button {
                MouseButton::Left => println!("Pressed left mouse btn"),
                MouseButton::Middle => println!("Pressed middle mouse btn"),
                MouseButton::Right => println!("Pressed right mouse btn"),
            },

            EventType::MouseButtonRelease(mouse_event) => match mouse_event.button {
                MouseButton::Left => println!("Released left mouse btn"),
                MouseButton::Middle => println!("Released middle mouse btn"),
                MouseButton::Right => println!("Released right mouse btn"),
            },

            EventType::Unimplemented => {}
        }
    }
}
