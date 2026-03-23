use macroquad::prelude::*;

pub fn gerar_frames(
    frame_w: f32,
    frame_h: f32,
    sheet_w: f32,
    sheet_h: f32,
) -> Vec<Rect> {
    let mut frames = Vec::new();

    // Calcula o grid com base nas dimensões da folha.
    let cols = (sheet_w / frame_w) as i32;
    let rows = (sheet_h / frame_h) as i32;

    for y in 0..rows {
        for x in 0..cols {
            frames.push(Rect::new(
                x as f32 * frame_w,
                y as f32 * frame_h,
                frame_w,
                frame_h,
            ));
        }
    }
    frames
}

#[derive(Clone)]
pub struct Animation {
    frames: Vec<Rect>,
    current: usize,
    timer: f32,
    speed: f32,
}

impl Animation {
    pub fn new(frames: Vec<Rect>, speed: f32) -> Self {
        Self {
            frames,
            current: 0,
            timer: 0.0,
            speed,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Avança para o próximo quadro com base no tempo decorrido.
        self.timer += dt;
        while self.timer >= self.speed {
            self.timer -= self.speed;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    pub fn frame(&self) -> Rect {
        self.frames[self.current]
    }
}
