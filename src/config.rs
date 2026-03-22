use macroquad::prelude::*;

// Tamanho lógico base do sprite (antes da escala).
pub const SPRITE_SIZE: f32 = 24.0;
pub const SCALE: f32 = 1.0;

pub const PLAYER_BULLET_WIDTH: f32 = SPRITE_SIZE * 0.20;
pub const PLAYER_BULLET_HEIGHT: f32 = SPRITE_SIZE * 0.30;

// Resolução interna de render (espaço da lógica do jogo).
pub const INTERNAL_WIDTH: u32 = 256;
pub const INTERNAL_HEIGHT: u32 = 350;

// Fator de escala da janela sobre a resolução interna.
pub const WINDOW_WIDTH: i32 = INTERNAL_WIDTH as i32 * 2;
pub const WINDOW_HEIGHT: i32 = INTERNAL_HEIGHT as i32 * 2;

// Converte a posição do mouse do SO para coordenadas internas.
pub fn mouse_internal() -> Vec2 {
    let (mx, my) = mouse_position();
    vec2(
        mx * INTERNAL_WIDTH as f32 / screen_width(),
        my * INTERNAL_HEIGHT as f32 / screen_height(),
    )
}
