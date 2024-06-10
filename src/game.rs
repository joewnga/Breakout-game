use macroquad::prelude::*;
use crate::{{player::Player, ball::Ball, block::BlockType}, utils::{draw_title_text, GameState, resolve_collision, reset_game, init_blocks}};

pub async fn run() {
    let font = load_ttf_font("assets/fonts/Merriweather-BoldItalic.ttf")
        .await
        .expect("Failed to load font");

    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    balls.push(Ball::new(vec2(
        screen_width() * 0.5,
        screen_height() * 0.6,
    )));

    init_blocks(&mut blocks);

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.circle, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.circle, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                score += 10;
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(ball.circle.center));
                                }
                            }
                        }
                    }
                }
                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }

                let balls_len = balls.len();
                balls.retain(|ball| ball.circle.center.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;
                    if player_lives > 0 {
                        balls.push(Ball::new(
                            player.rect.point()
                                + vec2(player.rect.w * 0.5, -50.0),
                        ));
                    } else {
                        game_state = GameState::Dead;
                    }
                }
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            }
            GameState::LevelCompleted | GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut blocks,
                        &mut balls,
                        &mut player,
                    );
                }
            }
        }

        
        clear_background(GRAY);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to begin!", &font);
            }
            GameState::Game => {
                let score_text = format!("Score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(&font), 30, 1.0);
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5 - score_text_dim.width * 0.5,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 15,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    &format!("Lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 15,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! Your Score: {}", score), &font);
            }
            GameState::Dead => {
                draw_title_text(&format!("Game Over! Your Score: {}", score), &font);
            }
        }

        next_frame().await
    }
}
