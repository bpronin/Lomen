use fmt::Display;
use std::fmt;
use std::fmt::Formatter;

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

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_fmt() {
        assert_eq!(format!("{}", Color::new(0xAA, 0xBB, 0xCC)), "#AABBCC");
    }

    #[test]
    fn test_zone_colors_fmt() {
        let z = ZoneColors {
            right: Color::from(0xAA0000).into(),
            center: Color::from(0xBB0000).into(),
            left: Color::from(0xCC0000).into(),
            game: Color::from(0xDD0000).into(),
        };
        assert_eq!(
            format!("{}", z),
            "R: #AA0000, C: #BB0000, L: #CC0000, G: #DD0000"
        );
    }

    #[test]
    fn test_into() {
        let c: u64 = Color::new(0xAA, 0xBB, 0xCC).into();
        assert_eq!(c, 0x00AABBCC);
    }

    #[test]
    fn test_from() {
        let c: Color = Color::from(0x00AABBCC);
        assert_eq!(c, Color::new(0xAA, 0xBB, 0xCC));
    }
}
