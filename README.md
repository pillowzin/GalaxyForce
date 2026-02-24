# Galaxy Forces 🚀

Galaxy Forces é um jogo arcade 2D no estilo **shoot ’em up**, desenvolvido em **Rust** utilizando a biblioteca **Macroquad**.

O objetivo é simples: sobreviver às waves de inimigos, derrotar minibosses e encarar o boss final — com dificuldade escalando indefinidamente.

---

## 🎮 Gameplay

- Controle da nave via **mouse**
- Disparo contínuo
- Waves progressivas com aumento de dificuldade
- Sistema de inimigos com comportamentos distintos
- Boss com movimento próprio e tiros
- Loop infinito após o boss

---

## 👾 Tipos de inimigos

- **Normal** — frágil, rápido
- **Red** — mais resistente
- **MiniBoss** — perseguição gradual ao player
- **Boss** — grande, resistente, movimento pesado e tiros

---

## ✨ Features

- Sistema de estados (Menu / Playing)
- Spawner de inimigos por fase
- Colisão AABB
- Animações via sprite sheets
- Explosões animadas
- Camera shake e feedback visual ao dano
- Dificuldade escalável por fase
- Fundo animado com estrelas

---

## 🛠️ Tecnologias

- **Rust**
- **Macroquad**
- Sprite sheets para animações
- Fonte pixel-art (Press Start 2P)

---

## ▶️ Como rodar o projeto

1. Tenha o Rust instalado
2. Clone o repositório
3. Execute, na raiz do projeto:

```bash
cargo run
```
### 📂 Estrutura do projeto
src/
 ├── main.rs            # Loop principal e estados
 ├── state_menu.rs      # Menu inicial
 ├── state_playing.rs   # Gameplay
 ├── player.rs          # Player
 ├── enemy.rs           # Inimigos e comportamentos
 ├── spawner.rs         # Waves e progressão
 ├── bullet.rs          # Tiros do player
 ├── enemy_bullet.rs    # Tiros dos inimigos
 ├── explosion.rs       # Explosões animadas
 ├── animation.rs       # Sistema de animação
 ├── collision.rs       # Colisão AABB
 └── star.rs            # Fundo animado

### 📌 Observações

Este projeto foi feito com foco em aprendizado, estrutura e feeling arcade.
Sugestões e melhorias são bem-vindas.
