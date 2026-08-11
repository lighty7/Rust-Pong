//! Paddle representation and AI tracking engine module.
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. Pattern Matching (`match`): Exhaustive evaluation of `AIDifficulty` enum variants.
//! 2. Method Overloading Alternative: Explicit function names (`move_up`, `move_down`).
//! 3. Floating point clamping: `offset.clamp(-1.0, 1.0)` restricts range to valid boundaries.

use crate::ball::Ball;
use crate::config::BotDifficulty;

#[derive(Debug, Clone)]
pub struct Paddle {
    pub x: f64,
    pub y: f64,
    pub height: u16,
    pub speed: f64,
    pub score: u32,
}

impl Paddle {
    /// Creates a new Paddle at column `x` and row `start_y`.
    pub fn new(x: f64, start_y: f64, height: u16) -> Self {
        Self {
            x,
            y: start_y,
            height,
            speed: 24.0,
            score: 0,
        }
    }

    /// Moves paddle upwards towards terminal row 0.
    pub fn move_up(&mut self, delta_time: f64, min_y: f64) {
        self.y -= self.speed * delta_time;
        let half_h = (self.height as f64) / 2.0;
        if self.y - half_h < min_y {
            self.y = min_y + half_h;
        }
    }

    /// Moves paddle downwards towards maximum height limit.
    pub fn move_down(&mut self, delta_time: f64, max_y: f64) {
        self.y += self.speed * delta_time;
        let half_h = (self.height as f64) / 2.0;
        if self.y + half_h > max_y {
            self.y = max_y - half_h;
        }
    }

    /// Autonomous Bot controller tracking ball location.
    pub fn update_bot(
        &mut self,
        ball: &Ball,
        difficulty: BotDifficulty,
        delta_time: f64,
        min_y: f64,
        max_y: f64,
    ) {
        let target_y = ball.y;

        let bot_speed = match difficulty {
            BotDifficulty::Easy => {
                if ball.dir_x > 0.0 {
                    self.speed * 0.55
                } else {
                    return; // Idle when ball is moving away
                }
            }
            BotDifficulty::Medium => {
                if ball.dir_x > 0.0 {
                    self.speed * 0.80
                } else {
                    self.speed * 0.30
                }
            }
            BotDifficulty::Hard => self.speed * 1.10,
        };

        // Deadzone check to avoid high-frequency jittering
        let deadzone = 0.5;
        if self.y < target_y - deadzone {
            self.y += bot_speed * delta_time;
        } else if self.y > target_y + deadzone {
            self.y -= bot_speed * delta_time;
        }

        // Clamp paddle position inside boundaries
        let half_h = (self.height as f64) / 2.0;
        if self.y - half_h < min_y {
            self.y = min_y + half_h;
        }
        if self.y + half_h > max_y {
            self.y = max_y - half_h;
        }
    }

    /// Evaluates if ball coordinates overlap with paddle bounding area.
    pub fn check_collision(&self, ball_x: f64, ball_y: f64) -> bool {
        let half_h = (self.height as f64) / 2.0;
        let x_match = (ball_x - self.x).abs() <= 1.2;
        let y_match = ball_y >= self.y - half_h - 0.5 && ball_y <= self.y + half_h + 0.5;
        x_match && y_match
    }

    /// Returns normalized hit distance from paddle center (-1.0 to 1.0).
    pub fn get_hit_offset(&self, ball_y: f64) -> f64 {
        let half_h = (self.height as f64) / 2.0;
        let offset = (ball_y - self.y) / half_h;
        offset.clamp(-1.0, 1.0)
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    #[allow(dead_code)]
    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}
