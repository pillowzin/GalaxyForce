use macroquad::prelude::*;
use crate::config::{PLAYER_BULLET_HEIGHT, PLAYER_BULLET_WIDTH};

pub struct Bullet {
    pub pos: Vec2,
    speed: f32, // velocidade para cima em pixels/seg
}

impl Bullet {
    pub fn new(origin: Vec2) -> Self {
        Self {
            pos: origin,
            speed: 400.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Move para cima a cada quadro.
        self.pos.y -= self.speed * dt;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - PLAYER_BULLET_WIDTH / 2.0,
            self.pos.y - PLAYER_BULLET_HEIGHT,
            PLAYER_BULLET_WIDTH,
            PLAYER_BULLET_HEIGHT,
            RED,
        );
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(
            self.pos.x - PLAYER_BULLET_WIDTH / 2.0,
            self.pos.y - PLAYER_BULLET_HEIGHT,
            PLAYER_BULLET_WIDTH,
            PLAYER_BULLET_HEIGHT,
        )
    }

    pub fn offscreen(&self) -> bool {
        // Limite conservador acima do topo da tela.
        self.pos.y < -16.0
    }
}
