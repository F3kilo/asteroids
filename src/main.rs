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
    last_update: f64,
    ship: Ship,
}

impl Default for Game {
    fn default() -> Self {
        let time = get_time();
        Self {
            start_time: time,
            last_update: time,
            ship: Ship::default(),
        }
    }
}

impl Game {
    pub fn update(&mut self) -> Option<f64> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(get_time() - self.start_time);
        }

        let elapsed_time = self.elapsed_time();

        self.ship.update(elapsed_time);

        self.last_update = get_time();
        None
    }

    fn time(&self) -> f64 {
        get_time() - self.start_time
    }

    fn elapsed_time(&self) -> f64 {
        get_time() - self.last_update
    }

    pub fn draw(&self, best_time: f64) {
        self.draw_time(best_time);
        self.ship.draw();
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
            Self::draw_menu()
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
}

pub struct Ship {
    position: f32,
    speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: screen_width() / 2.0,
            speed: 0.0,
        }
    }
}

impl Ship {
    const SHIP_WIDTH: f32 = 25.0;
    const SHIP_HEIGHT: f32 = 50.0;
    const SHIP_OFFSET: f32 = 30.0;

    pub fn update(&mut self, elapsed_time: f64) {
        const ACCELERATION: f32 = 200.0;
        const DECELERATION: f32 = 180.0;
        let elapsed_time = elapsed_time as f32;

        self.speed /= DECELERATION * elapsed_time;

        if is_key_down(KeyCode::A) {
            self.speed -= ACCELERATION * elapsed_time;
        }

        if is_key_down(KeyCode::D) {
            self.speed += ACCELERATION * elapsed_time;
        }

        self.position += self.speed;
        self.position = self.position.clamp(
            Self::SHIP_WIDTH / 2.0,
            screen_width() - Self::SHIP_WIDTH / 2.0,
        );
    }

    pub fn draw(&self) {
        let top = Vec2::new(
            self.position,
            screen_height() - Self::SHIP_HEIGHT / 2.0 - Self::SHIP_OFFSET,
        );
        let left = Vec2::new(
            self.position - Self::SHIP_WIDTH / 2.0,
            screen_height() - Self::SHIP_OFFSET,
        );
        let right = Vec2::new(
            self.position + Self::SHIP_WIDTH / 2.0,
            screen_height() - Self::SHIP_OFFSET,
        );
        draw_triangle(top, right, left, WHITE)
    }
}
