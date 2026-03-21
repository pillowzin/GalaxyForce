use macroquad::prelude::*;
use crate::enemy::{Enemy, EnemyKind, visual_por_kind};

fn spawn_enemy(
    kind: EnemyKind,
    normal_texture: Texture2D,
    red_texture: Texture2D,
    miniboss_texture: Texture2D,
    boss_texture: Texture2D) -> Enemy {
    let enemy_size = 16.0;
    let pos = vec2(
        rand::gen_range(10.0, crate::config::INTERNAL_WIDTH as f32 - enemy_size),
        rand::gen_range(-200.0, -20.0));

    let speed = match kind {
        EnemyKind::Normal => rand::gen_range(2.0, 4.0),
        EnemyKind::Red => rand::gen_range(2.5, 4.5),
        EnemyKind::MiniBoss => 1.8,
        EnemyKind::Boss => 1.2,
    };

    let visual = visual_por_kind(
        kind,
        normal_texture,
        red_texture,
        miniboss_texture,
        boss_texture,
    );

    Enemy::new(kind, pos, speed, visual.texture, visual.frames)
}

pub fn inimigos_para_fase(
    stage: u32,
    normal_texture: Texture2D,
    red_texture: Texture2D,
    miniboss_texture: Texture2D,
    boss_texture: Texture2D,
) -> Vec<Enemy> {
    let mut enemies = Vec::new();

    match stage {
        // WAVE 1 — tutorial
        1 => {
            for _ in 0..8 {
                enemies.push(spawn_enemy(
                    EnemyKind::Normal,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 2
        2 => {
            for _ in 0..12 {
                enemies.push(spawn_enemy(
                    EnemyKind::Normal,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 3 — primeiros reds
        3 => {
            for _ in 0..6 {
                enemies.push(spawn_enemy(
                    EnemyKind::Normal,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
            for _ in 0..2 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 4
        4 => {
            for _ in 0..4 {
                enemies.push(spawn_enemy(
                    EnemyKind::Normal,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
            for _ in 0..4 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 5 — pré miniboss
        5 => {
            for _ in 0..2 {
                enemies.push(spawn_enemy(
                    EnemyKind::Normal,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
            for _ in 0..6 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 6 — primeiro miniboss
        6 => {
            for _ in 0..6 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 7 — caos controlado
        7 => {
            for _ in 0..4 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
            for _ in 0..1 {
                enemies.push(spawn_enemy(
                    EnemyKind::MiniBoss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

        // WAVE 8
        8 => {
            for _ in 0..1 {
                enemies.push(spawn_enemy(
                    EnemyKind::MiniBoss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                    ));
            }
            for _ in 0..1 {
                enemies.push(spawn_enemy(
                    EnemyKind::MiniBoss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                    ));
            }
        }

        // WAVE 9 — preparação final
        9 => {
            for _ in 0..10 {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
            for _ in 0..2 {
                enemies.push(spawn_enemy(
                    EnemyKind::MiniBoss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }

       10 => {
                enemies.push(spawn_enemy(
                    EnemyKind::Boss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        // LOOP INFINITO
        _ => {
            let extra = stage - 10;

            for _ in 0..(8 + extra * 2) {
                enemies.push(spawn_enemy(
                    EnemyKind::Red,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }

            for _ in 0..(1 + extra / 2) {
                enemies.push(spawn_enemy(
                    EnemyKind::MiniBoss,
                    normal_texture.clone(),
                    red_texture.clone(),
                    miniboss_texture.clone(),
                    boss_texture.clone(),
                ));
            }
        }
    }
    enemies
}
