use macroquad::prelude::*;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut state = State::default();

    loop {
        clear_background(DARKGRAY);

        state.update();
        state.draw();

        next_frame().await;
    }
}

struct Game {
    start_time: f64,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            start_time: get_time(),
        }
    }
}

impl Game {
    pub fn update(&mut self) -> Option<f64> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(get_time() - self.start_time);
        }

        None
    }

    fn time(&self) -> f64 {
        get_time() - self.start_time
    }

    pub fn draw(&self, best_time: f64) {
        self.draw_time(best_time);
    }

    fn draw_time(&self, best_time: f64) {
        let font_size = 24.0;
        let text = format!("Best time: {:.2}", best_time);
        let text_size = measure_text(&text, None, font_size as _, 1.0);
        draw_text(&text, 0.0, screen_height(), font_size, BLACK);

        let text = format!("Your time: {:.2}", self.time());
        draw_text(
            &text,
            0.0,
            screen_height() - text_size.height,
            font_size,
            BLACK,
        );
    }
}

struct State {
    best_time: f64,
    game: Option<Game>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            best_time: 0.0,
            game: None,
        }
    }
}

impl State {
    pub fn update(&mut self) {
        if self.game.is_none() && is_key_pressed(KeyCode::Enter) {
            let game = Game::default();
            self.game = Some(game);
            return;
        }

        let finished = self.game.as_mut().and_then(|game| game.update());

        if let Some(new_time) = finished {
            self.game = None;
            if new_time > self.best_time {
                self.best_time = new_time;
            }
        }
    }

    pub fn draw(&self) {
        if let Some(game) = &self.game {
            game.draw(self.best_time)
        } else {
            draw_menu()
        }
    }
}

pub fn draw_menu() {
    let font_size = 40.0;
    let text = "Press Enter to start game.";
    let text_size = measure_text(text, None, font_size as _, 1.0);
    let text_pos = (
        (screen_width() - text_size.width) / 2.0,
        (screen_height() - text_size.height) / 2.0,
    );
    draw_text(text, text_pos.0, text_pos.1, font_size, BLACK);
}
