use crate::kbd_light_control::{get_colors, get_keyboard_type, is_lighting_supported, set_colors, Color, ZoneColors};
use argh::FromArgs;
use std::str::FromStr;

mod kbd_light_control;

/// A tool to control keyboard lighting of HP OMEN laptops
#[derive(FromArgs)]
struct Args {
    /// displays keyboard lighting status information
    #[argh(switch, short = 'i')]
    info: bool,

    /// sets color for the first (right) zone of the keyboard in hex format
    #[argh(option, short = '1')]
    right: Option<String>,

    /// sets color for the second (center) zone of the keyboard in hex format
    #[argh(option, short = '2')]
    center: Option<String>,

    /// sets color for the third (left) zone of the keyboard in hex format
    #[argh(option, short = '3')]
    left: Option<String>,

    /// sets color for the forth (game) zone of the keyboard in hex format
    #[argh(option, short = '4')]
    game: Option<String>,

    /// sets color for all zones of the keyboard except those specified specifically in hex format
    #[argh(option, short = 'a')]
    all: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

    if args.info {
        println!("Keyboard type: {}", get_keyboard_type().unwrap());
        println!("Lighting supported: {}", is_lighting_supported().unwrap());

        let colors = get_colors().unwrap();
        println!("Lighting colors:");
        println!("\tright: {:?}", colors.right.unwrap());
        println!("\tcenter: {:?}", colors.center.unwrap());
        println!("\tleft: {:?}", colors.left.unwrap());
        println!("\tgame: {:?}", colors.game.unwrap());
    }

    let default_color = args.all.map(|s|
        Color::from_str(s.as_str()).unwrap_or_default()
    );

    let colors = ZoneColors {
        right: args.right.map_or(default_color, |s| {
            Color::from_str(s.as_str()).ok()
        }),
        center: args.center.map_or(default_color, |s| {
            Color::from_str(s.as_str()).ok()
        }),
        left: args.left.map_or(default_color, |s| {
            Color::from_str(s.as_str()).ok()
        }),
        game: args.game.map_or(default_color, |s| {
            Color::from_str(s.as_str()).ok()
        }),
    };
    set_colors(colors).unwrap();
}
