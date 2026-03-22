use macroquad::prelude::*;
use rand::gen_range;
use crate::config::{INTERNAL_WIDTH, INTERNAL_HEIGHT};

const W: f32 = INTERNAL_WIDTH as f32;
const H: f32 = INTERNAL_HEIGHT as f32;
const TILE: f32 = 8.0;
const TITLE_Y: f32 = 15.0 * TILE; // 80
const MENU_START_Y: f32 = 23.0 * TILE;
const MENU_GAP: f32 = 3.0 * TILE;      // 24
const STAR_COUNT: usize = 100;

#[derive(Clone)]
struct Star {
    pos: Vec2,
    speed: f32,
}

struct Spark {
    x: f32,
    y: f32,
    size: f32,
    color: Color,
}

pub struct MenuState {
    time: f32,
    stars: Vec<Star>,
    fade: f32, // desvanecimento de entrada ao entrar
    speaker: Texture2D,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Start,
    Mute,
    Quit,
    None,
}

impl MenuState {
    pub fn new(speaker: Texture2D) -> Self {
        let stars = (0..STAR_COUNT)
            .map(|_| Star {
                pos: vec2(
                    gen_range(0.0, W),
                    gen_range(0.0, H),
                ),
                speed: gen_range(10.0, 40.0),
            })
            .collect();

        Self {
            time: 0.0,
            stars,
            fade: 1.0,
            speaker,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        if self.fade > 0.0 {
            self.fade -= dt * 0.8;
        }

        for star in self.stars.iter_mut() {
            star.pos.y += star.speed * dt;
            if star.pos.y > H {
                star.pos.y = 0.0;
                star.pos.x = gen_range(0.0, W);
            }
        }
    }

    pub fn draw(&self, font: &Font, muted: bool) {
        // Preenchimento do fundo.
        draw_rectangle(
            0.0,
            0.0,
            W,
            H,
            Color::new(0.02, 0.02, 0.05, 1.0),
        );

        // Campo de estrelas com brilho sutil.
        for star in &self.stars {
            let twinkle = (self.time * 4.0 + star.pos.x).sin() * 0.4 + 0.6;
            draw_rectangle(
                star.pos.x,
                star.pos.y,
                1.0,
                1.0,
                Color::new(twinkle, twinkle, twinkle, 1.0),
            );
        }

        // Animação de flutuação do título.
        let offset = (self.time * 1.5).sin() * 2.0;
        let center_x = W * 0.5;
        let title_size = 28;

        let a = "GALAXY";
        let b = "FORCE";

        let da = measure_text(a, Some(font), title_size, 1.0);
        let db = measure_text(b, Some(font), title_size, 1.0);

        let glow = (self.time * 3.0).sin() * 0.2 + 0.8;

        // sombra
        draw_text_ex(
            a,
            center_x - da.width / 2.0 + 3.0,
            TITLE_Y + offset + 3.0,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: Color::new(0.0, 0.0, 0.0, 0.6),
                ..Default::default()
            },
        );

        // brilho vermelho
        draw_text_ex(
            a,
            center_x - da.width / 2.0,
            TITLE_Y + offset,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: Color::new(glow, 0.2, 0.2, 1.0),
                ..Default::default()
            },
        );

        // realce
        draw_text_ex(
            a,
            center_x - da.width / 2.0,
            TITLE_Y + offset - 1.0,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: WHITE,
                ..Default::default()
            },
        );

        // FORCE (parte do título)

        draw_text_ex(
            b,
            center_x - db.width / 2.0 + 3.0,
            TITLE_Y + 2.0 * TILE + offset + 3.0,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: Color::new(0.0, 0.0, 0.0, 0.6),
                ..Default::default()
            },
        );

        draw_text_ex(
            b,
            center_x - db.width / 2.0,
            TITLE_Y + 2.0 * TILE + offset,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: Color::new(0.2, 0.6 * glow, 1.0, 1.0),
                ..Default::default()
            },
        );

        // Espiral de faíscas atrás do título para profundidade.
        let center = vec2(center_x, TITLE_Y - 28.0);
        let radius = 80.0;

        let mut sparks: Vec<Spark> = Vec::with_capacity(70);

        for i in 0..70 {
            let angle = self.time * 0.8 + i as f32 * 0.2;
            let spiral = i as f32 * 0.7;

            let x = center.x + angle.cos() * (radius + spiral * 0.3);
            let y = center.y + angle.sin() * (radius * 0.35)
                + (self.time * 35.0 + spiral) % 40.0;

            let r = (self.time * 2.0 + i as f32).sin() * 0.5 + 0.5;
            let g = (self.time * 1.5 + i as f32 * 0.7).sin() * 0.5 + 0.5;
            let b = (self.time * 1.2 + i as f32 * 1.3).sin() * 0.5 + 0.5;

            let size = 1.5 + (self.time * 5.0 + i as f32).sin() * 0.6;

            sparks.push(Spark {
                x,
                y,
                size,
                color: Color::new(r, g, b, 0.9),
            });
        }

        let depth_line = TITLE_Y + TILE;

        // PARTÍCULAS ATRÁS DO TÍTULO
        for s in sparks.iter().filter(|s| s.y < depth_line) {
            draw_rectangle(
                s.x,
                s.y,
                s.size,
                s.size * 1.4,
                Color::new(s.color.r, s.color.g, s.color.b, 0.35),
            );

            draw_rectangle(
                s.x - 1.0,
                s.y - 1.0,
                s.size + 2.0,
                s.size + 2.0,
                Color::new(s.color.r, s.color.g, s.color.b, 0.12),
            );
        }

        // PARTÍCULAS NA FRENTE DO TÍTULO
        for s in sparks.iter().filter(|s| s.y >= depth_line) {
            draw_rectangle(
                s.x,
                s.y,
                s.size,
                s.size * 1.4,
                s.color,
            );

            draw_rectangle(
                s.x - 1.0,
                s.y - 1.0,
                s.size + 2.0,
                s.size + 2.0,
                Color::new(s.color.r, s.color.g, s.color.b, 0.18),
            );
        }

        let icon_size = 16.0;

        // Ícone de som alterna entre quadros mudo/ativo.
        let src = if muted {
            Rect::new(16.0, 0.0, 16.0, 16.0)
        } else {
            Rect::new(0.0, 0.0, 16.0, 16.0)
        };

        draw_texture_ex(
            &self.speaker,
            W * 0.5 - icon_size / 2.0,
            MENU_START_Y + MENU_GAP * 3.0,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                dest_size: Some(vec2(icon_size, icon_size)),
                ..Default::default()
            },
        );

        let credit = "feito por jakezin";

        let dim = measure_text(credit, Some(font), 8, 1.0);

        draw_text_ex(
            credit,
            W - dim.width - 6.0,
            H - 6.0,
            TextParams {
                font: Some(font),
                font_size: 8,
                color: YELLOW,
                ..Default::default()
            },
        );

        // Sobreposição de desvanecimento de entrada.
        if self.fade > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                W,
                H,
                Color::new(0.0, 0.0, 0.0, self.fade),
            );
        }

    }

    pub fn draw_buttons(&self, font: &Font) -> MenuAction {
        // Retorna a primeira ação clicada, se houver.
        let center_x = W * 0.5;
        let y = MENU_START_Y;

        Self::button(center_x, y, "INICIAR", font, MenuAction::Start)
            .or(Self::button(center_x, y + MENU_GAP, "MUTAR AUDIO", font, MenuAction::Mute))
            .or(Self::button(center_x, y + MENU_GAP * 2.0, "SAIR", font, MenuAction::Quit))
            .unwrap_or(MenuAction::None)
    }

    fn button(
        x: f32,
        y: f32,
        label: &str,
        font: &Font,
        action: MenuAction,
    ) -> Option<MenuAction> {

        // Converte o mouse para a escala interna.
        let (mx, my) = mouse_position();
        let mx = mx * (INTERNAL_WIDTH as f32 / screen_width());
        let my = my * (INTERNAL_HEIGHT as f32 / screen_height());

        let dim = measure_text(label, Some(font), 16, 1.0);

        let bw = dim.width + 40.0;
        let bh = 22.0;

        // Detecção de passagem do mouse no espaço lógico.
        let hovered = Rect::new(x - bw / 2.0, y - bh / 2.0, bw, bh)
            .contains(vec2(mx, my));

        let t = get_time() as f32;

        let pulse = (t * 3.0).sin() * 0.08 + 1.0;
        let scale = if hovered { 1.2 * pulse } else { pulse };

        let glow = (t * 4.0).sin() * 0.5 + 0.5;

        let text_color = if hovered {
            Color::new(0.4 + glow * 0.6, 0.8, 1.0, 1.0)
        } else {
            Color::new(0.6, 0.7, 1.0, 1.0)
        };

        // linha de brilho atrás
        let line_w = bw * scale;
        draw_rectangle(
            x - line_w / 2.0,
            y + 8.0,
            line_w,
            2.0,
            Color::new(0.2, 0.6 + glow * 0.4, 1.0, 0.8),
        );

        // brilho extra quando passa o mouse
        if hovered {
            draw_rectangle(
                x - line_w / 2.0,
                y + 6.0,
                line_w,
                6.0,
                Color::new(0.2, 0.6, 1.0, 0.15),
            );
        }

        draw_text_ex(
            label,
            x - dim.width / 2.0,
            y + dim.height / 2.0,
            TextParams {
                font: Some(font),
                font_size: 16,
                color: text_color,
                ..Default::default()
            },
        );

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            Some(action)
        } else {
            None
        }

    }
}
