use macroquad::prelude::*;

// Constants
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


