use macroquad::prelude::*;

pub const SPRITE_SIZE: f32 = 24.0;
pub const SCALE: f32 = 1.0;

pub const PLAYER_BULLET_WIDTH: f32 = SPRITE_SIZE * 0.22;
pub const PLAYER_BULLET_HEIGHT: f32 = SPRITE_SIZE * 0.55;

pub const INTERNAL_WIDTH: u32 = 256;
pub const INTERNAL_HEIGHT: u32 = 350;

pub const WINDOW_COLS: u32 = 32;
pub const WINDOW_ROWS: u32 = 30;

pub const WINDOW_WIDTH: i32 = INTERNAL_WIDTH as i32 * 2;
pub const WINDOW_HEIGHT: i32 = INTERNAL_HEIGHT as i32 * 2;

pub fn mouse_internal() -> Vec2 {
    let (mx, my) = mouse_position();
    vec2(
        mx * INTERNAL_WIDTH as f32 / screen_width(),
        my * INTERNAL_HEIGHT as f32 / screen_height(),
    )
}
