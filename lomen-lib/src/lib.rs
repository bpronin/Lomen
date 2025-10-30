use crate::color::{Color, ZoneColors};

pub mod color;
pub mod light_control;

#[repr(C)]
#[derive(Debug)]
pub struct NumZoneColors {
    pub right: u64,
    pub center: u64,
    pub left: u64,
    pub game: u64,
}

const NO_COLOR: u64 = 0xFFFFFFFF;

#[unsafe(no_mangle)]
pub extern "C" fn is_lighting_supported() -> bool {
    light_control::is_lighting_supported().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_keyboard_type() -> u8 {
    light_control::get_keyboard_type().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_colors() -> NumZoneColors {
    let colors = light_control::get_colors().unwrap();
    NumZoneColors {
        right: colors.right.unwrap().into(),
        center: colors.center.unwrap().into(),
        left: colors.left.unwrap().into(),
        game: colors.game.unwrap().into(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_colors(colors: NumZoneColors) -> bool {
    light_control::set_colors(ZoneColors {
        right: color_from_num(colors.right),
        center: color_from_num(colors.center),
        left: color_from_num(colors.left),
        game: color_from_num(colors.game),
    })
    .is_ok()
}

fn color_from_num(value: u64) -> Option<Color> {
    if value == NO_COLOR {
        None
    } else {
        Some(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_colors() {
        let colors = NumZoneColors{
            right: NO_COLOR,
            center: NO_COLOR,
            left: 0x00FF00,
            game: 0xFF0000,
        };

        assert_eq!(set_colors(colors), true);
    }

}