use macroquad::prelude::*;
use crate::config::*;

const W: f32 = INTERNAL_WIDTH as f32;
const H: f32 = INTERNAL_HEIGHT as f32;

pub struct DeathState {
    time: f32,
}

pub enum DeathAction {
    None,
    Restart,
    Menu,
    Quit,
}

impl DeathState {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn draw(&self, font: &Font) -> DeathAction {
        draw_rectangle(0.0, 0.0, W, H, BLACK);

        let center_x = W * 0.5;

        // título
        let title = "YOU DIED";
        let dim = measure_text(title, Some(font), 24, 1.0);

        draw_text_ex(
            title,
            center_x - dim.width / 2.0,
            H * 0.35,
            TextParams {
                font: Some(font),
                font_size: 24,
                color: Color::new(1.0, 0.2, 0.2, 1.0),
                ..Default::default()
            },
        );

        // botões
        let mut y = H * 0.5;

        if let Some(a) = button(center_x, y, "RESTART", font, DeathAction::Restart) {
            return a;
        }

        y += 24.0;

        if let Some(a) = button(center_x, y, "MENU", font, DeathAction::Menu) {
            return a;
        }

        y += 24.0;

        if let Some(a) = button(center_x, y, "QUIT", font, DeathAction::Quit) {
            return a;
        }

        DeathAction::None
    }
}

fn button(
    x: f32,
    y: f32,
    label: &str,
    font: &Font,
    action: DeathAction,
) -> Option<DeathAction> {

    let (mx, my) = mouse_position();
    let mx = mx * (INTERNAL_WIDTH as f32 / screen_width());
    let my = my * (INTERNAL_HEIGHT as f32 / screen_height());

    let dim = measure_text(label, Some(font), 16, 1.0);

    let bw = dim.width + 40.0;
    let bh = 22.0;

    let hovered = Rect::new(x - bw / 2.0, y - bh / 2.0, bw, bh)
        .contains(vec2(mx, my));

    let t = get_time() as f32;
    let glow = (t * 4.0).sin() * 0.5 + 0.5;

    let text_color = if hovered {
        Color::new(1.0, 0.4 + glow * 0.6, 0.4, 1.0)
    } else {
        Color::new(1.0, 0.5, 0.5, 1.0)
    };

    draw_rectangle(
        x - bw / 2.0,
        y + 8.0,
        bw,
        2.0,
        Color::new(1.0, 0.2 + glow * 0.4, 0.2, 0.8),
    );

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