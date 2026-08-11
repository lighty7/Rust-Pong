//! Entry point for Ping-Pong (Rust Edition).
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. Module System (`mod` keyword): Declares submodule tree structure.
//! 2. Panic Hooks (`std::panic::set_hook`): Custom handler restoring raw terminal state
//!    if an unhandled panic occurs, preventing terminal corruption.

mod ball;
mod config;
mod game;
mod paddle;
mod renderer;

use game::Game;
use renderer::Renderer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Set custom panic hook to restore terminal raw mode safely on crash
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Renderer::disable_raw_mode();
        default_panic(info);
    }));

    let mut game = Game::new();
    game.run()?;

    Ok(())
}
