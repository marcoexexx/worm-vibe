# WORMZONE

A slither.io-inspired multiplayer snake game built with **Rust** and **Bevy 0.15**.

Grow your worm by eating food, eliminate other worms by making them crash into your body, and become the king of the arena.

## Play Now

**[Play in Browser](https://marcoexexx.github.io/worm-vibe/)** (WASM, no install needed)

## Screenshots

> *Coming soon*

## Features

- 8 food types (donut, cherry, banana, apple, grape, watermelon, strawberry, cookie)
- AI worms with 5 difficulty levels (Noob, Easy, Normal, Hard, Ruthless)
- Speed boost with energy drain
- King crown indicator for the longest worm
- Leaderboard with top 10 ranking
- Minimap with king direction arrow
- Sound effects and visual particles
- Settings menu (controls, sound, zoom)
- 3 control modes: keyboard (WASD), mouse (cursor follow), touch (joystick)
- Mobile responsive with virtual joystick and boost button

## Controls

| Action | Keyboard | Mouse | Touch |
|--------|----------|-------|-------|
| Move | WASD / Arrow keys | Cursor position | Joystick |
| Boost | Space / Shift | Right click | Boost button |
| Pause | ESC | ESC | - |

## Run Locally

### Desktop (recommended)

```bash
cargo build --release
./target/release/game
```

### Browser (WASM)

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

# Build & serve
./build-wasm.sh
python3 -m http.server -d web 8080
# Open http://localhost:8080
```

## Deploy

```bash
./deploy-gh-pages.sh
```

Deploys to GitHub Pages at `https://<user>.github.io/<repo>/`.

## Architecture

8-crate Domain-Driven Design:

```
crates/
  domain/     Pure game logic (worm, food, settings, config)
  physics/    Spatial hash grid, collision detection
  ai/         NPC brain with difficulty scaling
  app/        Game loop orchestration
  infra/      Score persistence (filesystem / WASM stub)
  input/      Keyboard, mouse, touch input
  rendering/  Bevy sprites, HUD, menus, effects
  game/       Binary entry point, state machine, system wiring
```

## Tech Stack

- **Rust** — safe, fast, compiled
- **Bevy 0.15** — ECS game engine
- **WASM** — browser deployment via wasm-bindgen

## License

[MIT](LICENSE) &copy; 2026 Aung Koko Lwin
