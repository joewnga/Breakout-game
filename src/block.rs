use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = vec2(100.0, 40.0);

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




//Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn block_initial_position() {
        let block = Block::new(vec2(50.0, 50.0), BlockType::Regular, RED);
        
        // Assert that the block's initial position is set correctly
        assert_eq!(block.rect.x, 50.0);
        assert_eq!(block.rect.y, 50.0);
    }

    #[test]
    fn block_lives() {
        let mut block = Block::new(vec2(50.0, 50.0), BlockType::Regular, RED);
        
        // Assert that the block's initial lives are 2
        assert_eq!(block.lives, 2);
        
        // Decrease the block's lives and assert the new value
        block.lives -= 1;
        assert_eq!(block.lives, 1);
    }
}
