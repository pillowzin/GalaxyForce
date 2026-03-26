# Galaxy Force

A fast, arcade-style space shooter built with Rust and Macroquad. Pilot a tiny ship through escalating enemy waves, rack up kills, and survive bosses in a retro CRT-styled presentation.

## Gameplay Overview
- You control the ship with the mouse. The ship follows the cursor and is clamped to the playfield.
- Press `Q` to fire; bullets come from the ship center.
- Each stage spawns a wave of enemies. Clearing all enemies triggers a short transition effect, then the next stage begins.
- Difficulty ramps as stages increase. Minibosses and a boss appear among the waves.
- Enemy collisions and enemy bullets can damage you. HP is shown as hearts; when HP reaches 0 you hit the death screen.
- Your total kills are tracked in the HUD.

## Features (Current)
- Mouse-driven movement with tight, screen-clamped control.
- Stage-based waves with increasing speed and intensity.
- Multiple enemy types with distinct sizes, HP, and behavior.
- Player bullets, boss bullets, hit flashes, explosions, and camera shake.
- Retro presentation: pixel art sprites, CRT post-process, scanline/vignette feel, pixel font UI.
- Menu, pause, and death screens with click-to-select UI.
- Audio: menu music and click/laser/collision SFX.

## Controls
- Move: Mouse
- Shoot: `Q`
- Pause/Resume: `Esc`
- Menu selection: Left mouse button

## How To Run
1. Install Rust (stable) and Cargo.
2. From the project root:

```bash
cargo run
```

The game runs at an internal resolution of 256x350 and scales to a 512x700 window.

## Folder Structure
- `src/`: All Rust source files.
- `src/main.rs`: Game loop, state transitions, render target, CRT pass.
- `src/state_menu.rs`: Title screen and menu UI.
- `src/state_playing.rs`: Combat loop, waves, collisions, HUD updates.
- `src/state_paused.rs`, `src/state_death.rs`: Pause and death screens.
- `src/player.rs`, `src/enemy.rs`, `src/bullet.rs`: Core gameplay entities.
- `src/spawner.rs`: Stage wave definitions.
- `src/crt_shader.rs`: CRT shader code.
- `audio/`: Music and sound effects.
- `fonts/`: Pixel font (Press Start 2P).
- `sprites/`: Pixel art assets (player, enemies, boss, UI icons, explosion frames).

## Credits
- Code + art: Jakezin (in-game credit).
- Font: Press Start 2P (loaded from `fonts/PressStart2P-Regular.ttf`).

## License
All rights reserved.
