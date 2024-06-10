use macroquad::prelude::*;

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

// Constants
pub const PLAYER_SIZE: Vec2 = vec2(100.0, 12.0);
pub const PLAYER_SPEED: f32 = 600.0;
