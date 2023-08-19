use undici::x11::{
    common::{MouseButton, Vector2},
    display::Display,
    events::event::EventType,
    window::Modifier,
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

    let mut starting_mouse_position: Option<Vector2> = None;

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

            EventType::MouseButtonPress(mouse_event) => {
                if let Some(_window) = event.subwindow {
                    starting_mouse_position = Some(mouse_event.root_position)
                }
            }

            EventType::MotionNotify(motion_event) => {
                if let Some(starting_mouse_position) = &starting_mouse_position {
                    let x_diff = motion_event.root_position.x - starting_mouse_position.x;
                    let y_diff = motion_event.root_position.y - starting_mouse_position.y;
                }
            }

            _ => {}
        }
    }
}
