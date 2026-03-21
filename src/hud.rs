use macroquad::prelude::*;
use crate::player::Player;
use crate::config::{INTERNAL_HEIGHT, INTERNAL_WIDTH};

const HEART_SIZE: f32 = 16.0;

pub fn draw(
    player: &Player,
    heart_texture: &Texture2D,
    skull_texture: &Texture2D,
    kills: u32,
) {
    let y = INTERNAL_HEIGHT as f32 - 24.0;
    let base_x = 10.0;

    // --- FUNDO DO HUD (leve transparência) ---
    draw_rectangle(
        0.0,
        INTERNAL_HEIGHT as f32 - 26.0,
        INTERNAL_WIDTH as f32,
        26.0,
        Color::new(0.0, 0.0, 0.0, 0.35),
    );

    // --- VIDAS ---
    for i in 0..player.max_hp {
        let x = base_x + (i as f32 * 18.0);

        let alive = i < player.hp;

        let color = if alive {
            WHITE
        } else {
            Color::new(0.25, 0.25, 0.25, 1.0)
        };

        // leve "pulsar" na última vida
        let scale = if alive && i == player.hp - 1 {
            let t = get_time() as f32;
            1.0 + (t * 6.0).sin() * 0.08
        } else {
            1.0
        };

        draw_texture_ex(
            heart_texture,
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(
                    HEART_SIZE * scale,
                    HEART_SIZE * scale,
                )),
                ..Default::default()
            },
        );
    }

    // --- KILLS (lado direito, mais organizado) ---
    let skull_x = INTERNAL_WIDTH as f32 - 80.0;

    draw_texture_ex(
        skull_texture,
        skull_x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(16.0, 16.0)),
            ..Default::default()
        },
    );

    let text = format!("{:04}", kills);

    // sombra (profundidade)
    draw_text(
        &text,
        skull_x + 24.0 + 1.0,
        y + 14.0 + 1.0,
        18.0,
        BLACK,
    );

    // texto principal
    draw_text(
        &text,
        skull_x + 24.0,
        y + 14.0,
        18.0,
        WHITE,
    );
}