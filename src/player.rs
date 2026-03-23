use macroquad::prelude::*;
use crate::config::{SPRITE_SIZE, SCALE, INTERNAL_WIDTH, INTERNAL_HEIGHT};
use crate::config::mouse_internal;
use crate::thruster::ThrusterParticle;

pub struct Player {
    pub pos: Vec2,
    texture: Texture2D,
    hit_timer: f32,      // temporizador de flash de invulnerabilidade
    shake_timer: f32,    // temporizador visual do recuo
    particles: Vec<ThrusterParticle>,
    last_hit_dir: Vec2,
    pub hp: i32,
    pub max_hp: i32,
    pub heart_anim_frame: usize,
    heart_anim_timer: f32, // animação de perda de coração da interface
    pub heart_anim_index: i32,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            pos: vec2(200.0, 400.0),
            texture,

            particles: Vec::new(),
            hit_timer: 0.0,
            shake_timer: 0.0,
            last_hit_dir: Vec2::ZERO,

            hp: 4,
            max_hp: 4,
            heart_anim_frame: 0,
            heart_anim_timer: 0.0,
            heart_anim_index: -1,
        }
    }
    fn size(&self) -> f32 {
        SPRITE_SIZE * SCALE
    }

    pub fn update(&mut self, dt: f32) {
        // Diminui os temporizadores de hit/flash.
        self.hit_timer = (self.hit_timer - dt).max(0.0);
        self.shake_timer = (self.shake_timer - dt).max(0.0);

        let mouse = mouse_internal();
        let size = self.size();

        // Movimento pelo cursor (limitado aos limites da tela).
        self.pos.x = (mouse.x - size / 2.0)
            .clamp(0.0, INTERNAL_WIDTH as f32 - size);

        self.pos.y = (mouse.y - size / 2.0)
            .clamp(0.0, INTERNAL_HEIGHT as f32 - size);

        let size = self.size();

        self.pos.x = self.pos.x.clamp(0.0, INTERNAL_WIDTH as f32 - size);
        self.pos.y = self.pos.y.clamp(0.0, INTERNAL_HEIGHT as f32 - size);

        let engine = vec2(
            self.pos.x + size * 0.5,
            self.pos.y + size,
        );

        // Emissão aleatória do propulsor para ruído visual.
        if rand::gen_range(0, 2) == 0 {
            self.particles.push(ThrusterParticle::new(engine));
        }

        // Atualiza partículas do propulsor.
        for p in self.particles.iter_mut() {
            p.update(dt);
        }

        // Remove partículas mortas.
        self.particles.retain(|p| !p.dead());
        
        // Anima o ícone do coração do último HP perdido.
        if self.heart_anim_index >= 0 {
            const HEART_ANIM_FRAMES: usize = 8;
            const HEART_ANIM_FRAME_TIME: f32 = 0.05;

            self.heart_anim_timer += dt;
            if self.heart_anim_timer >= HEART_ANIM_FRAME_TIME {
                self.heart_anim_timer -= HEART_ANIM_FRAME_TIME;
                self.heart_anim_frame += 1;

                if self.heart_anim_frame >= HEART_ANIM_FRAMES {
                    self.heart_anim_frame = 0;
                    self.heart_anim_index = -1;
                }
            }
        }
    }

    // Aplica dano e retorna true se o jogador morreu.
    pub fn damage(&mut self) -> bool {
        self.hp = (self.hp - 1).max(0);

        // Anima o coração que acabou de perder.
        self.heart_anim_index = self.hp;
        self.heart_anim_frame = 0;
        self.heart_anim_timer = 0.0;

        self.hp <= 0
    }

    pub fn hitbox(&self) -> Rect {
        let size = self.size();
        Rect::new(self.pos.x, self.pos.y, size, size)
    }

    // Dispara feedback de hit (flash + shake).
    pub fn hit(&mut self, from: Vec2) {
        self.hit_timer = 0.2;
        self.shake_timer = 0.12;

        self.last_hit_dir = (self.pos - from).normalize_or_zero();
    }

    pub fn is_flashing(&self) -> bool {
        self.hit_timer > 0.0
    }

    pub fn draw(&self) {
        let size = self.size();

        let mut draw_pos = self.pos;
        if self.shake_timer > 0.0 {
            let strength = 4.0 * (self.shake_timer / 0.12);
            draw_pos += self.last_hit_dir * strength;
        }

        let flashing = self.hit_timer > 0.0;
        let flash_alpha = (self.hit_timer / 0.2).clamp(0.0, 1.0);

        // Desenha partículas do propulsor atrás da nave.
        for p in &self.particles {
            p.draw();
        }

        draw_texture_ex(
            &self.texture,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );

        if flashing {
            draw_texture_ex(
                &self.texture,
                draw_pos.x - 2.0,
                draw_pos.y - 2.0,
                Color::new(1.0, 1.0, 1.0, flash_alpha),
                DrawTextureParams {
                    dest_size: Some(vec2(size + 4.0, size + 4.0)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn reset(&mut self) {
        self.pos = vec2(
            INTERNAL_WIDTH as f32 * 0.5,
            INTERNAL_HEIGHT as f32 * 0.7,
        );

        self.hit_timer = 0.0;
        self.shake_timer = 0.0;
        self.last_hit_dir = Vec2::ZERO;
        self.heart_anim_frame = 0;
        self.heart_anim_timer = 0.0;
        self.heart_anim_index = -1;
        self.hp = self.max_hp;
    }
}
