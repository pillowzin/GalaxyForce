use macroquad::prelude::*;
use macroquad::audio::*;
use std::rc::Rc;
use std::collections::VecDeque;

use crate::player::Player;
use crate::enemy::{Enemy, EnemyKind};
use crate::bullet::Bullet;
use crate::collision::aabb;
use crate::spawner::inimigos_para_fase;
use crate::hud;
use crate::explosion::Explosion;
use crate::enemy_bullet::EnemyBullet;

struct StageParticle {
    angle: f32,
    radius: f32,
    speed: f32,
    color_offset: f32,
}

pub struct PlayingState {
    pub stage: u32,
    pub enemies: Vec<Enemy>,
    pending_enemies: VecDeque<Enemy>,
    pub bullets: Vec<Bullet>,
    pub explosions: Vec<Explosion>,
    pub enemy_bullets: Vec<EnemyBullet>,
    pub heart_texture: Texture2D,
    pub skull_texture: Texture2D,
    pub kills: u32,

    stage_particles: Vec<StageParticle>,

    shoot_timer: f32,          // recarga dos tiros do jogador
    spawn_timer: f32,          // temporizador de spawn gradual
    spawn_interval: f32,       // intervalo atual entre spawns
    waiting_next_stage: bool,  // flag de transição após limpar a fase
    stage_timer: f32,          // tempo decorrido durante a transição

    normal_enemy_texture: Texture2D,
    red_enemy_texture: Texture2D,
    miniboss_texture: Texture2D,
    boss_texture: Texture2D,

    explosion_texture: Texture2D,
    explosion_frames: Rc<Vec<Rect>>,

    laser_sound: Sound,
    collide_sound: Sound,
}

pub struct GameAssets {
    pub normal_enemy: Texture2D,
    pub red_enemy: Texture2D,
    pub miniboss: Texture2D,
    pub boss: Texture2D,
    pub explosion: Texture2D,
    pub explosion_frames: Rc<Vec<Rect>>,
    pub heart: Texture2D,
    pub skull: Texture2D,
}

const SHOOT_DELAY: f32 = 0.28;
const NEXT_STAGE_DURATION: f32 = 1.8;

fn spawn_interval_for_stage(stage: u32) -> f32 {
    // Intervalo menor em fases avançadas, sem picos bruscos.
    let base = 0.08;
    let min = 0.05;
    let step = 0.03;
    let s = stage.saturating_sub(1) as f32;
    (base - s * step).max(min)
}

impl PlayingState {
    pub async fn new(assets: GameAssets) -> Self {
        // Áudio para laser e colisão.
        let laser_sound = load_sound("audio/laser.wav").await.unwrap();
        let collide_sound = load_sound("audio/collide.wav").await.unwrap();

        let stage = 1;

        // Cria a primeir))a onda (spawn gradual).
        let pending = inimigos_para_fase(
            stage,
            assets.normal_enemy.clone(),
            assets.red_enemy.clone(),
            assets.miniboss.clone(),
            assets.boss.clone(),
        );

        Self {
            stage,
            enemies: Vec::new(),
            pending_enemies: VecDeque::from(pending),
            bullets: Vec::new(),
            explosions: Vec::new(),
            enemy_bullets: Vec::new(),

            shoot_timer: 0.0,
            spawn_timer: 0.0,
            spawn_interval: spawn_interval_for_stage(stage),
            waiting_next_stage: false,
            stage_timer: 0.0,

            normal_enemy_texture: assets.normal_enemy,
            red_enemy_texture: assets.red_enemy,
            miniboss_texture: assets.miniboss,
            boss_texture: assets.boss,
            laser_sound,
            collide_sound,
            explosion_texture: assets.explosion,
            explosion_frames: assets.explosion_frames,
            heart_texture: assets.heart,
            skull_texture: assets.skull,
            kills: 0,
            // Partículas para a espiral de "próxima fase".
            stage_particles: (0..30).map(|i| StageParticle {
                angle: i as f32 * 0.4,
                radius: 5.0 + (i as f32 * 0.2),
                speed: 1.5 + rand::gen_range(0.5, 2.0),
                color_offset: i as f32,
            }).collect(),
        }
    }

    fn update_spawning(&mut self, dt: f32) {
        if self.pending_enemies.is_empty() {
            return;
        }

        self.spawn_timer += dt;
        let interval = self.spawn_interval;

        while self.spawn_timer >= interval && !self.pending_enemies.is_empty() {
            self.spawn_timer -= interval;
            if let Some(enemy) = self.pending_enemies.pop_front() {
                self.enemies.push(enemy);
            }
        }
    }

    fn update_stage_particles(&mut self, dt: f32) {
        // Avanço angular simples para a espiral.
        for p in self.stage_particles.iter_mut() {
            p.angle += p.speed * dt;
        }
    }

    fn draw_stage_particle(&self, p: &StageParticle, center: Vec2, time: f32) {
        let fade = if time > NEXT_STAGE_DURATION - 0.6 {
            (NEXT_STAGE_DURATION - time) / 0.6
        } else {
            1.0
        };

        let alpha = fade.clamp(0.0, 1.0);

        let angle = p.angle + time * 0.2;

        let pulse = (time * 3.0 + p.color_offset).sin();

        let spiral = 8.0 + p.radius * 0.15 + pulse * 6.0;

        let x = center.x + angle.cos() * spiral;
        let y = center.y + angle.sin() * spiral * 0.85;

        let r = (time * 2.0 + p.color_offset).sin() * 0.5 + 0.5;
        let g = (time * 1.5 + p.color_offset * 0.7).sin() * 0.5 + 0.5;
        let b = (time * 1.2 + p.color_offset * 1.3).sin() * 0.5 + 0.5;

        draw_rectangle(
            x,
            y,
            2.0,
            2.0,
            Color::new(r * 1.4, g * 1.4, b * 1.4, alpha),
        );
    }

    fn handle_player_shoot(&mut self, player: &Player) {
        if is_key_down(KeyCode::Q) && self.shoot_timer >= SHOOT_DELAY {

            let origin = vec2(
                player.hitbox().x + player.hitbox().w / 2.0,
                player.hitbox().y,
            );

            self.bullets.push(Bullet::new(origin));

            // Reseta a recarga quando um tiro é criado.
            self.shoot_timer = 0.0;

            play_sound(
                &self.laser_sound,
                PlaySoundParams {
                    volume: 0.4,
                    looped: false,
                },
            );
        }
    }

    fn update_enemies(&mut self, player: &Player, dt: f32) {

        // Leve aumento de dificuldade por fase.
        let speed_mult = 1.0 + (self.stage as f32 - 1.0) * 0.07;

        let player_center_x =
            player.hitbox().x + player.hitbox().w / 2.0;

        // Atualização de movimento por inimigo.
        for enemy in self.enemies.iter_mut() {
            enemy.update_with_speed_mult(dt, speed_mult, player_center_x);
        }

        // Chefe atira balas ricocheteando ocasionalmente.
        for enemy in self.enemies.iter_mut() {
            if enemy.kind == EnemyKind::Boss
                && rand::gen_range(0.0, 1.0) < 0.02
            {
                let boss_center = enemy.pos + vec2(
                    enemy.hitbox().w / 2.0,
                    enemy.hitbox().h / 2.0,
                );

                let dir = vec2(
                    rand::gen_range(-1.0, 1.0),
                    rand::gen_range(0.2, 1.0),
                ).normalize_or_zero();

                self.enemy_bullets.push(
                    EnemyBullet::new(boss_center, dir * 5.0)
                );
            }
        }
    }

    fn update_enemy_bullets(&mut self, dt: f32) {

        for bullet in self.enemy_bullets.iter_mut() {
            bullet.update(dt);
        }

        self.enemy_bullets.retain(|b| !b.is_dead());
    }

    fn handle_player_enemy_collision(&mut self, player: &mut Player) {

        for enemy in self.enemies.iter_mut() {

            if aabb(player.hitbox(), enemy.hitbox()) {
                // Colisão machuca o jogador; inimigos pequenos morrem no impacto.
                match enemy.kind {
                    EnemyKind::Normal | EnemyKind::Red => {
                        enemy.hp = 0;
                        player.hit(enemy.pos);
                        if player.damage() {
                        }
                    }

                    EnemyKind::MiniBoss | EnemyKind::Boss => {
                        player.hit(enemy.pos);
                        if player.damage() {
                        }
                    }
                }

                play_sound(
                    &self.collide_sound,
                    PlaySoundParams {
                        volume: 0.6,
                        looped: false,
                    },
                );
            }
        }
    }

    fn update_player_bullets(&mut self, dt: f32) {

        for bullet in self.bullets.iter_mut() {
            bullet.update(dt);
        }

        self.bullets.retain(|b| !b.offscreen());
    }

    fn update_explosions(&mut self, dt: f32) {

        for explosion in self.explosions.iter_mut() {
            explosion.update(dt);
        }

        self.explosions.retain(|e| !e.is_finished());
    }

    fn handle_player_bullet_enemy_collision(&mut self) {
        let mut remove_bullet = vec![false; self.bullets.len()];

        for (bi, bullet) in self.bullets.iter().enumerate() {

            for enemy in self.enemies.iter_mut() {

                if aabb(bullet.hitbox(), enemy.hitbox()) {

                    enemy.hp -= 1;

                    if enemy.hp == 0 {
                        self.kills += 1;
                    }

                    remove_bullet[bi] = true;

                    let center = vec2(
                        enemy.hitbox().x + enemy.hitbox().w / 2.0,
                        enemy.hitbox().y + enemy.hitbox().h / 2.0,
                    );

                    // Cria uma explosão única no centro do inimigo.
                    self.explosions.push(
                        Explosion::new(
                            center - vec2(32.0, 32.0),
                            self.explosion_texture.clone(),
                            self.explosion_frames.clone(),
                        )
                    );

                    play_sound(
                        &self.collide_sound,
                        PlaySoundParams {
                            volume: 0.6,
                            looped: false,
                        },
                    );

                    break;
                }
            }
        }

        // Remove tiros que colidiram neste quadro.
        let mut i = 0;
        self.bullets.retain(|_| {
            let keep = !remove_bullet[i];
            i += 1;
            keep
        });
    }

    fn handle_enemy_bullet_player_collision(&mut self, player: &mut Player) {

        for bullet in self.enemy_bullets.iter_mut() {

            if aabb(bullet.hitbox(), player.hitbox()) {

                if player.damage() {
                    println!("PLAYER DEAD");
                }

                // Mata o tiro após acertar.
                bullet.bounces_left = 0;

                play_sound(
                    &self.collide_sound,
                    PlaySoundParams {
                        volume: 0.6,
                        looped: false,
                    },
                );
            }
        }
    }

    fn cleanup_dead_entities(&mut self) {
        // Remove inimigos com 0 HP.
        self.enemies.retain(|e| e.hp > 0);
    }

    fn handle_stage_transition(&mut self, dt: f32) {

        if self.enemies.is_empty() && self.pending_enemies.is_empty() && !self.waiting_next_stage {

            // Inicia transição quando a onda é limpa.
            self.waiting_next_stage = true;
            self.stage_timer = 0.0;
        }

        if self.waiting_next_stage {

            self.stage_timer += dt;
            self.update_stage_particles(dt);

            if self.stage_timer >= NEXT_STAGE_DURATION {

                // Avança para a próxima onda.
                self.waiting_next_stage = false;
                if self.stage < 10 {
                    self.stage += 1;
                } else {
                    println!("acabou!");
                }

                let pending = inimigos_para_fase(
                    self.stage,
                    self.normal_enemy_texture.clone(),
                    self.red_enemy_texture.clone(),
                    self.miniboss_texture.clone(),
                    self.boss_texture.clone(),
                );
                self.enemies.clear();
                self.pending_enemies = VecDeque::from(pending);
                self.spawn_interval = spawn_interval_for_stage(self.stage);
                self.spawn_timer = 0.0;
            }
        }
    }

    pub fn update(&mut self, player: &mut Player, dt: f32) {
        // Ordem principal de atualização.
        self.shoot_timer += dt;

        self.handle_player_shoot(player);
        self.update_spawning(dt);
        self.update_enemies(player, dt);
        self.update_enemy_bullets(dt);

        self.handle_player_enemy_collision(player);

        self.update_player_bullets(dt);
        self.update_explosions(dt);

        self.handle_player_bullet_enemy_collision();
        self.handle_enemy_bullet_player_collision(player);

        self.cleanup_dead_entities();
        self.handle_stage_transition(dt);
    }

    pub fn draw(&self, player: &Player, font: &Font) {
        if self.waiting_next_stage {
            // Desenha partículas atrás da nave antes.
            let center = vec2(
                player.hitbox().x + player.hitbox().w / 2.0,
                player.hitbox().y + player.hitbox().h * 0.7,
            );

            let time = self.stage_timer;

            for p in &self.stage_particles {
                if (p.angle + time * 0.2).sin() < 0.0 {
                    self.draw_stage_particle(p, center, time);
                }
            }
        }
        player.draw();

        if self.waiting_next_stage {
            // Desenha partículas em primeiro plano sobre a nave.
            let center = vec2(
                player.hitbox().x + player.hitbox().w / 2.0,
                player.hitbox().y + player.hitbox().h * 0.7,
            );

            let time = self.stage_timer;

            for p in &self.stage_particles {
                if (p.angle + time * 0.2).sin() >= 0.0 {
                    self.draw_stage_particle(p, center, time);
                }
            }
        }

        for enemy in self.enemies.iter() {
            enemy.draw();
        }

        for bullet in self.bullets.iter() {
            bullet.draw();
        }

        for bullet in self.enemy_bullets.iter() {
            bullet.draw();
        }

        for explosion in self.explosions.iter() {
            explosion.draw();
        }

        if self.waiting_next_stage {
            // Espiral de "próxima fase" + texto do banner.
            let center = vec2(
                player.hitbox().x + player.hitbox().w / 2.0,
                player.hitbox().y + player.hitbox().h / 2.0,
            );

            let time = self.stage_timer;

            for p in self.stage_particles.iter() {
                let t = self.stage_timer;

                let fade = if t > NEXT_STAGE_DURATION - 0.6 {
                    (NEXT_STAGE_DURATION - t) / 0.6
                } else {
                    1.0
                };

                let alpha = fade.clamp(0.0, 1.0);
                let angle = p.angle + time * 0.2;
                let pulse = (time * 3.0 + p.color_offset).sin();
                let spiral = 8.0 + p.radius * 0.15 + pulse * 6.0;

                let x = center.x + angle.cos() * spiral;
                let y = center.y + angle.sin() * spiral * 0.85;

                let r = (time * 2.0 + p.color_offset).sin() * 0.5 + 0.5;
                let g = (time * 1.5 + p.color_offset * 0.7).sin() * 0.5 + 0.5;
                let b = (time * 1.2 + p.color_offset * 1.3).sin() * 0.5 + 0.5;

                let size = 2.0;

                let brightness = 1.4;

                draw_rectangle(
                    x,
                    y,
                    size,
                    size,
                    Color::new(r * brightness, g * brightness, b * brightness, alpha),
                );

            }
            let t = self.stage_timer;

            let alpha = if t < 0.5 {
                t / 0.5
            } else if t > NEXT_STAGE_DURATION - 0.5 {
                (NEXT_STAGE_DURATION - t) / 0.5
            } else {
                1.0
            };

            let text = "PRÓXIMA FASE!";
            let font_size = 10.0;
            let text_dim = measure_text(text, None, font_size as u16, 1.0);

            let player_center_x =
                player.hitbox().x + player.hitbox().w / 2.0;

            let x = player_center_x - text_dim.width / 2.0 - 36.0;
            let y = player.hitbox().y - 20.0;

            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: font_size as u16,
                    color: Color::new(0.92, 0.6, 0.25, alpha),
                    ..Default::default()
                },
            );
        }
    }

    pub fn draw_hud(&self, player: &Player) {
        hud::draw(
            player,
            &self.heart_texture,
            &self.skull_texture,
            self.kills,
        );
    }
}
