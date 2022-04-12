use macroquad::prelude::*;

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        clear_background(DARKGRAY);
        next_frame().await;
    }
}
