use macroquad::prelude::*;
use crate::config::{INTERNAL_WIDTH, INTERNAL_HEIGHT};

pub struct EnemyBullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub bounces_left: i32, // ricochetes restantes nas paredes
}

impl EnemyBullet {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
            radius: 4.0,      // bala redonda pixelada
            bounces_left: 4,  // quantidade de ricochetes
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.pos += self.vel * dt * 60.0;

        // ricochete horizontal
        if self.pos.x - self.radius <= 0.0 || self.pos.x + self.radius >= INTERNAL_WIDTH as f32 {
            self.vel.x *= -1.0;
            self.bounces_left -= 1;
        }

        // ricochete vertical
        if self.pos.y - self.radius <= 0.0 || self.pos.y + self.radius >= INTERNAL_HEIGHT as f32 {
            self.vel.y *= -1.0;
            self.bounces_left -= 1;
        }
    }

    pub fn is_dead(&self) -> bool {
        // A bala expira após um número fixo de ricochetes.
        self.bounces_left <= 0
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(
            self.pos.x - self.radius,
            self.pos.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
}
