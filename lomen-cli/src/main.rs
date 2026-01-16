use argh::FromArgs;
use libloading::os::windows;
use libloading::os::windows::Library;
use windows::LOAD_WITH_ALTERED_SEARCH_PATH;

#[derive(FromArgs)]
#[argh(description = "Tool to control keyboard lighting of HP OMEN laptops.
Use 6-digits RGB hex codes for colors (e.g. #FF00FF).
In PowerShell enquote them (e.g. \"#FF00FF\") or use codes without '#' symbol (e.g. FF00FF)
")]
struct Args {
    #[argh(
        switch,
        short = 'i',
        description = "display keyboard lighting status information"
    )]
    info: bool,

    #[argh(
        option,
        short = 'r',
        description = "set color for the first (right) zone of the keyboard"
    )]
    right: Option<String>,

    #[argh(
        option,
        short = 'c',
        description = "set color for the second (center) zone of the keyboard"
    )]
    center: Option<String>,

    #[argh(
        option,
        short = 'l',
        description = "set color for the third (left) zone of the keyboard"
    )]
    left: Option<String>,

    #[argh(
        option,
        short = 'g',
        description = "set color for the forth (game) zone of the keyboard"
    )]
    game: Option<String>,

    #[argh(
        option,
        short = 'a',
        description = "set color for all zones of the keyboard except those specified specifically"
    )]
    all: Option<String>,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ColorsData {
    pub right: u64,
    pub center: u64,
    pub left: u64,
    pub game: u64,
}

const NO_COLOR: u64 = 0xFFFFFFFF;

fn main() {
    let lib = unsafe { Library::load_with_flags("lomen.dll", LOAD_WITH_ALTERED_SEARCH_PATH) }
        .expect("Failed to load lomen.dll library");

    /* no args */
    if std::env::args().count() <= 1 {
        print_info(&lib);
        return;
    }

    let args: Args = argh::from_env();

    if args.right.is_some()
        || args.center.is_some()
        || args.left.is_some()
        || args.game.is_some()
        || args.all.is_some()
    {
        set_colors(
            &lib,
            ColorsData {
                right: str_to_color(&args.right, &args.all),
                center: str_to_color(&args.center, &args.all),
                left: str_to_color(&args.left, &args.all),
                game: str_to_color(&args.game, &args.all),
            },
        );
    }

    /* after all to show modified status */
    if args.info {
        print_info(&lib);
    }
}

fn print_info(lib: &Library) {
    println!("Keyboard type: {}", get_keyboard_type(lib));
    println!("Lighting supported: {}", is_lighting_supported(lib));

    let colors = get_colors(lib);
    println!("Zone colors:");
    println!("\t{:08}{:#08X}", "right", colors.right);
    println!("\t{:08}{:#08X}", "center", colors.center);
    println!("\t{:08}{:#08X}", "left", colors.left);
    println!("\t{:08}{:#08X}", "game", colors.game);
}

fn is_lighting_supported(lib: &Library) -> bool {
    unsafe {
        type Fn = extern "stdcall" fn() -> bool;
        let fun = lib.get::<Fn>(b"is_lighting_supported\0").unwrap();
        fun()
    }
}

fn get_keyboard_type(lib: &Library) -> u8 {
    unsafe {
        type Fn = extern "stdcall" fn() -> u8;
        let fun = lib.get::<Fn>(b"get_keyboard_type\0").unwrap();
        fun()
    }
}

fn get_colors(lib: &Library) -> ColorsData {
    unsafe {
        type Fn = extern "stdcall" fn(*mut ColorsData);
        let fun = lib.get::<Fn>(b"get_colors\0").unwrap();

        let mut colors = ColorsData::default();
        fun(&mut colors);
        colors
    }
}

fn set_colors(lib: &Library, colors: ColorsData) {
    unsafe {
        type Fn = extern "stdcall" fn(*const ColorsData);
        let fun = lib.get::<Fn>(b"set_colors\0").unwrap();

        fun(&colors);
    }
}

fn str_to_color(color: &Option<String>, default_color: &Option<String>) -> u64 {
    match color {
        Some(it) => parse_hex_color(it),
        None => match default_color {
            Some(it) => parse_hex_color(it),
            None => NO_COLOR,
        },
    }
}

fn parse_hex_color(s: &str) -> u64 {
    let x = s.strip_prefix('#').unwrap_or(s);
    u64::from_str_radix(x, 16).expect(format!("Invalid color code: {}", s).as_str())
}
