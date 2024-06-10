use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub block_type: BlockType,
    pub color: Color,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType,  color: Color) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_type,
            color,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => self.color,
                _ => Color::new(self.color.r, self.color.g, self.color.b, 0.5),
            },
            BlockType::SpawnBallOnDeath => DARKGRAY,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

// Constants
pub const BLOCK_SIZE: Vec2 = vec2(100.0, 40.0);
