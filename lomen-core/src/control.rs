use crate::color::{Color, LightingColors};
use crate::transition::LightingColorsTransition;
use error::Error;
use log::debug;
use std::error;
use std::thread::sleep;
use std::time::Duration;
use wmi::{IWbemClassWrapper, Variant, WMIConnection};

static SIGN: [u8; 4] = [83, 69, 67, 85];

/* Command constants */
const CMD_COMMON: u32 = 131081;
const CMD_GAMING: u32 = 131080;

/* Command type constants */
const CMD_TYPE_GET_PLATFORM_INFO: u32 = 1;
const CMD_TYPE_GET_ZONE_COLORS: u32 = 2;
const CMD_TYPE_SET_ZONE_COLORS: u32 = 3;
// const CMD_TYPE_STATUS: u32 = 4;
// const CMD_TYPE_SET_BRIGHTNESS: u32 = 5;
// const CMD_TYPE_SET_LIGHT_BAR_COLORS: u32 = 11;
const CMD_TYPE_GET_KEYBOARD_TYPE: u32 = 43;

// /* Lighting levels */
// const LIGHTING_LEVEL_ON: u8 = 228;
// const LIGHTING_LEVEL_OFF: u8 = 100;

/* Zone indices */
const RIGHT_ZONE_INDEX: usize = 0;
const CENTER_ZONE_INDEX: usize = 1;
const LEFT_ZONE_INDEX: usize = 2;
const GAME_ZONE_INDEX: usize = 3;

fn bytes_to_variant(bytes: &[u8]) -> Variant {
    Variant::Array(bytes.iter().copied().map(Variant::UI1).collect())
}

fn variant_to_bytes(v: Variant) -> Result<Vec<u8>, Box<dyn Error>> {
    match v {
        Variant::Array(vec) => {
            let mut out = Vec::with_capacity(vec.len());
            for (i, item) in vec.into_iter().enumerate() {
                match item {
                    Variant::UI1(b) => out.push(b),
                    other => {
                        return Err(
                            format!("Element {} has unsupported type: {:?}", i, other).into()
                        );
                    }
                }
            }
            Ok(out)
        }
        other => Err(format!("Variant::Array expected, but {:?} found", other).into()),
    }
}

fn rgb_offsets(zone_index: usize) -> (usize, usize, usize) {
    let offset = 25 + zone_index * 3;
    (offset, offset + 1, offset + 2)
}

fn get_zone_color(data: &[u8], zone_index: usize) -> Color {
    let (ri, gi, bi) = rgb_offsets(zone_index);
    Color::new(data[ri], data[gi], data[bi])
}

fn set_zone_color(data: &mut [u8], zone_index: usize, color: Option<Color>) {
    if let Some(c) = color {
        let (ri, gi, bi) = rgb_offsets(zone_index);
        data[ri] = c.r;
        data[gi] = c.g;
        data[bi] = c.b;
    }
}

fn execute_wmi_command(
    command_code: u32,
    command_type: u32,
    data: Option<&[u8]>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    debug!("Executing command: {:?}, type: {:?}", command_code, command_type);

    let wmi_con = WMIConnection::with_namespace_path(r"root\wmi")?;

    let (payload, payload_size) = match data {
        Some(d) => {
            let i = d.len() as u32;
            (bytes_to_variant(d.into()), i)
        }
        None => (Variant::Null, 0u32),
    };

    let in_data = wmi_con.get_object("hpqBDataIn")?;
    in_data.put_property("Sign", bytes_to_variant(&SIGN))?;
    in_data.put_property("Command", Variant::UI4(command_code))?;
    in_data.put_property("CommandType", Variant::UI4(command_type))?;
    in_data.put_property("Size", Variant::UI4(payload_size))?;
    in_data.put_property("hpqBData", payload)?;

    let in_params = wmi_con
        .get_object("hpqBIntM")?
        .get_method("hpqBIOSInt128")?
        .unwrap()
        .spawn_instance()?;
    in_params.put_property("InData", in_data)?;

    let out_params = wmi_con
        .exec_method(
            r"hpqBIntM.InstanceName='ACPI\PNP0C14\0_0'",
            "hpqBIOSInt128",
            Some(&in_params),
        )?
        .unwrap();

    let out_data: IWbemClassWrapper = out_params.get_property("OutData")?.try_into()?;

    let return_code: u32 = out_data.get_property("rwReturnCode")?.try_into()?;
    if return_code != 0 {
        return Err(format!("Invalid return code: {}", return_code).into());
    }

    Ok(variant_to_bytes(out_data.get_property("Data")?)?)
}

/// Returns keyboard type
pub fn get_keyboard_type() -> Result<u8, Box<dyn Error>> {
    let data = execute_wmi_command(CMD_GAMING, CMD_TYPE_GET_KEYBOARD_TYPE, None)?;
    Ok(data[0])
}

/// Checks whether keyboard lighting is supported
pub fn is_lighting_supported() -> Result<bool, Box<dyn Error>> {
    let data = execute_wmi_command(CMD_COMMON, CMD_TYPE_GET_PLATFORM_INFO, None)?;
    Ok((data[0] & 1) == 1)
}

/// Returns current keyboard lighting colors
pub fn get_colors() -> Result<LightingColors, Box<dyn Error>> {
    let result = execute_wmi_command(CMD_COMMON, CMD_TYPE_GET_ZONE_COLORS, None)?;
    let data = result.as_ref();

    Ok(LightingColors {
        right: get_zone_color(data, RIGHT_ZONE_INDEX).into(),
        center: get_zone_color(data, CENTER_ZONE_INDEX).into(),
        left: get_zone_color(data, LEFT_ZONE_INDEX).into(),
        game: get_zone_color(data, GAME_ZONE_INDEX).into(),
    })
}

/// Sets keyboard lighting colors
pub fn set_colors(colors: &LightingColors) -> Result<(), Box<dyn Error>> {
    let mut result = execute_wmi_command(CMD_COMMON, CMD_TYPE_GET_ZONE_COLORS, None)?;
    let data = result.as_mut();

    set_zone_color(data, RIGHT_ZONE_INDEX, colors.right);
    set_zone_color(data, CENTER_ZONE_INDEX, colors.center);
    set_zone_color(data, LEFT_ZONE_INDEX, colors.left);
    set_zone_color(data, GAME_ZONE_INDEX, colors.game);

    execute_wmi_command(CMD_COMMON, CMD_TYPE_SET_ZONE_COLORS, Some(data))?;

    Ok(())
}

/// Smoothly changes keyboard lighting colors
pub fn transit_colors(
    to_colors: &LightingColors,
    duration: Duration,
    fps: u8,
) -> Result<(), Box<dyn Error>> {
    let transition = LightingColorsTransition::new(get_colors()?, to_colors.clone(), duration);
    let delay = duration.div_f32(fps as f32);

    for colors in transition {
        set_colors(&colors)?;
        sleep(delay);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_lighting_supported() {
        let result = is_lighting_supported();
        assert!(result.is_ok());

        println!("Lighting supported: {}", result.unwrap());
    }

    #[test]
    fn test_get_keyboard_type() {
        let result = get_keyboard_type();
        assert!(result.is_ok());

        println!("Keyboard type: {}", result.unwrap());
    }

    #[test]
    fn test_get_colors() {
        let result = get_colors();
        assert!(result.is_ok());

        println!("Colors: {:?}", result);
    }

    #[test]
    fn test_set_colors() {
        let colors = LightingColors {
            right: Some(Color::from(0xFFFF00)),
            center: None,
            left: Some(Color::from(0x00FF00)),
            game: None,
        };
        let result = set_colors(&colors);

        assert!(result.is_ok());
    }

    #[test]
    fn test_transit_colors() {
        let colors = LightingColors {
            right: Some(Color::from(0x0000FF)),
            center: Some(Color::from(0x00FFFF)),
            left: Some(Color::from(0xFFFF00)),
            game: None,
        };

        let result = transit_colors(&colors, Duration::from_secs(1), 50);

        assert!(result.is_ok());
    }
}
