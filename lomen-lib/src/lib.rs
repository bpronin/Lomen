use lomen_core::color::{Color, ZoneColors};
use lomen_core::light_control;

#[derive(Debug)]
#[repr(C)]
pub struct ColorsArg {
    pub right: u64,
    pub center: u64,
    pub left: u64,
    pub game: u64,
}

const NO_COLOR: u64 = 0xFFFFFFFF;

#[unsafe(no_mangle)]
pub extern "stdcall" fn is_lighting_supported() -> bool {
    light_control::is_lighting_supported().unwrap()
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn get_keyboard_type() -> u8 {
    light_control::get_keyboard_type().unwrap()
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn get_colors(out_data: *mut ColorsArg) {
    if !out_data.is_null() {
        let colors = light_control::get_colors().unwrap();
        unsafe {
            (*out_data).right = color_to_num(colors.right);
            (*out_data).center = color_to_num(colors.center);
            (*out_data).left = color_to_num(colors.left);
            (*out_data).game = color_to_num(colors.game);
        }
    } else {
        panic!("Out data pointer is null.");
    }
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn set_colors(data: *const ColorsArg) {
    if !data.is_null() {
        let colors = unsafe {
            ZoneColors {
                right: num_to_color((*data).right),
                center: num_to_color((*data).center),
                left: num_to_color((*data).left),
                game: num_to_color((*data).game),
            }
        };
        light_control::set_colors(colors).unwrap()
    }else {
        panic!("Data pointer is null.");
    }
}

fn num_to_color(color: u64) -> Option<Color> {
    if color == NO_COLOR {
        None
    } else {
        Some(color.into())
    }
}

fn color_to_num(color: Option<Color>) -> u64 {
    match color {
        Some(color) => color.into(),
        None => NO_COLOR,
    }
}
