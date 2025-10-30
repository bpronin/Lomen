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
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return Err("Hex string must be 6 characters long".into());
        }
        Ok(Self {
            r: u8::from_str_radix(&s[0..2], 16).map_err(|e| format!("{}", e))?,
            g: u8::from_str_radix(&s[2..4], 16).map_err(|e| format!("{}", e))?,
            b: u8::from_str_radix(&s[4..6], 16).map_err(|e| format!("{}", e))?,
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b))
    }
}

#[derive(Debug)]
pub struct ZoneColors {
    pub right: Option<Color>,
    pub center: Option<Color>,
    pub left: Option<Color>,
    pub game: Option<Color>,
}

impl Display for ZoneColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "R: {}, C: {}, L: {}, G: {}",
            self.right.unwrap_or_default(),
            self.center.unwrap_or_default(),
            self.left.unwrap_or_default(),
            self.game.unwrap_or_default()
        ))
    }
}

#[test]
fn test_color_fmt() {
    assert_eq!(format!("{}", Color::new(0xAA, 0xBB, 0xCC)), "#AABBCC");
}

#[test]
fn test_zone_colors_fmt() {
    let z = ZoneColors {
        right: Color::from_str("#AA0000").ok(),
        center: Color::from_str("#BB0000").ok(),
        left: Color::from_str("#CC0000").ok(),
        game: Color::from_str("#DD0000").ok(),
    };
    assert_eq!(format!("{}", z), "R: #AA0000, C: #BB0000, L: #CC0000, G: #DD0000");
}

#[test]
fn test_from_str() {
    assert_eq!(
        Color::new(0xAA, 0xBB, 0xCC),
        Color::from_str("#AABBCC").unwrap()
    );
    assert_eq!(
        Color::new(0xAA, 0xBB, 0xCC),
        Color::from_str("AABBCC").unwrap()
    );
}
