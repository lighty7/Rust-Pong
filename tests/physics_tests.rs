//! Integration unit tests for Rust Ping-Pong physics and collision system.

use rust_pingpong::ball::Ball;
use rust_pingpong::paddle::Paddle;

#[test]
fn test_ball_movement() {
    let mut ball = Ball::new(40.0, 12.0);
    let initial_x = ball.x;
    ball.update(0.1);
    assert_ne!(ball.x, initial_x);
}

#[test]
fn test_wall_bounce() {
    let mut ball = Ball::new(40.0, 0.0);
    let initial_dir_y = ball.dir_y;
    ball.bounce_y();
    assert_eq!(ball.dir_y, -initial_dir_y);
}

#[test]
fn test_paddle_collision() {
    let paddle = Paddle::new(5.0, 10.0, 5);
    assert!(paddle.check_collision(5.0, 10.0));
    assert!(!paddle.check_collision(100.0, 10.0));
}

#[test]
fn test_score_tracking() {
    let mut paddle = Paddle::new(5.0, 10.0, 5);
    assert_eq!(paddle.score, 0);
    paddle.increment_score();
    assert_eq!(paddle.score, 1);
    paddle.reset_score();
    assert_eq!(paddle.score, 0);
}
