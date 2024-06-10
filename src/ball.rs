use macroquad::prelude::*;

pub const BALL_RADIUS: f32 = 15.0; 
pub const BALL_SPEED: f32 = 400.0;

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn draw(&self, color: Color) {
        draw_circle(self.center.x, self.center.y, self.radius, color);
    }

    
}

pub struct Ball {
    pub circle: Circle,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            circle: Circle::new(pos, BALL_RADIUS),
            vel: vec2(rand::gen_range(-1.0, 1.0), 1.0).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.circle.center += self.vel * dt * BALL_SPEED;

        if self.circle.center.x - self.circle.radius < 0.0 {
            self.vel.x = 1.0;
        }
        if self.circle.center.x + self.circle.radius > screen_width() {
            self.vel.x = -1.0;
        }
        if self.circle.center.y - self.circle.radius < 0.0 {
            self.vel.y = 1.0;
        }
    }

    pub fn draw(&self) {
        self.circle.draw(WHITE);
    }
}



//Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    // Mock function to simulate ball update without macroquad runtime
    fn mock_ball_update(ball: &mut Ball, dt: f32, screen_width: f32) {
        ball.circle.center += ball.vel * dt * BALL_SPEED;

        if ball.circle.center.x - ball.circle.radius < 0.0 {
            ball.vel.x = 1.0;
        }
        if ball.circle.center.x + ball.circle.radius > screen_width {
            ball.vel.x = -1.0;
        }
        if ball.circle.center.y - ball.circle.radius < 0.0 {
            ball.vel.y = 1.0;
        }
    }

    // Mock screen dimensions
    const MOCK_SCREEN_WIDTH: f32 = 800.0;
    

    #[test]
    fn ball_initial_position() {
    
        let ball = Ball::new(vec2(50.0, 50.0));
        
        // Assert that the ball's initial position is set correctly
        assert_eq!(ball.circle.center.x, 50.0);
        assert_eq!(ball.circle.center.y, 50.0);
    }

    #[test]
    fn ball_update_position() {
        
        let mut ball = Ball::new(vec2(50.0, 50.0));
        mock_ball_update(&mut ball, 1.0, MOCK_SCREEN_WIDTH); // Assuming dt = 1.0 for simplicity
        
        // Assert that the ball's position has changed
        assert!(ball.circle.center.x != 50.0 || ball.circle.center.y != 50.0);
    }
}