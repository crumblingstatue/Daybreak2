use macroquad::miniquad::Texture;
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
    pub fn advance_and_draw(&mut self, x: f32, y: f32, speed_ms: u32, d_box_line_tex: Texture2D) {
        // Draw rectangle box
        //let rect_color = Color::new(255., 153., 153., 255.); // LOL WUT WHY IT DRAWING WHITE INSTEAD OF PINKISH
        draw_rectangle(
            32.0,
            screen_height() - 200.0,
            screen_width() - 64.0,
            180.0,
            DARKGREEN,
        );
        //DRAW STUFF AROUND DBOX
        draw_texture_ex(
            d_box_line_tex,
            32.,
            388.,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(Vec2 {
                    x: screen_width() - 64.,
                    y: 12.,
                }),
                ..Default::default()
            },
        );
        draw_texture_ex(
            d_box_line_tex,
            32.,
            screen_height() - 20.,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(Vec2 {
                    x: screen_width() - 64.,
                    y: 12.,
                }),
                ..Default::default()
            },
        );
        draw_texture_ex(
            d_box_line_tex,
            -76.,
            484.,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(Vec2 { x: 204., y: 12. }),
                rotation: 1.5708,
                ..Default::default()
            },
        );
        draw_texture_ex(
            d_box_line_tex,
            672.,
            484.,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(Vec2 { x: 204., y: 12. }),
                rotation: 1.5708,
                ..Default::default()
            },
        );

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
            let cursor = self.line_cursor.min(line.len());
            let origin_y: f32 = 395.0;
            let origin_x: f32 = 20.0;
            draw_text_ex(
                if i < self.line { line } else { &line[..cursor] },
                x + origin_x,
                (y + i as f32 * 16.) + origin_y,
                TextParams {
                    font_size: 14,
                    font: self.font,
                    ..Default::default()
                },
            );
        }
    }
}
