//! Terminal Renderer using Crossterm for ANSI colors and raw keyboard input.
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. `crossterm::terminal::enable_raw_mode()`: Puts terminal into raw mode (disabling canonical buffering).
//! 2. `crossterm::event::poll()`: Non-blocking check for keyboard input events.
//! 3. `execute!` / `queue!`: Macro syntax for sending VT100 / ANSI escape sequences to stdout.

use crate::ball::Ball;
use crate::config::{AIDifficulty, Config};
use crate::paddle::Paddle;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

pub struct Renderer {
    pub width: u16,
    pub height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Enables raw terminal input mode and hides cursor.
    pub fn enable_raw_mode() -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), Hide)?;
        Ok(())
    }

    /// Restores default terminal state and shows cursor.
    pub fn disable_raw_mode() -> io::Result<()> {
        execute!(io::stdout(), Show, ResetColor)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Polls for keyboard events non-blockingly within `timeout`.
    pub fn poll_key(timeout: Duration) -> io::Result<Option<char>> {
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => return Ok(Some(c)),
                        KeyCode::Esc => return Ok(Some('q')),
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }

    /// Renders game state to terminal using Crossterm double-buffered queue macros.
    pub fn render(
        &self,
        ball: &Ball,
        player: &Paddle,
        ai: &Paddle,
        diff: AIDifficulty,
        paused: bool,
    ) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Move to origin (0,0) instead of clear screen to eliminate flicker
        execute!(stdout, MoveTo(0, 0))?;

        // Title Header
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        writeln!(stdout, "  === GOOGLE PING-PONG (RUST EDITION) ===  ")?;

        let diff_str = match diff {
            AIDifficulty::Easy => "EASY",
            AIDifficulty::Medium => "MEDIUM",
            AIDifficulty::Hard => "HARD",
        };

        // Scoreboard
        execute!(stdout, SetForegroundColor(Color::White))?;
        writeln!(
            stdout,
            "  [ PLAYER (W/S) : {} ]    AI ({}) : {}    [ Q:Quit  P:Pause ]",
            player.score, diff_str, ai.score
        )?;

        // Top Border
        execute!(stdout, SetForegroundColor(Config::COLOR_BORDER))?;
        write!(stdout, "+")?;
        for _ in 0..self.width {
            write!(stdout, "-")?;
        }
        writeln!(stdout, "+")?;

        let ball_x = ball.x.round() as i32;
        let ball_y = ball.y.round() as i32;

        let player_x = player.x.round() as i32;
        let player_y = player.y.round() as i32;
        let player_half_h = (player.height / 2) as i32;

        let ai_x = ai.x.round() as i32;
        let ai_y = ai.y.round() as i32;
        let ai_half_h = (ai.height / 2) as i32;

        for y in 0..self.height as i32 {
            execute!(stdout, SetForegroundColor(Config::COLOR_BORDER))?;
            write!(stdout, "|")?;

            for x in 0..self.width as i32 {
                if x == ball_x && y == ball_y {
                    execute!(stdout, SetForegroundColor(Config::COLOR_BALL))?;
                    write!(stdout, "O")?;
                } else if x == player_x
                    && (y >= player_y - player_half_h && y <= player_y + player_half_h)
                {
                    execute!(stdout, SetForegroundColor(Config::COLOR_PLAYER))?;
                    write!(stdout, "#")?;
                } else if x == ai_x && (y >= ai_y - ai_half_h && y <= ai_y + ai_half_h) {
                    execute!(stdout, SetForegroundColor(Config::COLOR_AI))?;
                    write!(stdout, "#")?;
                } else if x == (self.width / 2) as i32 {
                    execute!(stdout, SetForegroundColor(Config::COLOR_BORDER))?;
                    write!(stdout, ":")?;
                } else {
                    write!(stdout, " ")?;
                }
            }

            execute!(stdout, SetForegroundColor(Config::COLOR_BORDER))?;
            writeln!(stdout, "|")?;
        }

        // Bottom Border
        execute!(stdout, SetForegroundColor(Config::COLOR_BORDER))?;
        write!(stdout, "+")?;
        for _ in 0..self.width {
            write!(stdout, "-")?;
        }
        writeln!(stdout, "+")?;

        if paused {
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            writeln!(
                stdout,
                "              *** GAME PAUSED - Press P to Resume ***              "
            )?;
        } else {
            writeln!(
                stdout,
                "                                                                   "
            )?;
        }

        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        Ok(())
    }

    /// Renders Difficulty Selection Menu.
    pub fn render_menu(&self) -> io::Result<AIDifficulty> {
        self.clear_screen()?;
        let mut stdout = io::stdout();

        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        writeln!(stdout, "\n   =========================================")?;
        writeln!(stdout, "         GOOGLE PING-PONG (RUST EDITION)     ")?;
        writeln!(stdout, "   =========================================")?;

        execute!(stdout, SetForegroundColor(Color::White))?;
        writeln!(stdout, "\n  Select AI Difficulty Level:\n")?;
        writeln!(stdout, "  [1] Easy   (Relaxed pace)")?;
        writeln!(stdout, "  [2] Medium (Standard challenge)")?;
        writeln!(stdout, "  [3] Hard   (Expert precise AI)\n")?;
        write!(stdout, "  Press key [1, 2, or 3] to start: ")?;
        stdout.flush()?;

        Self::enable_raw_mode()?;

        loop {
            if let Ok(Some(key)) = Self::poll_key(Duration::from_millis(10)) {
                match key {
                    '1' => return Ok(AIDifficulty::Easy),
                    '2' => return Ok(AIDifficulty::Medium),
                    '3' => return Ok(AIDifficulty::Hard),
                    _ => {}
                }
            }
        }
    }

    /// Renders Game Over Summary Screen.
    pub fn render_game_over(&self, player_won: bool) -> io::Result<()> {
        let mut stdout = io::stdout();
        writeln!(stdout, "\n")?;

        if player_won {
            execute!(stdout, SetForegroundColor(Config::COLOR_PLAYER))?;
            writeln!(stdout, "  =========================================")?;
            writeln!(stdout, "          YOU WON! CONGRATULATIONS!        ")?;
            writeln!(stdout, "  =========================================")?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            writeln!(stdout, "  You defeated the Computer AI!")?;
        } else {
            execute!(stdout, SetForegroundColor(Config::COLOR_AI))?;
            writeln!(stdout, "  =========================================")?;
            writeln!(stdout, "          GAME OVER - COMPUTER AI WON      ")?;
            writeln!(stdout, "  =========================================")?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            writeln!(stdout, "  Better luck next time!")?;
        }

        writeln!(stdout, "\n  Press any key to exit...")?;
        stdout.flush()?;

        loop {
            if let Ok(Some(_)) = Self::poll_key(Duration::from_millis(20)) {
                break;
            }
        }

        Self::disable_raw_mode()?;
        Ok(())
    }
}
