use macroquad::prelude::*;
use std::default::Default;

pub struct TextAnim {
    pub text: &'static str,
    pub font: Font,
}
impl TextAnim {
    pub fn draw(&self) {
        draw_text_ex(
            self.text,
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
