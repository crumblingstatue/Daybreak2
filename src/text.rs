use macroquad::prelude::*;
use std::{
    default::Default,
    time::{Duration, Instant},
};

pub struct TextAnim<'a> {
    pub text: &'a str,
    pub line: usize,
    pub cursor: usize,
    pub line_cursor: usize,
    pub font: Font,
    pub last_update: Instant,
}

impl<'a> TextAnim<'a> {
    pub fn new(text: &'a str, font: Font) -> Self {
        Self {
            text,
            line: 0,
            cursor: 0,
            line_cursor: 0,
            font,
            last_update: Instant::now(),
        }
    }
    /// SPEED IS IN MILLISECONDS
    pub fn advance_and_draw(&mut self, x: f32, y: f32, speed_ms: u32) {
        let elapsed = self.last_update.elapsed();
        let mut should_update = false;
        if elapsed >= Duration::from_millis(speed_ms.into()) {
            should_update = true;
            self.last_update = Instant::now();
        }
        if should_update {
            self.cursor += 1;
            self.line_cursor += 1;
            if self.text.get(self.cursor..self.cursor + 1).unwrap_or("") == "\n" {
                self.line += 1;
                self.line_cursor = 0;
            }
        }
        for (i, line) in self.text.lines().enumerate() {
            if i > self.line {
                break;
            }
            dbg!(line);
            let cursor = self.line_cursor.min(line.len());
            draw_text_ex(
                if i < self.line { line } else { &line[..cursor] },
                x,
                y + i as f32 * 16.,
                TextParams {
                    font_size: 14,
                    font: self.font,
                    ..Default::default()
                },
            );
        }
    }
}
