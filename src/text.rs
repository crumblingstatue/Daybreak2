use macroquad::prelude::*;
use std::{
    default::Default,
    time::{Duration, Instant},
};

pub struct TextAnim<'a> {
    pub text: &'a str,
    pub cursor: usize,
    pub font: Font,
    pub last_update: Instant,
}

impl<'a> TextAnim<'a> {
    pub fn new(text: &'a str, font: Font) -> Self {
        Self {
            text,
            cursor: 0,
            font,
            last_update: Instant::now(),
        }
    }
    /// SPEED IS IN MILLISECONDS
    pub fn advance_and_draw(&mut self, speed_ms: u32) {
        let elapsed = self.last_update.elapsed();
        if elapsed >= Duration::from_millis(speed_ms.into()) {
            if self.cursor < self.text.len() - 1 {
                self.cursor += 1;
            }
            self.last_update = Instant::now();
        }
        draw_text_ex(
            &self.text[..self.cursor],
            32.0,
            32.0,
            TextParams {
                font_size: 14,
                font: self.font,
                ..Default::default()
            },
        );
    }
}
