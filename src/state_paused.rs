use macroquad::prelude::*;
use crate::config::*;
use crate::config::mouse_internal;

pub struct PausedState {
    pub speaker: Texture2D,
}

pub enum PauseAction {
    None,
    Resume,
    Menu,
    Quit,
    ToggleSound,
}

impl PausedState {
    pub fn new(speaker: Texture2D) -> Self {
        Self { speaker }
    }

	pub fn draw(
	    &self,
	    font: &Font,
	    muted: bool,
	) -> PauseAction {

	    const W: f32 = INTERNAL_WIDTH as f32;
	    const H: f32 = INTERNAL_HEIGHT as f32;

        // Sobreposição escurecida.
	    draw_rectangle(
	        0.0,
	        0.0,
	        W,
	        H,
	        Color::new(0.02, 0.02, 0.05, 0.85),
	    );

	    let center_x = W * 0.5;
	    let y = H * 0.40;

	    let title = "PAUSED";

	    let dim = measure_text(title, Some(font), 16, 1.0);

	    draw_text_ex(
	        title,
	        center_x - dim.width / 2.0,
	        y,
	        TextParams {
	            font: Some(font),
	            font_size: 16,
	            color: YELLOW,
	            ..Default::default()
	        },
	    );

		let center_x = W * 0.5;
		let mut y = H * 0.45;

        // Botões retornam assim que clicados.
		if let Some(a) = button(center_x, y, "RESUME", font, PauseAction::Resume) {
		    return a;
		}

		y += 24.0;

		if let Some(a) = button(center_x, y, "MENU", font, PauseAction::Menu) {
		    return a;
		}

		y += 24.0;

		if let Some(a) = button(center_x, y, "QUIT", font, PauseAction::Quit) {
		    return a;
		}

	    // speaker embaixo
	    let icon_size = 16.0;
	    let speaker_y = H - 32.0;

        // Seleção do sprite do ícone de áudio.
	    let src = if muted {
	        Rect::new(16.0, 0.0, 16.0, 16.0)
	    } else {
	        Rect::new(0.0, 0.0, 16.0, 16.0)
	    };

	    draw_texture_ex(
	        &self.speaker,
	        center_x - icon_size / 2.0,
	        speaker_y,
	        WHITE,
	        DrawTextureParams {
	            source: Some(src),
	            dest_size: Some(vec2(icon_size, icon_size)),
	            ..Default::default()
	        },
	    );

        // Alterna o som ao clicar no ícone.
	    let mouse = mouse_internal();
	    let rect = Rect::new(center_x - 8.0, speaker_y, 16.0, 16.0);

	    if rect.contains(mouse) && is_mouse_button_pressed(MouseButton::Left) {
	        return PauseAction::ToggleSound;
	    }

	    PauseAction::None
	}
}
fn button(
    x: f32,
    y: f32,
    label: &str,
    font: &Font,
    action: PauseAction,
) -> Option<PauseAction> {

    // Converte o mouse para a escala interna.
    let (mx, my) = mouse_position();
    let mx = mx * (INTERNAL_WIDTH as f32 / screen_width());
    let my = my * (INTERNAL_HEIGHT as f32 / screen_height());

    let dim = measure_text(label, Some(font), 16, 1.0);

    let bw = dim.width + 40.0;
    let bh = 22.0;

    // Detecção de passagem do mouse na área do rótulo.
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

    let line_w = bw * scale;

    draw_rectangle(
        x - line_w / 2.0,
        y + 8.0,
        line_w,
        2.0,
        Color::new(0.2, 0.6 + glow * 0.4, 1.0, 0.8),
    );

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
