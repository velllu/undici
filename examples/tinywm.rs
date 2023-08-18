use undici::x11::{
    common::MouseButton, display::Display, events::event::EventType, window::Modifier,
};

fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("DISPLAY", ":90");

    let display = Display::new().expect("could not open display");

    let root_window = display.get_root_window();

    root_window.grab_key("Return", Modifier::Alt);
    root_window.grab_mouse_button(MouseButton::Left, Modifier::Alt);
    root_window.grab_mouse_button(MouseButton::Right, Modifier::Alt);

    loop {
        let event = display.get_event();

        match event.type_ {
            EventType::KeyPress(_key_event) => {
                if let Some(_window) = event.subwindow {
                    // This will only happen if the key is pressed inside a Window, and
                    // not the root window in general
                    println!("Return was pressed inside a window!");
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
