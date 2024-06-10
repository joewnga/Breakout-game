mod game;
mod player;
mod ball;
mod block;
mod utils;

use game::run;

#[macroquad::main("breakout")]
async fn main() {
    run().await;
}
