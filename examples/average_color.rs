//! Prints the average color of the screen

use std::time::Instant;
use undici::x11::{display::Display, window::kill_window};

fn main() {
    let starting_time = Instant::now();

    let display = Display::new().expect("could not open display");
    let mut root_window = display.get_root_window();

    let screenshot = root_window.get_image();

    let starting_calculation_time = Instant::now();

    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);

    for x in 0..screenshot.width {
        for y in 0..screenshot.height {
            let pixel = screenshot.get_pixel(x as i32, y as i32);

            r += pixel.r as u32;
            g += pixel.g as u32;
            b += pixel.b as u32;
        }
    }

    let total_pixels = screenshot.width * screenshot.height;
    let r = (r as u32 / total_pixels) as u8;
    let g = (g as u32 / total_pixels) as u8;
    let b = (b as u32 / total_pixels) as u8;

    kill_window(&mut root_window);

    println!("Average screen color: #{:x}{:x}{:x}", r, g, b);
    println!("");

    #[cfg(debug_assertions)]
    println!("You should run this with --release for it to be faster!!");

    println!(
        "Elapsed time since program started: {:?}",
        Instant::now() - starting_time
    );

    println!(
        "Elapsed time since calculation started (after display opened etc): {:?}",
        Instant::now() - starting_calculation_time
    );
}
