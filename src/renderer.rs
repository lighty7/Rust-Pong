//! Macroquad 2D Graphical Renderer Module.

use crate::ball::Ball;
use crate::config::{BotDifficulty, Config};
use crate::paddle::Paddle;

use macroquad::prelude::*;

pub struct Renderer {
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Clears the screen and draws background
    pub fn clear_screen(&self) {
        clear_background(Config::COLOR_BACKGROUND);
    }

    /// Renders full 2D game state to window
    pub fn render(
        &self,
        ball: &Ball,
        player: &Paddle,
        bot: &Paddle,
        diff: BotDifficulty,
        paused: bool,
    ) {
        self.clear_screen();

        let diff_str = match diff {
            BotDifficulty::Easy => "EASY",
            BotDifficulty::Medium => "MEDIUM",
            BotDifficulty::Hard => "HARD",
        };

        // 1. Top & Bottom Borders
        draw_rectangle(0.0, 0.0, self.width, 10.0, Config::COLOR_BORDER);
        draw_rectangle(
            0.0,
            self.height - 10.0,
            self.width,
            10.0,
            Config::COLOR_BORDER,
        );

        // 2. Dashed Net Line (Center)
        let center_x = self.width / 2.0;
        let mut dash_y = 15.0;
        while dash_y < self.height - 15.0 {
            draw_rectangle(center_x - 2.0, dash_y, 4.0, 15.0, Config::COLOR_NET);
            dash_y += 25.0;
        }

        // 3. HUD Scoreboard & Controls Header
        let score_text = format!(
            "PLAYER: {}    BOT ({}): {}",
            player.score, diff_str, bot.score
        );
        draw_text(&score_text, 40.0, 45.0, 28.0, WHITE);
        draw_text(
            "Controls: W/S or Up/Down | P: Pause | Esc: Quit",
            self.width - 420.0,
            45.0,
            18.0,
            LIGHTGRAY,
        );

        // 4. Player Paddle (Left)
        draw_rectangle(
            player.x as f32 - Config::PADDLE_WIDTH / 2.0,
            player.y as f32 - Config::PADDLE_HEIGHT / 2.0,
            Config::PADDLE_WIDTH,
            Config::PADDLE_HEIGHT,
            Config::COLOR_PLAYER,
        );

        // 5. Bot Paddle (Right)
        draw_rectangle(
            bot.x as f32 - Config::PADDLE_WIDTH / 2.0,
            bot.y as f32 - Config::PADDLE_HEIGHT / 2.0,
            Config::PADDLE_WIDTH,
            Config::PADDLE_HEIGHT,
            Config::COLOR_BOT,
        );

        // 6. Ball
        draw_circle(
            ball.x as f32,
            ball.y as f32,
            Config::BALL_RADIUS,
            Config::COLOR_BALL,
        );

        // 7. Pause Overlay Screen
        if paused {
            draw_rectangle(
                0.0,
                0.0,
                self.width,
                self.height,
                Color::new(0.0, 0.0, 0.0, 0.65),
            );
            let text = "GAME PAUSED";
            let dims = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                (self.width - dims.width) / 2.0,
                self.height / 2.0 - 20.0,
                50.0,
                YELLOW,
            );
            let subtext = "Press P to Resume";
            let sub_dims = measure_text(subtext, None, 24, 1.0);
            draw_text(
                subtext,
                (self.width - sub_dims.width) / 2.0,
                self.height / 2.0 + 30.0,
                24.0,
                WHITE,
            );
        }
    }

    /// Renders Difficulty Selection Menu Screen with Mouse & Key interaction
    pub fn render_menu(&self) -> Option<BotDifficulty> {
        self.clear_screen();

        // Header Title
        let title = "PING-PONG (RUST GUI EDITION)";
        let title_dims = measure_text(title, None, 40, 1.0);
        draw_text(
            title,
            (self.width - title_dims.width) / 2.0,
            120.0,
            40.0,
            SKYBLUE,
        );

        draw_text(
            "Select Bot Difficulty to Start Game:",
            (self.width - 320.0) / 2.0,
            180.0,
            24.0,
            WHITE,
        );

        // Buttons: Easy, Medium, Hard
        let btn_width = 360.0;
        let btn_height = 55.0;
        let btn_x = (self.width - btn_width) / 2.0;

        let mouse_pos = mouse_position();
        let mouse_click = is_mouse_button_pressed(MouseButton::Left);

        let options = [
            (
                BotDifficulty::Easy,
                "1. EASY (Relaxed Pace)",
                240.0,
                KeyCode::Key1,
            ),
            (
                BotDifficulty::Medium,
                "2. MEDIUM (Standard Challenge)",
                320.0,
                KeyCode::Key2,
            ),
            (
                BotDifficulty::Hard,
                "3. HARD (Expert Precise Bot)",
                400.0,
                KeyCode::Key3,
            ),
        ];

        for (diff, label, y_pos, key) in options {
            let is_hover = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + btn_width
                && mouse_pos.1 >= y_pos
                && mouse_pos.1 <= y_pos + btn_height;

            let color = if is_hover {
                Color::new(0.20, 0.45, 0.85, 1.0)
            } else {
                Color::new(0.14, 0.18, 0.26, 1.0)
            };

            draw_rectangle(btn_x, y_pos, btn_width, btn_height, color);
            draw_rectangle_lines(btn_x, y_pos, btn_width, btn_height, 2.0, SKYBLUE);

            let lbl_dims = measure_text(label, None, 22, 1.0);
            draw_text(
                label,
                btn_x + (btn_width - lbl_dims.width) / 2.0,
                y_pos + 34.0,
                22.0,
                WHITE,
            );

            if (is_hover && mouse_click) || is_key_pressed(key) {
                return Some(diff);
            }
        }

        None
    }

    /// Renders Game Over Summary Screen
    pub fn render_game_over(&self, player_won: bool) -> bool {
        draw_rectangle(
            0.0,
            0.0,
            self.width,
            self.height,
            Color::new(0.0, 0.0, 0.0, 0.75),
        );

        let title = if player_won {
            "VICTORY! YOU WON!"
        } else {
            "GAME OVER - BOT WON!"
        };
        let color = if player_won { GREEN } else { RED };

        let dims = measure_text(title, None, 44, 1.0);
        draw_text(
            title,
            (self.width - dims.width) / 2.0,
            self.height / 2.0 - 40.0,
            44.0,
            color,
        );

        let subtext = "Press SPACE or ENTER to Play Again";
        let sub_dims = measure_text(subtext, None, 24, 1.0);
        draw_text(
            subtext,
            (self.width - sub_dims.width) / 2.0,
            self.height / 2.0 + 30.0,
            24.0,
            WHITE,
        );

        is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter)
    }
}
