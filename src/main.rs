use crate::kbd_light_control::{get_keyboard_type, is_lighting_supported};

mod kbd_light_control;

fn main() {
    println!("Keyboard type: {}", get_keyboard_type().unwrap());
    println!("Lighting supported: {}", is_lighting_supported().unwrap());
}
