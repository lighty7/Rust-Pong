//! Configuration module for Google Ping-Pong (Rust Edition).
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. `pub enum`: Strongly typed enumerations in Rust. Enums can derive traits like `Copy`, `Clone`, `PartialEq`, `Debug`.
//! 2. `pub const`: Compile-time constants in Rust require explicit type annotations (`: usize`, `: f64`).
//! 3. `#[derive(...)]`: Macro attribute that automatically implements trait behavior for a type.

use crossterm::style::Color;

/// AI Skill levels altering prediction accuracy and tracking velocity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
}

/// Global Game Configuration Parameters
pub struct Config;

impl Config {
    /// Width of the terminal playing grid
    pub const BOARD_WIDTH: u16 = 80;

    /// Height of the terminal playing grid
    pub const BOARD_HEIGHT: u16 = 24;

    /// Target Frame Rate (60 Frames Per Second)
    pub const TARGET_FPS: u64 = 60;
    pub const FRAME_DURATION_MS: u64 = 1000 / Self::TARGET_FPS;

    /// Paddle height in vertical characters
    pub const PADDLE_HEIGHT: u16 = 5;

    /// Maximum score needed to win match
    pub const MAX_SCORE: u32 = 5;

    // Google Brand Colors (Crossterm RGB Color instances)
    pub const COLOR_BORDER: Color = Color::Rgb { r: 66, g: 133, b: 244 }; // Google Blue
    pub const COLOR_PLAYER: Color = Color::Rgb { r: 52, g: 168, b: 83 };  // Google Green
    pub const COLOR_AI: Color     = Color::Rgb { r: 234, g: 67, b: 53 };  // Google Red
    pub const COLOR_BALL: Color   = Color::Rgb { r: 251, g: 188, b: 5 };  // Google Yellow
}
