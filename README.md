# 🏓 Ping-Pong (Rust Edition)

[![Rust Version](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org/)
[![Cargo Build](https://img.shields.io/badge/Cargo-Passing-brightgreen.svg)](https://doc.rust-lang.org/cargo/)
[![Docker Container](https://img.shields.io/badge/Docker-Ready-blue)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

An industry-standard, memory-safe, terminal-based **Ping-Pong game written in idiomatic Rust**. Features an autonomous Computer Bot opponent with selectable difficulty levels, frame delta timing (~60 FPS), crossterm terminal manipulation, and ANSI TrueColor retro graphics.

This repository serves as both a production-ready application and an **in-depth Rust tutorial reference** explaining Ownership, Borrowing (`&`, `&mut`), Enums, Pattern Matching, Struct Methods (`impl`), Error Handling (`Result`), and Cargo build workflows.

---

## 📚 Tutorial & Language Reference Guide

### 1. Key Rust Concepts Demonstrated

| Concept | File Location | Description & Syntax Reference |
| :--- | :--- | :--- |
| **Structs & Implementation** | [`ball.rs`](src/ball.rs), [`paddle.rs`](src/paddle.rs) | `pub struct Ball` paired with `impl Ball` blocks for object-like state & methods. |
| **Ownership & Borrowing** | [`paddle.rs`](src/paddle.rs) | Methods taking `&self` (read-only borrow) vs `&mut self` (exclusive mutable borrow). |
| **Enums & Pattern Matching** | [`config.rs`](src/config.rs), [`game.rs`](src/game.rs) | `match difficulty { ... }` handles all variants (`Easy`, `Medium`, `Hard`) safely. |
| **Derive Macros** | [`config.rs`](src/config.rs) | `#[derive(Debug, Clone, Copy, PartialEq)]` generates trait implementations. |
| **Error Handling** | [`main.rs`](src/main.rs), [`game.rs`](src/game.rs) | Return type `Result<(), Box<dyn Error>>` handles terminal initialization failure. |
| **Panic Hook Handler** | [`main.rs`](src/main.rs) | `std::panic::set_hook(...)` guarantees raw terminal state is restored on unexpected crashes. |
| **Cross-Platform Input** | [`renderer.rs`](src/renderer.rs) | `crossterm::event::poll` listens for keypresses without blocking game loop thread. |

---

## 🏗️ Architecture & Module Design

```
                     +--------------------------+
                     |        main.rs           |
                     +------------+-------------+
                                  |
                                  v
                     +--------------------------+
                     |        game.rs           |
                     +----+-------+--------+----+
                          |       |        |
        +-----------------+       |        +-----------------+
        |                         v                          |
+-------+--------+       +----------------+        +---------+------+
|    ball.rs     |       |   paddle.rs    |        |   renderer.rs  |
+----------------+       +----------------+        +----------------+
(Kinematics &    )       (Player & Bot    )        (Crossterm ANSI  )
(Bounce Physics  )       (Tracking Logic  )        (Raw Mode Input  )
```

### Bot Opponent Algorithm
The Bot paddle uses an adaptive tracking heuristic based on the selected difficulty:
1. **Easy**: Delays tracking until the ball moves towards its boundary (`dir_x > 0`) and moves at 55% speed.
2. **Medium**: Moves at 80% speed with center alignment deadzones to prevent jittering.
3. **Hard**: Predicts ball trajectory intercept coordinates at 110% speed with linear trajectory estimation.

---

## 🚀 Getting Started

### Prerequisites

#### Native Build:
- **Rust Toolchain**: Rust 1.70+ and Cargo (installed via `rustup`).

#### Docker Build:
- **Docker Engine** (or Docker Desktop) & `docker-compose`.

---

## 🛠️ Building & Running Natively

```bash
# 1. Clone the repository
git clone https://github.com/your-username/rust-pingpong.git
cd rust-pingpong

# 2. Run automated physics unit tests
cargo test

# 3. Build and launch optimized release binary!
cargo run --release
```

---

## 🐳 Running via Docker (Any Machine)

Docker ensures the game runs on **any Linux, macOS, or Windows host** without needing a local Rust compiler.

### Option A: Using `docker-compose` (Recommended)
```bash
# Build and run interactively
docker-compose run --rm rust-pingpong
```

### Option B: Using `docker` CLI directly
```bash
# Build Docker image
docker build -t rust-pingpong .

# Run container interactively with TTY allocated
docker run -it --rm rust-pingpong
```

---

## 🎮 Game Controls

| Key | Action |
| :---: | :--- |
| `W` | Move Player Paddle **Up** |
| `S` | Move Player Paddle **Down** |
| `P` | **Pause / Resume** Game |
| `Q` | **Quit** Game |

---

## 🧪 Testing

The codebase includes automated unit tests covering collision detection, angle reflections, speed increments, and score management:

```bash
cargo test
```

---

## 📄 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.
