//! Entry point for Ping-Pong GUI (Rust Edition).

mod ball;
mod config;
mod game;
mod paddle;
mod renderer;

use config::Config;
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ping-Pong (Rust GUI Edition)".to_string(),
        window_width: Config::SCREEN_WIDTH as i32,
        window_height: Config::SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        if !game.update_frame() {
            break;
        }
        next_frame().await;
    }
}
