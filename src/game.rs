//! Main Game Engine loop and state machine module for GUI.

use crate::ball::Ball;
use crate::config::{BotDifficulty, Config};
use crate::paddle::Paddle;
use crate::renderer::Renderer;

use macroquad::prelude::*;

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
    bot: Paddle,
    renderer: Renderer,
    difficulty: BotDifficulty,
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
                (Config::SCREEN_WIDTH / 2.0) as f64,
                (Config::SCREEN_HEIGHT / 2.0) as f64,
            ),
            player: Paddle::new(
                30.0,
                (Config::SCREEN_HEIGHT / 2.0) as f64,
                Config::PADDLE_HEIGHT as u16,
            ),
            bot: Paddle::new(
                (Config::SCREEN_WIDTH - 30.0) as f64,
                (Config::SCREEN_HEIGHT / 2.0) as f64,
                Config::PADDLE_HEIGHT as u16,
            ),
            renderer: Renderer::new(Config::SCREEN_WIDTH, Config::SCREEN_HEIGHT),
            difficulty: BotDifficulty::Medium,
            state: GameState::Menu,
            running: true,
            player_won: false,
        }
    }

    /// Single frame update for Macroquad async loop
    pub fn update_frame(&mut self) -> bool {
        let delta_time = get_frame_time().min(0.05) as f64;

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        match self.state {
            GameState::Menu => {
                if let Some(selected_diff) = self.renderer.render_menu() {
                    self.difficulty = selected_diff;
                    self.reset_round();
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.process_input(delta_time);
                self.update(delta_time);
                self.handle_collisions();
                self.check_score();

                self.renderer
                    .render(&self.ball, &self.player, &self.bot, self.difficulty, false);
            }
            GameState::Paused => {
                self.process_input(delta_time);
                self.renderer
                    .render(&self.ball, &self.player, &self.bot, self.difficulty, true);
            }
            GameState::GameOver => {
                if self.renderer.render_game_over(self.player_won) {
                    self.player.score = 0;
                    self.bot.score = 0;
                    self.state = GameState::Menu;
                }
            }
        }

        self.running
    }

    fn process_input(&mut self, delta_time: f64) {
        if is_key_pressed(KeyCode::P) {
            self.state = match self.state {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
                other => other,
            };
        }

        if self.state == GameState::Playing {
            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                self.player.move_up(delta_time, 10.0);
            }
            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.player
                    .move_down(delta_time, (Config::SCREEN_HEIGHT - 10.0) as f64);
            }
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.ball.update(delta_time);
        self.bot.update_bot(
            &self.ball,
            self.difficulty,
            delta_time,
            10.0,
            (Config::SCREEN_HEIGHT - 10.0) as f64,
        );
    }

    fn handle_collisions(&mut self) {
        // Top / Bottom Wall Collisions
        if self.ball.y <= 10.0 + (Config::BALL_RADIUS as f64) {
            self.ball.y = 10.0 + (Config::BALL_RADIUS as f64);
            self.ball.bounce_y();
        } else if self.ball.y >= (Config::SCREEN_HEIGHT - 10.0 - Config::BALL_RADIUS) as f64 {
            self.ball.y = (Config::SCREEN_HEIGHT - 10.0 - Config::BALL_RADIUS) as f64;
            self.ball.bounce_y();
        }

        // Left Player Paddle Collision
        if self.player.check_collision(self.ball.x, self.ball.y) && self.ball.dir_x < 0.0 {
            let offset = self.player.get_hit_offset(self.ball.y);
            self.ball.x = self.player.x + (Config::PADDLE_WIDTH / 2.0 + Config::BALL_RADIUS) as f64;
            self.ball.bounce_x(offset);
        }

        // Right Bot Paddle Collision
        if self.bot.check_collision(self.ball.x, self.ball.y) && self.ball.dir_x > 0.0 {
            let offset = self.bot.get_hit_offset(self.ball.y);
            self.ball.x = self.bot.x - (Config::PADDLE_WIDTH / 2.0 + Config::BALL_RADIUS) as f64;
            self.ball.bounce_x(offset);
        }
    }

    fn check_score(&mut self) {
        // Point Bot
        if self.ball.x < 0.0 {
            self.bot.increment_score();
            self.check_match_over();
        }
        // Point Player
        else if self.ball.x > Config::SCREEN_WIDTH as f64 {
            self.player.increment_score();
            self.check_match_over();
        }
    }

    fn check_match_over(&mut self) {
        if self.player.score >= Config::MAX_SCORE {
            self.player_won = true;
            self.state = GameState::GameOver;
        } else if self.bot.score >= Config::MAX_SCORE {
            self.player_won = false;
            self.state = GameState::GameOver;
        } else {
            self.reset_round();
        }
    }

    fn reset_round(&mut self) {
        self.ball.reset(
            (Config::SCREEN_WIDTH / 2.0) as f64,
            (Config::SCREEN_HEIGHT / 2.0) as f64,
        );
        self.player.y = (Config::SCREEN_HEIGHT / 2.0) as f64;
        self.bot.y = (Config::SCREEN_HEIGHT / 2.0) as f64;
    }
}
