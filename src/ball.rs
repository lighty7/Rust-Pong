//! Ball kinematics and physics reflection module.
//!
//! # TUTORIAL SYNTAX & CONCEPTS:
//! 1. Struct Definition: `pub struct Ball` defines memory fields.
//! 2. Method Signatures:
//!    - `&mut self`: Mutable borrow. Allows method to modify internal fields (`self.x`, `self.y`).
//!    - `&self`: Immutable borrow. Read-only access to fields.
//! 3. External Crate (`rand`): `rand::thread_rng()` creates thread-local random number generator.

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub speed: f64,
}

impl Ball {
    /// Creates a new Ball centered at given coordinates.
    pub fn new(start_x: f64, start_y: f64) -> Self {
        let mut ball = Self {
            x: start_x,
            y: start_y,
            dir_x: 1.0,
            dir_y: 0.0,
            speed: 450.0,
        };
        ball.reset(start_x, start_y);
        ball
    }

    /// Resets position and randomizes initial serve angle.
    pub fn reset(&mut self, start_x: f64, start_y: f64) {
        self.x = start_x;
        self.y = start_y;
        self.speed = 450.0;

        let mut rng = rand::thread_rng();
        self.dir_x = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        self.dir_y = rng.gen_range(-0.6..=0.6);
    }

    /// Updates spatial coordinates based on delta time step (`dt`).
    pub fn update(&mut self, delta_time: f64) {
        self.x += self.dir_x * self.speed * delta_time;
        self.y += self.dir_y * self.speed * delta_time;
    }

    /// Reverses vertical velocity upon hitting top or bottom borders.
    pub fn bounce_y(&mut self) {
        self.dir_y = -self.dir_y;
    }

    /// Reverses horizontal velocity and applies paddle strike angle spin.
    pub fn bounce_x(&mut self, hit_offset: f64) {
        self.dir_x = -self.dir_x;
        self.dir_y = hit_offset * 1.2;

        // Increase volley speed slightly over time
        if self.speed < 850.0 {
            self.speed += 25.0;
        }
    }
}
