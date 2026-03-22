use macroquad::audio::*;

pub struct AudioManager {
    theme: Sound,
    click: Sound,
    pub music_muted: bool, // controle global de música
}

impl AudioManager {
    pub async fn new() -> Self {
        // Carrega os recursos de áudio do menu/interface na inicialização.
        let theme = load_sound("audio/seila.wav").await.unwrap();
        let click = load_sound("audio/click.wav").await.unwrap();

        Self {
            theme,
            click,
            music_muted: false,
        }
    }

    // toca a música do menu
    pub fn play_menu_music(&self) {
        if self.music_muted {
            return;
        }

        play_sound(
            &self.theme,
            PlaySoundParams {
                looped: true,
                volume: 0.4,
            },
        );
    }

    pub fn toggle_music(&mut self) {
        // Para ou reinicia o tema em loop.
        self.music_muted = !self.music_muted;

        if self.music_muted {
            stop_sound(&self.theme);
        } else {
            self.play_menu_music();
        }
    }

    pub fn click(&self) {
        play_sound(
            &self.click,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
}
