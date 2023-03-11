use macroquad::prelude::*;

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

#[macroquad::main("Daybreak 2")]
async fn main() {
    let mut plr = Player::default();
    let spd = 5.0;
    loop {
        draw_circle(plr.pos.x, plr.pos.y, 32.0, RED);
        if is_key_down(KeyCode::Left) {
            plr.pos.x -= spd;
        }
        if is_key_down(KeyCode::Right) {
            plr.pos.x += spd;
        }
        if is_key_down(KeyCode::Up) {
            plr.pos.y -= spd;
        }
        if is_key_down(KeyCode::Down) {
            plr.pos.y += spd;
        }
        next_frame().await
    }
}
