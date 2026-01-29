use crate::color::Color;
use std::time::{Duration, Instant};

pub struct ColorTransition {
    from: Color,
    to: Color,
    start_at: Instant,
    duration: Duration,
    is_finished: bool,
}

impl ColorTransition {
    pub fn new(from: Color, to: Color, duration: Duration) -> Self {
        Self {
            from,
            to,
            start_at: Instant::now(),
            duration,
            is_finished: false,
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
        let mut factor = elapsed.as_secs_f32() / self.duration.as_secs_f32();

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
