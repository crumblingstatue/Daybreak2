mod animation;
mod graphics;
mod tiles;

use animation::{draw_anim_sprite, AnimDesc, AnimState};
use graphics::{SheetInfo, TileSheetInfo};
use macroquad::prelude::*;
use tiles::Tilemap;

struct Player {
    pos: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: vec2(100., 100.),
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

fn select_frog_anim(dir: Dir, idle: bool) -> AnimDesc {
    use animation::*;
    match (dir, idle) {
        (Dir::Left, true) => FROG_IDLE_LEFT,
        (Dir::Left, false) => FROG_LEFT,
        (Dir::Up, true) => FROG_IDLE_UP,
        (Dir::Up, false) => FROG_UP,
        (Dir::Right, true) => FROG_IDLE_RIGHT,
        (Dir::Right, false) => FROG_RIGHT,
        (Dir::Down, true) => FROG_IDLE_DOWN,
        (Dir::Down, false) => FROG_DOWN,
    }
}

#[macroquad::main("Daybreak 2")]
async fn main() {
    let mut plr = Player::default();
    let spd = 5.0;
    let frog_texture = Texture2D::from_file_with_format(include_bytes!("../res/frog.png"), None);
    let tiles_texture =
        Texture2D::from_file_with_format(include_bytes!("../res/RPGTileset.png"), None);
    let frog_sheet = SheetInfo {
        sprite_w: 48.,
        sprite_h: 48.,
    };
    let tile_sheet = SheetInfo {
        sprite_w: 16.,
        sprite_h: 16.,
    };
    let tile_sheet_extra = TileSheetInfo { tiles_per_row: 4 };
    let mut frog_anim = AnimState::from_desc(animation::FROG_IDLE_DOWN);
    let mut frame_counter = 0;
    let mut frog_dir = Dir::Down;
    let mut tilemap = Tilemap::new(50, 38);
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            tilemap.tile_at_mut(x, y).lo = rand::gen_range(1u32, 32) as u16;
        }
    }
    loop {
        let mut any_pressed = false;
        if is_key_down(KeyCode::Left) {
            frog_dir = Dir::Left;
            plr.pos.x -= spd;
            any_pressed = true;
        }
        if is_key_down(KeyCode::Right) {
            frog_dir = Dir::Right;
            plr.pos.x += spd;
            any_pressed = true;
        }
        if is_key_down(KeyCode::Up) {
            frog_dir = Dir::Up;
            plr.pos.y -= spd;
            any_pressed = true;
        }
        if is_key_down(KeyCode::Down) {
            frog_dir = Dir::Down;
            plr.pos.y += spd;
            any_pressed = true;
        }
        if frame_counter % 10 == 0 {
            frog_anim.advance();
        }
        frame_counter += 1;
        frog_anim.desc = select_frog_anim(frog_dir, !any_pressed);
        tilemap.draw(tiles_texture, &tile_sheet, &tile_sheet_extra);
        draw_circle(plr.pos.x, plr.pos.y, 4.0, RED);
        draw_anim_sprite(
            frog_texture,
            &frog_sheet,
            &frog_anim,
            plr.pos.x - 24.,
            plr.pos.y - 24.,
        );
        next_frame().await
    }
}
