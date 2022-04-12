use crate::rand::RandomRange;
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
    asteroid_timer: f64,
    asteroids: Vec<Asteroid>,
}

impl Default for Game {
    fn default() -> Self {
        let time = get_time();
        Self {
            start_time: time,
            last_update: time,
            ship: Ship::default(),
            asteroid_timer: 0.0,
            asteroids: Vec::with_capacity(100),
        }
    }
}

impl Game {
    pub fn update(&mut self) -> Option<f64> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(get_time() - self.start_time);
        }

        let elapsed_time = self.elapsed_time();
        self.asteroid_timer += elapsed_time;

        if self.asteroid_timer > 0.5 {
            self.asteroid_timer = 0.0;
            self.asteroids.push(Asteroid::default());
        }

        self.ship.update(elapsed_time);

        self.asteroids.retain(|asteroid| !asteroid.out_of_bounds());
        for asteroid in &mut self.asteroids {
            asteroid.update(elapsed_time, self.ship.vertical_speed());
            if self.ship.is_collapse(asteroid.position, asteroid.radius) {
                return Some(self.game_time());
            }
        }

        self.last_update = get_time();
        None
    }

    fn game_time(&self) -> f64 {
        get_time() - self.start_time
    }

    fn elapsed_time(&self) -> f64 {
        get_time() - self.last_update
    }

    pub fn draw(&self, best_time: f64) {
        self.draw_time(best_time);
        self.ship.draw();
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }

    fn draw_time(&self, best_time: f64) {
        let font_size = 24.0;
        let text = format!("Best time: {:.2}", best_time);
        let text_size = measure_text(&text, None, font_size as _, 1.0);
        draw_text(&text, 0.0, screen_height(), font_size, BLACK);

        let text = format!("Your time: {:.2}", self.game_time());
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
    vertical_speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: screen_width() / 2.0,
            speed: 0.0,
            vertical_speed: 100.0,
        }
    }
}

impl Ship {
    const SHIP_WIDTH: f32 = 25.0;
    const SHIP_HEIGHT: f32 = 50.0;
    const SHIP_OFFSET: f32 = 30.0;

    pub fn is_collapse(&self, point: Vec2, radius: f32) -> bool {
        let ship_radius = (Self::SHIP_WIDTH + Self::SHIP_HEIGHT) / 4.0;
        let ship_center = Vec2::new(
            self.position,
            screen_height() - Self::SHIP_OFFSET,
        );
        (point - ship_center).length() < radius + ship_radius
    }

    pub fn update(&mut self, elapsed_time: f64) {
        const ACCELERATION: f32 = 200.0;
        const VERTICAL_ACCELERATION: f32 = 50.0;
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

        self.vertical_speed += VERTICAL_ACCELERATION * elapsed_time;
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

    pub fn vertical_speed(&self) -> f32 {
        self.vertical_speed
    }
}

struct Asteroid {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Default for Asteroid {
    fn default() -> Self {
        let x = f32::gen_range(0.0, screen_width());
        let y = -2.0 * Self::MAX_RADIUS;

        let speed_x = f32::gen_range(0.0, Self::MAX_SPEED);
        let speed_y = f32::gen_range(0.0, Self::MAX_SPEED);
        Self {
            position: Vec2::new(x, y),
            speed: Vec2::new(speed_x, speed_y),
            radius: f32::gen_range(Self::MIN_RADIUS, Self::MAX_RADIUS),
        }
    }
}

impl Asteroid {
    const MIN_RADIUS: f32 = 25.0;
    const MAX_RADIUS: f32 = 100.0;
    const MAX_SPEED: f32 = 100.0;

    pub fn out_of_bounds(&self) -> bool {
        let (x, y) = (self.position.x, self.position.y);
        let left = -3.0 * Self::MAX_RADIUS;
        let right = screen_width() + 3.0 * Self::MAX_RADIUS;
        let bottom = screen_height() + 3.0 * Self::MAX_RADIUS;
        x < left || x > right || y > bottom
    }

    pub fn update(&mut self, elapsed_time: f64, ship_speed: f32) {
        let elapsed_time = elapsed_time as f32;
        self.position += self.speed * elapsed_time;
        self.position.y += ship_speed * elapsed_time;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, LIGHTGRAY);
    }
}
