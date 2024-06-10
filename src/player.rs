use macroquad::prelude::*;


pub const PLAYER_SIZE: Vec2 = vec2(100.0, 12.0);
pub const PLAYER_SPEED: f32 = 600.0;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - PLAYER_SIZE.x * 0.5,
                screen_height() - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

        self.rect.x = self.rect.x.clamp(0.0, screen_width() - self.rect.w);
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }
}



//Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    
    fn mock_player_update(player: &mut Player, dt: f32, screen_width: f32) {
        let x_move = 1.0; 
        player.rect.x += x_move * dt * PLAYER_SPEED;

        player.rect.x = player.rect.x.clamp(0.0, screen_width - player.rect.w);
    }

    
    const MOCK_SCREEN_WIDTH: f32 = 800.0;
    const MOCK_SCREEN_HEIGHT: f32 = 600.0;

    #[test]
    fn player_initial_position() {
        
        let player = Player {
            rect: Rect::new(
                MOCK_SCREEN_WIDTH * 0.5 - PLAYER_SIZE.x * 0.5,
                MOCK_SCREEN_HEIGHT - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        };

        assert!(player.rect.x >= 0.0 && player.rect.x + player.rect.w <= MOCK_SCREEN_WIDTH);
        assert!(player.rect.y >= 0.0 && player.rect.y + player.rect.h <= MOCK_SCREEN_HEIGHT);
    }

    #[test]
    fn player_update_position() {
        let mut player = Player {
            rect: Rect::new(
                MOCK_SCREEN_WIDTH * 0.5 - PLAYER_SIZE.x * 0.5,
                MOCK_SCREEN_HEIGHT - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        };

        
        mock_player_update(&mut player, 1.0, MOCK_SCREEN_WIDTH); 

        assert!(player.rect.x >= 0.0 && player.rect.x + player.rect.w <= MOCK_SCREEN_WIDTH);
    }
}
