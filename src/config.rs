//! Configuration module for Ping-Pong (Rust Edition GUI).

use macroquad::color::Color;

/// Bot Skill levels altering prediction accuracy and tracking velocity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}

/// Global Game Configuration Parameters
pub struct Config;

impl Config {
    /// GUI Window dimensions
    pub const SCREEN_WIDTH: f32 = 800.0;
    pub const SCREEN_HEIGHT: f32 = 600.0;

    /// Playfield grid proportions
    #[allow(dead_code)]
    pub const BOARD_WIDTH: u16 = 80;
    #[allow(dead_code)]
    pub const BOARD_HEIGHT: u16 = 24;

    /// Paddle dimensions
    pub const PADDLE_WIDTH: f32 = 16.0;
    pub const PADDLE_HEIGHT: f32 = 100.0;
    pub const BALL_RADIUS: f32 = 12.0;

    /// Maximum score needed to win match
    pub const MAX_SCORE: u32 = 5;

    // Macroquad Color instances
    pub const COLOR_BACKGROUND: Color = Color::new(0.08, 0.09, 0.12, 1.0); // Dark sleek background
    pub const COLOR_BORDER: Color = Color::new(0.26, 0.52, 0.96, 1.0); // Electric Blue
    pub const COLOR_PLAYER: Color = Color::new(0.20, 0.66, 0.33, 1.0); // Vibrant Green
    pub const COLOR_BOT: Color = Color::new(0.92, 0.26, 0.21, 1.0); // Crimson Red
    pub const COLOR_BALL: Color = Color::new(0.98, 0.74, 0.02, 1.0); // Neon Yellow
    pub const COLOR_NET: Color = Color::new(0.40, 0.45, 0.55, 0.6); // Translucent Net
}
