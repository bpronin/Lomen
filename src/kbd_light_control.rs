use error::Error;
use std::error;
use wmi::{COMLibrary, IWbemClassWrapper, Variant, WMIConnection};

// fn print_object(object: &IWbemClassWrapper) -> Result<(), Box<dyn Error>> {
//     let in_data_props = object.list_properties()?;
//     println!("{:?}", object);
//     for prop_name in in_data_props {
//         let prop = object.get_property(&prop_name)?;
//         println!("{prop_name}: {:?}", prop);
//     }
//
//     Ok(())
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum KbdType {
//     Normal,
//     WithNumpad,
//     WithoutNumpad,
//     Rgb,
//     OneZoneWithNumpad,
//     OneZoneWithoutNumpad,
// }

static SIGN: [u8; 4] = [83, 69, 67, 85];

/* Command constants */
const CMD_COMMON: u32 = 131081;
const CMD_GAMING: u32 = 131080;

/* Command type constants */
const CMD_TYPE_GET_PLATFORM_INFO: u32 = 1;
const CMD_TYPE_GET_ZONE_COLORS: u32 = 2;
const CMD_TYPE_SET_ZONE_COLORS: u32 = 3;
const CMD_TYPE_STATUS: u32 = 4;
const CMD_TYPE_SET_BRIGHTNESS: u32 = 5;
const CMD_TYPE_SET_LIGHT_BAR_COLORS: u32 = 11;
const CMD_TYPE_GET_KEYBOARD_TYPE: u32 = 43;

/* Lighting levels */
pub const LIGHTING_LEVEL_ON: u8 = 228;
pub const LIGHTING_LEVEL_OFF: u8 = 100;

/* Zone indices */
pub const RIGHT_ZONE_INDEX: i32 = 0;
pub const CENTER_ZONE_INDEX: i32 = 1;
pub const LEFT_ZONE_INDEX: i32 = 2;
pub const GAME_ZONE_INDEX: i32 = 3;

/* Offset for color data in command buffer */
pub const COLORS_DATA_OFFSET: i32 = 25;

fn vec_to_variant(data: Vec<u8>) -> Variant {
    Variant::Array(data.into_iter().map(Variant::UI1).collect())
}

pub fn variant_to_vec(v: Variant) -> Result<Vec<u8>, Box<dyn Error>> {
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

fn execute_command(
    command_code: u32,
    command_type: u32,
    data: Option<Vec<u8>>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let wmi_con = WMIConnection::with_namespace_path(r"root\wmi", COMLibrary::new()?)?;

    let in_data = wmi_con.get_object("hpqBDataIn")?;
    in_data.put_property("Sign", vec_to_variant(SIGN.to_vec()))?;
    in_data.put_property("Command", Variant::UI4(command_code))?;
    in_data.put_property("CommandType", Variant::UI4(command_type))?;
    in_data.put_property("Size", Variant::UI4(0))?;
    in_data.put_property("hpqBData", Variant::Null)?;

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
    // let returnCode = out_data.get_property("rwReturnCode")?;

    Ok(variant_to_vec(out_data.get_property("Data")?)?)
}

pub fn get_keyboard_type() -> Result<u8, Box<dyn Error>> {
    let result = execute_command(CMD_GAMING, CMD_TYPE_GET_KEYBOARD_TYPE, None)?;
    Ok(result[0])
}

/// Checks whether keyboard lighting is supported
pub fn is_lighting_supported() -> Result<bool, Box<dyn Error>> {
    let result = execute_command(CMD_COMMON, CMD_TYPE_GET_PLATFORM_INFO, None)?;
    Ok((result[0] & 1) == 1)
}
