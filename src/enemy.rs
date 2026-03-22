use macroquad::prelude::*;
use crate::animation::Animation;
use crate::animation::gerar_frames;
use crate::config::{SPRITE_SIZE, SCALE, INTERNAL_WIDTH, INTERNAL_HEIGHT};


#[derive(Clone, Copy, PartialEq)]
pub enum EnemyKind {
    Normal,
    Red,
    MiniBoss,
    Boss,
}

#[derive(Clone)]
pub struct Enemy {
    pub kind: EnemyKind,
    pub pos: Vec2,
    pub speed: f32,
    pub hp: i32,

    wobble_count: u32, // temporizador para atualizar target_x
    target_x: f32,     // alvo de deslocamento horizontal

    pub texture: Texture2D,
    pub anim: Animation,
    pub vel: Vec2,
    pub rotation: f32,
}

pub struct EnemyVisual {
    pub texture: Texture2D,
    pub frames: Vec<Rect>,
}

// Seleciona a folha de sprites + quadros por tipo de inimigo.
pub fn visual_por_kind(
    kind: EnemyKind,
    normal_texture: Texture2D,
    red_texture: Texture2D,
    miniboss_texture: Texture2D,
    boss_texture: Texture2D,
) -> EnemyVisual {
    match kind {
        EnemyKind::Normal => EnemyVisual {
            texture: normal_texture,
            frames: gerar_frames(16.0, 16.0, 32.0, 16.0),
        },
        EnemyKind::Red => EnemyVisual {
            texture: red_texture,
            frames: gerar_frames(16.0, 16.0, 32.0, 16.0),
        },
        EnemyKind::MiniBoss => EnemyVisual {
            texture: miniboss_texture,
            frames: gerar_frames(16.0, 16.0, 32.0, 16.0),
        },
        EnemyKind::Boss => EnemyVisual {
            texture: boss_texture, // por enquanto reutiliza enemy1
            frames: gerar_frames(32.0, 32.0, 128.0, 32.0),
        },
    }
}

fn scale_for_kind(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Normal => 1.0,
        EnemyKind::Red => 1.2,
        EnemyKind::MiniBoss => 1.6,
        EnemyKind::Boss => 2.0,
    }
}

impl Enemy {
    pub fn new(
        kind: EnemyKind,
        pos: Vec2,
        speed: f32,
        texture: Texture2D,
        frames: Vec<Rect>,
    ) -> Self {
        // HP base por tipo.
        let hp = match kind {
            EnemyKind::Normal => 1,
            EnemyKind::Red => 2,
            EnemyKind::MiniBoss => 8,
            EnemyKind::Boss => 30,
        };

        // Alvo inicial de deslocamento horizontal.
        let target_x = pos.x + rand::gen_range(-50.0, 50.0);

        Self {
            kind,
            pos,
            speed,
            hp,
            wobble_count: rand::gen_range(0, 40),
            target_x,
            texture,
            anim: Animation::new(frames, 0.12),

            vel: vec2(0.0, 1.0),
            rotation: 0.0,
        }
    }
    
    pub fn update_with_speed_mult(&mut self, speed_mult: f32, player_x: f32) {
        if self.kind == EnemyKind::Boss {
            let dt = get_frame_time();

            let scale = scale_for_kind(self.kind);
            let size = SPRITE_SIZE * SCALE * scale;

            // Varredura horizontal pesada e lenta com ricochete nas paredes.
            self.pos.x += self.speed * dt * 60.0;

            let half = size * 0.5;
            if self.pos.x - half < 0.0 {
                self.pos.x = half;
                self.speed = self.speed.abs();
            }
            if self.pos.x + half > INTERNAL_WIDTH as f32 {
                self.pos.x = INTERNAL_WIDTH as f32 - half;
                self.speed = -self.speed.abs();
            }

            // Oscilação vertical leve.
            let t = get_time() as f32;
            self.pos.y = 80.0 + (t * 1.5).sin() * 25.0;

            self.rotation = (t * 0.5).sin() * 0.04;

            self.anim.update();
            return;
        }

        if self.kind == EnemyKind::MiniBoss {
            let dt = get_frame_time();

            let scale = scale_for_kind(self.kind);
            let size = SPRITE_SIZE * SCALE * scale;

            // Centro do mini-chefe (para direção).
            let self_center = self.pos + vec2(size / 2.0, size / 2.0);

            // Centro aproximado do jogador para mirar.
            let player_center = vec2(player_x, INTERNAL_HEIGHT as f32 * 0.5);

            // Direção desejada em direção ao jogador.
            let desired_dir = (player_center - self_center).normalize_or_zero();

            // Quanto ele vira por quadro (menor = mais pesado).
            let turn_strength = 0.065;

            // Direção: mistura velocidade atual com direção desejada.
            self.vel += desired_dir * turn_strength;
            if self.vel.length() > 0.001 {
                self.vel = self.vel.normalize();
            }

            // Aceleração gradual.
            self.speed = (self.speed + 0.035).min(6.5);

            // Move com base na velocidade.
            self.pos += self.vel * self.speed * dt * 60.0;

            // Rotação segue o movimento atual, não o alvo.
            self.rotation = self.vel.y.atan2(self.vel.x) + std::f32::consts::FRAC_PI_2;

            self.anim.update();
            return;
        }

        let dt = get_frame_time();
        let time = get_time() as f32;

        // Velocidade base de queda + oscilação senoidal suave.
        self.pos.y += self.speed * speed_mult * dt * 60.0
            + (time * 10.0).sin() * 2.0;

        self.wobble_count += 1;
        if self.wobble_count >= 40 {
            self.wobble_count = 0;

            self.target_x = self.pos.x + rand::gen_range(-50.0, 50.0);
            self.target_x = self
                .target_x
                .clamp(10.0, INTERNAL_WIDTH as f32 - 26.0);
        }

        // Lerp em direção ao alvo horizontal.
        self.pos.x += (self.target_x - self.pos.x) * 0.15;

        // Recicla inimigos não-chefes que saem da tela para o topo.
        if self.pos.y > INTERNAL_HEIGHT as f32
            && self.kind != EnemyKind::MiniBoss
            && self.kind != EnemyKind::Boss
        {
            self.pos.y = -100.0;
            self.pos.x = rand::gen_range(10.0, INTERNAL_WIDTH as f32 - 26.0);
            self.target_x = self.pos.x;
        }
        // Balanço sutil durante a queda.
        self.rotation = std::f32::consts::PI
            + (get_time() as f32 * 2.0).sin() * 0.065;

        self.anim.update();
    }

    pub fn hitbox(&self) -> Rect {
        let scale = scale_for_kind(self.kind);
        let size = SPRITE_SIZE * SCALE * scale;

        Rect::new(
            self.pos.x,
            self.pos.y,
            size,
            size,
        )
    }

    pub fn draw(&self) {
        let scale = scale_for_kind(self.kind);
        let size = SPRITE_SIZE * SCALE * scale;

        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.anim.frame()),
                dest_size: Some(vec2(size, size)),
                rotation: self.rotation,
                pivot: Some(vec2(
                    self.pos.x + size / 2.0,
                    self.pos.y + size / 2.0,
                )),
                ..Default::default()
            },
        );
    }
}
