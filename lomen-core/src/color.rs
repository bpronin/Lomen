use fmt::Display;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Into<u64> for Color {
    fn into(self) -> u64 {
        let mut c: u64 = 0;
        c = (c << 8) | self.r as u64;
        c = (c << 8) | self.g as u64;
        c = (c << 8) | self.b as u64;
        c
    }
}

impl From<u64> for Color {
    fn from(value: u64) -> Self {
        Self::new((value >> 16) as u8, (value >> 8) as u8, value as u8)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b))
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: u64 = u64::from_str_radix(s.trim_start_matches("#"), 16)
            .map_err(|e| format!("Error parsing color: {}", e))?;

        Ok(Self::from(v))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ZoneColors {
    pub right: Option<Color>,
    pub center: Option<Color>,
    pub left: Option<Color>,
    pub game: Option<Color>,
}

impl ZoneColors {
    fn parse_color(s: &str) -> Option<Color> {
        Color::from_str(s).ok()
    }

    fn format_color(c: Option<Color>) -> String {
        c.map(|c| c.to_string()).unwrap_or_else(|| "_".to_string())
    }
}

impl Display for ZoneColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}, {}, {}, {}",
            Self::format_color(self.right),
            Self::format_color(self.center),
            Self::format_color(self.left),
            Self::format_color(self.game)
        ))
    }
}

impl Into<Vec<String>> for ZoneColors {
    fn into(self) -> Vec<String> {
        vec![
            Self::format_color(self.right),
            Self::format_color(self.center),
            Self::format_color(self.left),
            Self::format_color(self.game),
        ]
    }
}

impl From<Vec<&str>> for ZoneColors {
    fn from(value: Vec<&str>) -> Self {
        Self {
            right: Self::parse_color(value[0]),
            center: Self::parse_color(value[1]),
            left: Self::parse_color(value[2]),
            game: Self::parse_color(value[3]),
        }
    }
}

// impl Into<[u64; 4]> for ZoneColors {
//     fn into(self) -> [u64; 4] {
//         let parse = |c: Option<Color>| c.map(|c| c.into()).unwrap_or_default();
//         [
//             parse(self.right),
//             parse(self.center),
//             parse(self.left),
//             parse(self.game),
//         ]
//     }
// }
//
// impl From<[u64; 4]> for ZoneColors {
//     fn from(value: [u64; 4]) -> Self {
//         Self {
//             right: Some(Color::from(value[0])),
//             center: Some(Color::from(value[1])),
//             left: Some(Color::from(value[2])),
//             game: Some(Color::from(value[3])),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        assert_eq!(Color::new(0xAA, 0xBB, 0xCC), Color::from(0xAABBCC));
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!(Ok(Color::new(0xAA, 0xBB, 0xCC)), Color::from_str("#AABBCC"));
    }

    #[test]
    fn test_color_from_u64() {
        let c: Color = Color::from(0x00AABBCC);
        assert_eq!(c, Color::new(0xAA, 0xBB, 0xCC));
    }

    #[test]
    fn test_color_into_u64() {
        let c: u64 = Color::new(0xAA, 0xBB, 0xCC).into();
        assert_eq!(c, 0x00AABBCC);
    }

    #[test]
    fn test_color_display() {
        assert_eq!(format!("{}", Color::new(0xAA, 0xBB, 0xCC)), "#AABBCC");
    }

    #[test]
    fn test_colors_display() {
        let z = ZoneColors {
            right: Some(Color::from(0xAA0000)),
            center: Some(Color::from(0xBB0000)),
            left: None,
            game: Some(Color::from(0xDD0000)),
        };
        assert_eq!(format!("{}", z), "#AA0000, #BB0000, _, #DD0000");
    }

    #[test]
    fn test_colors_into_vec() {
        let actual: Vec<String> = ZoneColors {
            right: Some(Color::from(0xAA0000)),
            center: Some(Color::from(0xBB0000)),
            left: Some(Color::from(0xCC0000)),
            game: None,
        }
        .into();

        assert_eq!(vec!["#AA0000", "#BB0000", "#CC0000", "_"], actual);
    }

    #[test]
    fn test_colors_from_vec() {
        let actual = ZoneColors::from(vec!["#AA0000", "#BB0000", "#CC0000", "_"]);
        let expected = ZoneColors {
            right: Some(Color::from(0xAA0000)),
            center: Some(Color::from(0xBB0000)),
            left: Some(Color::from(0xCC0000)),
            game: None,
        };

        assert_eq!(expected, actual);
    }
}
