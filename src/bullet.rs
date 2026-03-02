use macroquad::prelude::*;
use crate::config::{PLAYER_BULLET_HEIGHT, PLAYER_BULLET_WIDTH};

pub struct Bullet {
    pub pos: Vec2,
    speed: f32,
}

impl Bullet {
    pub fn new(origin: Vec2) -> Self {
        Self {
            pos: origin,
            speed: 120.0,
        }
    }

    pub fn update(&mut self) {
        self.pos.y -= self.speed * get_frame_time();
    }

    pub fn draw(&self) {
        let width = PLAYER_BULLET_WIDTH;
        let height = PLAYER_BULLET_HEIGHT;

        draw_rectangle(
            self.pos.x - width / 2.0,
            self.pos.y - height,
            width,
            height,
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
        self.pos.y < -16.0
    }
}
