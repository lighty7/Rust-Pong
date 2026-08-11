//! Main Game Engine loop and state machine module.
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. `std::time::Instant`: High resolution clock for precise frame timing (`dt`).
//! 2. Error handling with `Result<(), Box<dyn std::error::Error>>`: Idiomatic Rust error propagation.

use crate::ball::Ball;
use crate::config::{AIDifficulty, Config};
use crate::paddle::Paddle;
use crate::renderer::Renderer;

use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    ball: Ball,
    player: Paddle,
    ai: Paddle,
    renderer: Renderer,
    difficulty: AIDifficulty,
    state: GameState,
    running: bool,
    player_won: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(
                (Config::BOARD_WIDTH / 2) as f64,
                (Config::BOARD_HEIGHT / 2) as f64,
            ),
            player: Paddle::new(
                2.0,
                (Config::BOARD_HEIGHT / 2) as f64,
                Config::PADDLE_HEIGHT,
            ),
            ai: Paddle::new(
                (Config::BOARD_WIDTH - 3) as f64,
                (Config::BOARD_HEIGHT / 2) as f64,
                Config::PADDLE_HEIGHT,
            ),
            renderer: Renderer::new(Config::BOARD_WIDTH, Config::BOARD_HEIGHT),
            difficulty: AIDifficulty::Medium,
            state: GameState::Menu,
            running: true,
            player_won: false,
        }
    }

    /// Primary execution loop.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.difficulty = self.renderer.render_menu()?;
        self.state = GameState::Playing;

        self.renderer.clear_screen()?;

        let mut last_time = Instant::now();

        while self.running && self.state != GameState::GameOver {
            let now = Instant::now();
            let mut delta_time = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            // Clamp delta time to avoid large physics steps
            if delta_time > 0.05 {
                delta_time = 0.05;
            }

            self.process_input()?;

            if self.state == GameState::Playing {
                self.update(delta_time);
                self.handle_collisions();
                self.check_score();
            }

            self.renderer.render(
                &self.ball,
                &self.player,
                &self.ai,
                self.difficulty,
                self.state == GameState::Paused,
            )?;

            // Cap at 60 FPS (~16.6ms per frame)
            let frame_duration = now.elapsed();
            let target_duration = Duration::from_millis(Config::FRAME_DURATION_MS);
            if frame_duration < target_duration {
                thread::sleep(target_duration - frame_duration);
            }
        }

        Renderer::disable_raw_mode()?;
        self.renderer.render_game_over(self.player_won)?;

        Ok(())
    }

    fn process_input(&mut self) -> Result<(), Box<dyn Error>> {
        if let Ok(Some(key)) = Renderer::poll_key(Duration::from_millis(1)) {
            match key {
                'q' | 'Q' => self.running = false,
                'p' | 'P' => {
                    self.state = match self.state {
                        GameState::Playing => GameState::Paused,
                        GameState::Paused => GameState::Playing,
                        other => other,
                    };
                }
                'w' | 'W' if self.state == GameState::Playing => {
                    self.player.move_up(0.05, 0.0);
                }
                's' | 'S' if self.state == GameState::Playing => {
                    self.player.move_down(0.05, Config::BOARD_HEIGHT as f64);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn update(&mut self, delta_time: f64) {
        self.ball.update(delta_time);
        self.ai.update_ai(
            &self.ball,
            self.difficulty,
            delta_time,
            0.0,
            Config::BOARD_HEIGHT as f64,
        );
    }

    fn handle_collisions(&mut self) {
        // Top / Bottom Wall Collisions
        if self.ball.y <= 0.0 {
            self.ball.y = 0.0;
            self.ball.bounce_y();
        } else if self.ball.y >= (Config::BOARD_HEIGHT - 1) as f64 {
            self.ball.y = (Config::BOARD_HEIGHT - 1) as f64;
            self.ball.bounce_y();
        }

        // Left Player Paddle Collision
        if self.player.check_collision(self.ball.x, self.ball.y) && self.ball.dir_x < 0.0 {
            let offset = self.player.get_hit_offset(self.ball.y);
            self.ball.x = self.player.x + 1.0;
            self.ball.bounce_x(offset);
        }

        // Right AI Paddle Collision
        if self.ai.check_collision(self.ball.x, self.ball.y) && self.ball.dir_x > 0.0 {
            let offset = self.ai.get_hit_offset(self.ball.y);
            self.ball.x = self.ai.x - 1.0;
            self.ball.bounce_x(offset);
        }
    }

    fn check_score(&mut self) {
        // Point AI
        if self.ball.x < 0.0 {
            self.ai.increment_score();
            self.reset_round();
        }
        // Point Player
        else if self.ball.x > Config::BOARD_WIDTH as f64 {
            self.player.increment_score();
            self.reset_round();
        }

        // Match Winner Check
        if self.player.score >= Config::MAX_SCORE {
            self.player_won = true;
            self.state = GameState::GameOver;
        } else if self.ai.score >= Config::MAX_SCORE {
            self.player_won = false;
            self.state = GameState::GameOver;
        }
    }

    fn reset_round(&mut self) {
        self.ball.reset(
            (Config::BOARD_WIDTH / 2) as f64,
            (Config::BOARD_HEIGHT / 2) as f64,
        );
        self.player.y = (Config::BOARD_HEIGHT / 2) as f64;
        self.ai.y = (Config::BOARD_HEIGHT / 2) as f64;
        thread::sleep(Duration::from_millis(500));
    }
}
