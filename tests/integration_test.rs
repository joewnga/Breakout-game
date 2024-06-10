// tests/integration_test.rs
use breakout_game::player::{Player, PLAYER_SIZE, PLAYER_SPEED};
use breakout_game::ball::{Ball, Circle, BALL_RADIUS, BALL_SPEED};
use breakout_game::utils::init_blocks;
use macroquad::prelude::*;

//test ball movement
#[test]
fn ball_movement() {
    
    let mut ball = Ball {
        circle: Circle::new(vec2(50.0, 50.0), BALL_RADIUS),
        vel: vec2(1.0, 1.0).normalize(),
    };
    ball.circle.center += ball.vel * BALL_SPEED * 1.0; 

    assert!(ball.circle.center.x != 50.0 || ball.circle.center.y != 50.0);
}

// #[test]
// fn game_initialization() {
    
//     const MOCK_SCREEN_WIDTH: f32 = 800.0;
//     const MOCK_SCREEN_HEIGHT: f32 = 600.0;

//     let player = Player {
//         rect: Rect::new(
//             MOCK_SCREEN_WIDTH * 0.5 - PLAYER_SIZE.x * 0.5,
//             MOCK_SCREEN_HEIGHT - 100.0,
//             PLAYER_SIZE.x,
//             PLAYER_SIZE.y,
//         ),
//     };
//     assert!(player.rect.x >= 0.0);

//     let ball = Ball {
//         circle: Circle::new(vec2(50.0, 50.0), BALL_RADIUS),
//         vel: vec2(1.0, 1.0).normalize(),
//     };
//     assert_eq!(ball.circle.center.x, 50.0);
//     assert_eq!(ball.circle.center.y, 50.0);

//     let mut blocks = Vec::new();
//     init_blocks(&mut blocks);
//     assert!(!blocks.is_empty());
// }

// #[test]
// fn player_movement() {
//     const MOCK_SCREEN_WIDTH: f32 = 800.0;
//     const MOCK_SCREEN_HEIGHT: f32 = 600.0;

//     // Testing player movement
//     let mut player = Player {
//         rect: Rect::new(
//             MOCK_SCREEN_WIDTH * 0.5 - PLAYER_SIZE.x * 0.5,
//             MOCK_SCREEN_HEIGHT - 100.0,
//             PLAYER_SIZE.x,
//             PLAYER_SIZE.y,
//         ),
//     };

//     // Simulate movement to the right
//     let initial_x = player.rect.x;
//     player.rect.x += PLAYER_SPEED * 1.0;

//     assert!(player.rect.x >= 0.0 && player.rect.x + player.rect.w <= MOCK_SCREEN_WIDTH);
//     assert!(player.rect.x > initial_x);
// }


