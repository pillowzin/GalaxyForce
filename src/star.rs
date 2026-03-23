use macroquad::prelude::*;
use crate::config::{INTERNAL_WIDTH, INTERNAL_HEIGHT};

#[derive(Clone)]
pub struct Star {
    pub pos: Vec2,
    pub speed: f32,
    pub size: f32,
    pub twinkle_offset: f32,
}

impl Star {

    pub fn new() -> Self {

        Self {

            // Geração aleatória na tela.
            pos: vec2(
                rand::gen_range(0.0, INTERNAL_WIDTH as f32),
                rand::gen_range(0.0, INTERNAL_HEIGHT as f32),
            ),

            speed: rand::gen_range(20.0, 80.0),
            size: rand::gen_range(1.0, 2.0),
            twinkle_offset: rand::gen_range(0.0, 10.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
     // Deriva para baixo com leve balanço horizontal.
        self.pos.y += self.speed * dt;
        self.pos.x += (self.pos.y * 0.05).sin() * 10.0 * dt;

        if self.pos.y > INTERNAL_HEIGHT as f32 {

            // Volta para o topo quando sai da tela.
            self.pos.y = 0.0;

            self.pos.x = rand::gen_range(
                0.0,
                INTERNAL_WIDTH as f32,
            );
        }
    }

    pub fn draw(&self, camera_offset: Vec2) {
        let t = get_time() as f32;

        // Brilho baseado no tempo + deslocamento por estrela.
        let twinkle = (t * 3.0 + self.twinkle_offset).sin() * 0.3 + 0.7;

        // Profundidade controla paralaxe e brilho.
        let depth = (self.speed / 80.0).clamp(0.2, 1.0);
        let brightness = (self.speed / 80.0).clamp(0.3, 1.0);

        let color = Color::new(
            brightness,
            brightness,
            brightness + 0.2,
            1.0,
        );
        draw_circle(
            self.pos.x + camera_offset.x * depth,
            self.pos.y + camera_offset.y * depth,
            self.size * twinkle,
            color,
        );
    }
}
