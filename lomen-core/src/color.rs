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
        ((self.r as u64) << 16) | ((self.g as u64) << 8) | (self.b as u64)
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
pub struct LightingColors {
    pub right: Option<Color>,
    pub center: Option<Color>,
    pub left: Option<Color>,
    pub game: Option<Color>,
}

impl Display for LightingColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let format = |c: Option<Color>| -> String {
            c.map(|c| c.to_string()).unwrap_or_else(|| "_".to_string())
        };

        write!(
            f,
            "[{}, {}, {}, {}]",
            format(self.right),
            format(self.center),
            format(self.left),
            format(self.game)
        )
    }
}

// impl FromStr for LightingColors {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let values: Vec<Option<Color>> = s
//             .trim_start_matches("[")
//             .trim_end_matches("]")
//             .split(',')
//             .map(|s| Self::parse_color(s.trim()).ok())
//             .collect();
//         Ok(Self {
//             right: values[0],
//             center: values[1],
//             left: values[2],
//             game: values[3],
//         })
//     }
// }

impl Into<Vec<String>> for LightingColors {
    fn into(self) -> Vec<String> {
        let format = |c: Option<Color>| -> String {
            c.map(|c| c.to_string()).unwrap_or_else(|| "".to_string())
        };

        vec![
            format(self.right),
            format(self.center),
            format(self.left),
            format(self.game),
        ]
    }
}

impl From<Vec<String>> for LightingColors {
    fn from(value: Vec<String>) -> Self {
        let parse = |s: &str| -> Option<Color> {
            if s.is_empty() {
                None
            } else {
                Some(Color::from_str(&s).expect(&format!("Error parsing color from: `{s}`")))
            }
        };

        Self {
            right: parse(&value[0]),
            center: parse(&value[1]),
            left: parse(&value[2]),
            game: parse(&value[3]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        assert_eq!(Color::new(0xAA, 0xBB, 0xCC), Color::from(0xAABBCC));
    }

    #[test]
    fn test_color_rgb() {
        let c = Color::new(0xAA, 0xBB, 0xCC);

        assert_eq!(0xAA, c.r);
        assert_eq!(0xBB, c.g);
        assert_eq!(0xCC, c.b);
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
        let colors = LightingColors {
            right: Some(Color::from(0xAA0000)),
            center: Some(Color::from(0xBB0000)),
            left: None,
            game: Some(Color::from(0xDD0000)),
        };

        assert_eq!(format!("{}", colors), "[#AA0000, #BB0000, _, #DD0000]");
    }

    // #[test]
    // fn test_colors_from_str() {
    //     let colors = LightingColors {
    //         right: Some(Color::from(0xAA0000)),
    //         center: Some(Color::from(0xBB0000)),
    //         left: Some(Color::from(0xCC0000)),
    //         game: None,
    //     };
    //
    //     assert_eq!(
    //         colors,
    //         LightingColors::from_str("[#AA0000, #BB0000, #CC0000, _]").unwrap()
    //     );
    // }

    #[test]
    fn test_colors_into_vec() {
        let colors = LightingColors {
            right: Some(Color::from(0xAA0000)),
            center: Some(Color::from(0xBB0000)),
            left: Some(Color::from(0xCC0000)),
            game: None,
        };
        let actual: Vec<String> = colors.into();

        assert_eq!(vec!["#AA0000", "#BB0000", "#CC0000", ""], actual);
    }

    #[test]
    fn test_colors_from_vec() {
        let colors = LightingColors::from(vec![
            "#AA0000".to_string(),
            "#BB0000".to_string(),
            "#CC0000".to_string(),
            "".to_string(),
        ]);

        assert_eq!(
            LightingColors {
                right: Some(Color::from(0xAA0000)),
                center: Some(Color::from(0xBB0000)),
                left: Some(Color::from(0xCC0000)),
                game: None,
            },
            colors
        );
    }
}
