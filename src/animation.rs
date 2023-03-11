use macroquad::prelude::*;

use crate::graphics::SheetInfo;

pub const FROG_DOWN: AnimDesc = AnimDesc {
    index: 0,
    frames: 3,
};

pub const FROG_LEFT: AnimDesc = AnimDesc {
    index: 1,
    frames: 3,
};

pub const FROG_RIGHT: AnimDesc = AnimDesc {
    index: 2,
    frames: 3,
};

pub const FROG_UP: AnimDesc = AnimDesc {
    index: 3,
    frames: 3,
};

pub const FROG_IDLE_DOWN: AnimDesc = AnimDesc {
    index: 4,
    frames: 3,
};

pub const FROG_IDLE_LEFT: AnimDesc = AnimDesc {
    index: 5,
    frames: 3,
};

pub const FROG_IDLE_RIGHT: AnimDesc = AnimDesc {
    index: 6,
    frames: 3,
};

pub const FROG_IDLE_UP: AnimDesc = AnimDesc {
    index: 7,
    frames: 3,
};

pub struct AnimDesc {
    pub index: u8,
    pub frames: u8,
}

pub struct AnimState {
    pub frame: u8,
    pub desc: AnimDesc,
}

impl AnimState {
    pub fn from_desc(desc: AnimDesc) -> Self {
        Self { frame: 0, desc }
    }
    pub fn advance(&mut self) {
        self.frame += 1;
        if self.frame >= self.desc.frames {
            self.frame = 0;
        }
    }
}

pub fn draw_anim_sprite(texture: Texture2D, sheet: &SheetInfo, anim: &AnimState, x: f32, y: f32) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: None,
            source: Some(Rect {
                x: (sheet.sprite_w * anim.frame as f32),
                y: (sheet.sprite_h * anim.desc.index as f32),
                w: sheet.sprite_w,
                h: sheet.sprite_h,
            }),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    )
}
