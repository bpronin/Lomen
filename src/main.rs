use crate::light_control::{get_colors, get_keyboard_type, is_lighting_supported, set_colors};
use argh::FromArgs;
use color::{Color, ZoneColors};

mod color;
mod light_control;

/// A tool to control keyboard lighting of HP OMEN laptops.
/// Use 6-digits RGB hex codes for colors (e.g. #FF00FF).
/// In PowerShell use color codes without '#' symbol or enquote them (e.g. FF00FF or "#FF00FF")
#[derive(FromArgs)]
struct Args {
    /// display keyboard lighting status information
    #[argh(switch, short = 'i')]
    info: bool,

    /// set color for the first (right) zone of the keyboard
    #[argh(option, short = 'r')]
    right: Option<Color>,

    /// set color for the second (center) zone of the keyboard
    #[argh(option, short = 'c')]
    center: Option<Color>,

    /// set color for the third (left) zone of the keyboard
    #[argh(option, short = 'l')]
    left: Option<Color>,

    /// set color for the forth (game) zone of the keyboard
    #[argh(option, short = 'g')]
    game: Option<Color>,

    /// set color for all zones of the keyboard except those specified specifically
    #[argh(option, short = 'a')]
    all: Option<Color>,
}

fn main() {
    /* no args */
    if std::env::args().count() <= 1 {
        print_info();
        return;
    }

    let args: Args = argh::from_env();

    if args.info {
        print_info();
    }

    if args.right.is_some()
        || args.center.is_some()
        || args.left.is_some()
        || args.game.is_some()
        || args.all.is_some()
    {
        set_colors(ZoneColors {
            right: args.right.or(args.all),
            center: args.center.or(args.all),
            left: args.left.or(args.all),
            game: args.game.or(args.all),
        })
        .unwrap();
    }
}

fn print_info() {
    println!("Lighting supported: {}", is_lighting_supported().unwrap());
    println!("Keyboard type: {}", get_keyboard_type().unwrap());

    let colors = get_colors().unwrap();
    println!("Lighting colors:");
    println!("\t{:08}{}", "right", colors.right.unwrap());
    println!("\t{:08}{}", "center", colors.center.unwrap());
    println!("\t{:08}{}", "left", colors.left.unwrap());
    println!("\t{:08}{}", "game", colors.game.unwrap());
}
