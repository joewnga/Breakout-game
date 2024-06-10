use macroquad::prelude::*;
use crate::{ball::Circle, block::{Block,BlockType}};

pub fn draw_title_text(text: &str, font: &Font) {
    let dims = measure_text(text, Some(font), 50, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5 - dims.width * 0.5,
        screen_height() * 0.5 - dims.height * 0.5,
        TextParams {
            font: Some(font),
            font_size: 50,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

pub fn resolve_collision(circle: &mut Circle, vel: &mut Vec2, rect: &Rect) -> bool {
    let closest_x = circle.center.x.clamp(rect.x, rect.x + rect.w);
    let closest_y = circle.center.y.clamp(rect.y, rect.y + rect.h);

    let distance_x = circle.center.x - closest_x;
    let distance_y = circle.center.y - closest_y;

    if distance_x.powi(2) + distance_y.powi(2) < circle.radius.powi(2) {
        if distance_x.abs() > distance_y.abs() {
            vel.x = -vel.x;
        } else {
            vel.y = -vel.y;
        }
        true
    } else {
        false
    }
}

pub fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<crate::ball::Ball>,
    player: &mut crate::player::Player,
) {
    *player = crate::player::Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    balls.push(crate::ball::Ball::new(vec2(
        screen_width() * 0.5,
        screen_height() * 0.5,
    )));
    blocks.clear();
    init_blocks(blocks);
}

pub fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5.0;
    let total_block_size = crate::block::BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5,
        50.0,
    );

    let colors = [
        RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE,
    ];

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        let layer = i / width; 
        let color = colors[layer % colors.len()]; 
        blocks.push(Block::new(
            board_start_pos + vec2(block_x, block_y),
            BlockType::Regular,
            color,
        ));
    }

    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}



//Unit Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ball::Circle;

    // Mock function to simulate collision without macroquad runtime
    fn mock_resolve_collision(circle: &mut Circle, vel: &mut Vec2, rect: &Rect) -> bool {
        let closest_x = circle.center.x.clamp(rect.x, rect.x + rect.w);
        let closest_y = circle.center.y.clamp(rect.y, rect.y + rect.h);

        let distance_x = circle.center.x - closest_x;
        let distance_y = circle.center.y - closest_y;

        if distance_x.powi(2) + distance_y.powi(2) < circle.radius.powi(2) {
            if distance_x.abs() > distance_y.abs() {
                vel.x = -vel.x;
            } else {
                vel.y = -vel.y;
            }
            true
        } else {
            false
        }
    }

    #[test]
    fn collision_detection() {
        let mut circle = Circle::new(vec2(50.0, 50.0), 10.0);
        let mut vel = vec2(1.0, 1.0);
        let rect = Rect::new(45.0, 45.0, 10.0, 10.0);

       
        let collision = mock_resolve_collision(&mut circle, &mut vel, &rect);
        assert!(collision);
    }
}
