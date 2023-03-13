mod animation;
mod graphics;
mod text;
mod tiles;

use animation::{draw_anim_sprite, AnimDesc, AnimState};
use egui_macroquad::egui;
use gamedebug_core::{imm_dbg, Info};
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

fn window_config() -> Conf {
    Conf {
        window_title: "Daybreak 2".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(PartialEq, Eq)]
enum UiTab {
    Textbox,
    LevelEdit,
    DebugLog,
}

impl UiTab {
    fn label(&self) -> &'static str {
        match self {
            UiTab::Textbox => "Text box",
            UiTab::LevelEdit => "Level edit",
            UiTab::DebugLog => "Debug log",
        }
    }
}

#[macroquad::main(window_config)]
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
    let mut ui_tab = UiTab::Textbox;

    //Test
    let font = load_ttf_font("./res/fonts/EightBitDragon-anqx.ttf")
        .await
        .unwrap();

    //GONN BE LOADING TEXTURES HERE SO PROLLY DEWWY IS GONN MOVE IT SOMEWHERE BETTER
    let d_box_line_tex = load_texture("./res/d_box_line.png").await.unwrap();
    let mut text_msg_buf = String::new();
    let mut ta = text::TextAnim::new(font);

    loop {
        clear_background(BLACK);
        let mp = mouse_position();
        let (tx, ty) = ((mp.0 / 16.).floor(), (mp.1 / 16.).floor());
        imm_dbg!((tx, ty));
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Daybreak 2").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut ui_tab, UiTab::Textbox, UiTab::Textbox.label());
                    ui.selectable_value(&mut ui_tab, UiTab::LevelEdit, UiTab::LevelEdit.label());
                    ui.selectable_value(&mut ui_tab, UiTab::DebugLog, UiTab::DebugLog.label());
                });
                ui.separator();
                match ui_tab {
                    UiTab::Textbox => {
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut text_msg_buf)
                                        .hint_text("Message"),
                                );
                            });
                        ui.horizontal(|ui| {
                            ui.label("Update delay");
                            ui.add(egui::DragValue::new(&mut ta.update_delay_ms).suffix("ms"));
                            if ui.button("Show text box").clicked() {
                                ta.set_text(text_msg_buf.clone());
                            }
                        });
                    }
                    UiTab::LevelEdit => {
                        if ui.button("Randomize tiles").clicked() {
                            for y in 0..tilemap.height {
                                for x in 0..tilemap.width {
                                    tilemap.tile_at_mut(x, y).lo = rand::gen_range(1u32, 32) as u16;
                                }
                            }
                        }
                    }
                    UiTab::DebugLog => {
                        gamedebug_core::set_enabled(true);

                        for info in gamedebug_core::IMMEDIATE.lock().unwrap().iter() {
                            match info {
                                Info::Msg(msg) => {
                                    ui.label(msg);
                                }
                                Info::Rect(..) => todo!(),
                            }
                        }
                        ui.separator();
                        for entry in gamedebug_core::PERSISTENT.lock().unwrap().iter() {
                            match &entry.info {
                                Info::Msg(msg) => {
                                    ui.label(format!("{}: {}", entry.frame, msg));
                                }
                                Info::Rect(..) => todo!(),
                            }
                        }
                    }
                }
            });
        });
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
        draw_rectangle_lines(tx * 16., ty * 16., 16., 16., 2., WHITE);

        //Test text
        ta.advance_and_draw(32., 32., d_box_line_tex);
        gamedebug_core::clear_immediates();
        gamedebug_core::inc_frame();

        egui_macroquad::draw();
        next_frame().await
    }
}
