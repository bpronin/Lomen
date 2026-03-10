use crate::color::{Color, LightingColors};
use std::time::{Duration, Instant};

struct ColorTransition {
    from: Color,
    to: Color,
    start_at: Instant,
    duration: Duration,
    is_finished: bool,
}

impl ColorTransition {
    pub fn new(from: Option<Color>, to: Option<Color>, duration: Duration) -> Self {
        Self {
            from: from.unwrap_or(Color::new(0, 0, 0)),
            to: to.unwrap_or(Color::new(0, 0, 0)),
            start_at: Instant::now(),
            duration,
            is_finished: from.is_none() || to.is_none(),
        }
    }
}

impl Iterator for ColorTransition {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        let elapsed = self.start_at.elapsed();

        let mut factor = elapsed.div_duration_f32(self.duration);
        if factor >= 1.0 {
            factor = 1.0;
            self.is_finished = true;
        }

        let interpolate = |start: u8, end: u8, f: f32| -> u8 {
            (start as f32 + (end as i16 - start as i16) as f32 * f) as u8
        };

        Some(Color::new(
            interpolate(self.from.r, self.to.r, factor),
            interpolate(self.from.g, self.to.g, factor),
            interpolate(self.from.b, self.to.b, factor),
        ))
    }
}

pub(crate) struct LightingColorsTransition {
    right: ColorTransition,
    center: ColorTransition,
    left: ColorTransition,
    game: ColorTransition,
}

impl LightingColorsTransition {
    pub fn new(from: LightingColors, to: LightingColors, duration: Duration) -> Self {
        Self {
            right: ColorTransition::new(from.right, to.right, duration),
            center: ColorTransition::new(from.center, to.center, duration),
            left: ColorTransition::new(from.left, to.left, duration),
            game: ColorTransition::new(from.game, to.game, duration),
        }
    }
}

impl Iterator for LightingColorsTransition {
    type Item = LightingColors;

    fn next(&mut self) -> Option<Self::Item> {
        if self.right.is_finished
            && self.center.is_finished
            && self.left.is_finished
            && self.game.is_finished
        {
            return None;
        }

        Some(LightingColors {
            right: self.right.next(),
            center: self.center.next(),
            left: self.left.next(),
            game: self.game.next(),
        })
    }
}
