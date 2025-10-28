use crate::kbd_light_control::{
    get_colors, get_keyboard_type, is_lighting_supported, set_colors, Color, ZoneColors,
};

mod kbd_light_control;

fn main() {
    println!("Keyboard type: {}", get_keyboard_type().unwrap());
    println!("Lighting supported: {}", is_lighting_supported().unwrap());
    println!("Lighting colors: {:?}", get_colors().unwrap());

    let colors = ZoneColors {
        right: Color {
            red: 255,
            green: 0,
            blue: 255,
        }
            .into(),
        center: None,
        left: None,
        game: None,
    };

    set_colors(colors).unwrap();
}
