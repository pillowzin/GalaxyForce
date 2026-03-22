 use macroquad::prelude::*;

pub struct ThrusterParticle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
}

impl ThrusterParticle {

    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(
                rand::gen_range(-2.0, 2.0),   // espalha pouco
                rand::gen_range(3.0, 6.0),    // cai bem devagar
            ),
            life: rand::gen_range(0.6, 1.0),
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        // movimento lento
        self.pos += self.vel * dt * 20.0;

        // gravidade bem leve
        self.vel.y += 4.0 * dt;

        // scroll do cenário
        self.pos.y += 10.0 * dt;

        self.life -= dt;
    }

    pub fn draw(&self) {

        let alpha = (self.life * 1.2).clamp(0.0, 1.0);

        // snap para pixel grid
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();

        // pixel 4x4 sólido
        draw_rectangle(
            x,
            y,
            4.0,
            4.0,
            Color::new(1.0, 0.6, 0.1, alpha),
        );
    }

    pub fn dead(&self) -> bool {
        self.life <= 0.0
    }
}
