use macroquad::prelude::*;

use crate::graphics::{SheetInfo, TileSheetInfo};

pub struct Tilemap {
    pub tiles: Vec<TileData>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy)]
pub struct TileData {
    /// The low tile
    pub lo: TileId,
    /// The high tile, rendered above the low tile
    pub hi: TileId,
}

const EMPTY: TileId = 0;
const EMPTY_TILE: TileData = TileData {
    lo: EMPTY,
    hi: EMPTY,
};

type TileId = u16;

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![EMPTY_TILE; width * height],
            width,
            height,
        }
    }
    pub fn draw(&self, texture: Texture2D, sheet: &SheetInfo, tile_extra: &TileSheetInfo) {
        for (i, td) in self.tiles.iter().enumerate() {
            let x = ((i % self.width) * sheet.sprite_w as usize) as f32;
            let y = ((i / self.width) * sheet.sprite_h as usize) as f32;
            if td.lo != EMPTY {
                draw_tile(texture, x, y, sheet, td.lo, tile_extra.tiles_per_row);
            }
            if td.hi != EMPTY {
                draw_tile(texture, x, y, sheet, td.hi, tile_extra.tiles_per_row);
            }
        }
    }
    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> &mut TileData {
        let idx = y * self.width + x;
        &mut self.tiles[idx]
    }
}

fn draw_tile(
    texture: Texture2D,
    x: f32,
    y: f32,
    sheet: &SheetInfo,
    tile: TileId,
    tiles_per_row: u16,
) {
    let idx = tile - 1;
    let xx = idx % tiles_per_row;
    let yy = idx / tiles_per_row;
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: None,
            source: Some(Rect {
                x: xx as f32 * sheet.sprite_w,
                y: yy as f32 * sheet.sprite_h,
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
