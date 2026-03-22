use macroquad::prelude::*;
use crate::config::{SCALE};

pub struct Explosion {
    pos: Vec2,
    frames: Vec<Rect>,
    current_frame: usize,
    frame_timer: f32,
    frame_duration: f32,
    finished: bool,
    texture: Texture2D,
    scale: f32,
}

impl Explosion {
    pub fn new(
        pos: Vec2,
        texture: Texture2D,
        frames: Vec<Rect>,
    ) -> Self {
        Self {
            pos,
            frames,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.08, // velocidade da explosão
            finished: false,
            texture,
            scale: SCALE,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.finished {
            return;
        }

        // Avança a animação baseado no tempo de quadro.
        self.frame_timer += dt;

        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                self.finished = true;
            }
        }
    }

    pub fn draw(&self) {
        if self.finished {
            return;
        }

        // Desenha o quadro atual escalado para o tamanho do jogo.
        let frame = self.frames[self.current_frame];

        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(frame),
                dest_size: Some(vec2(
                    frame.w * self.scale,
                    frame.h * self.scale,
                )),
                ..Default::default()
            },
        );
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
