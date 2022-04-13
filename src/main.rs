//! Пример мини-игры с использованием macroquad.
//! Управляем небольшим кораблём, уклоняясь от астероидов.
//! Задача: продержаться как можно дольше.

use crate::rand::RandomRange;
use macroquad::prelude::*;

// Точка входа в приложение. Макрос позволяет сделать функцию main асинхронной,
// а также иницилизирует окно.
#[macroquad::main("Asteroids")]
async fn main() {
    // Инициализирум состояние наший игры по умолчанию.
    let mut state = State::default();

    // Запускаем игровой цикл.
    loop {
        // Очищаем фон тёмно-серым цветом.
        clear_background(DARKGRAY);

        // Обновляем состояние игры.
        state.update();

        // Отображаем игру в окне.
        state.draw();

        // Ожидаем возможности заняться следующим кадром.
        next_frame().await;
    }
}

/// Состояние приложения.
struct State {
    /// Рекорное время.
    best_time: f64,
    /// Состояние игрового процесса.
    game: Option<Game>,
}

/// Логика создания состояния приложения.
impl Default for State {
    fn default() -> Self {
        Self {
            best_time: 0.0,
            game: None, // Изначально находимся в меню.
        }
    }
}

impl State {
    /// Логика обновления приложения.
    pub fn update(&mut self) {
        // Если нажат Enter - запускаем игру.
        if self.game.is_none() && is_key_pressed(KeyCode::Enter) {
            let game = Game::default(); // Создаём новое состояние игрового процесса.
            self.game = Some(game); // Запоминаем его.
            return;
        }

        // Если мы в игре - обновляем её состояние.
        let finished = self.game
            .as_mut(). // получаем уникальную (мутабельную) ссылку на содержимое Option, если оно есть.
            and_then(|game| { // Если получили, то выполняем функтор,
                game.update() // который обновляет состояние игры.
            });

        // Если игра завершена - то получим время, которое игроку удалось продержаться.
        if let Some(new_time) = finished {
            self.game = None; // Завершаем игру.
            if new_time > self.best_time {
                // Если новое время дольше рекордного,
                self.best_time = new_time; // то обновляем рекорд.
            }
        }
    }

    /// Отображение приложения.
    pub fn draw(&self) {
        // Если игра запущена - отображаем её,
        if let Some(game) = &self.game {
            game.draw(self.best_time)
        } else {
            // иначе, рисуем меню.
            Self::draw_menu()
        }
    }

    /// Отображение меню
    fn draw_menu() {
        let font_size = 40.0;
        let text = "Press Enter to start game.";

        // Вычисляем, какой размер занимает текст на экране.
        let text_size = measure_text(text, None, font_size as _, 1.0);

        // Располагаем текст по центру.
        let text_pos = (
            (screen_width() - text_size.width) / 2.0,
            (screen_height() - text_size.height) / 2.0,
        );

        // Отображаем текст
        draw_text(text, text_pos.0, text_pos.1, font_size, BLACK);
    }
}

/// Состояние игрового процесса.
struct Game {
    /// Время, когда игра запустилась.
    start_time: f64,
    /// Время предыдущего обновления состояния игры.
    last_update: f64,
    /// Корабль игрока.
    ship: Ship,
    /// Таймер появления астероидов.
    asteroid_timer: f64,
    /// Вектор астероидов.
    asteroids: Vec<Asteroid>,
}

impl Default for Game {
    /// Логика создания новой игры.
    fn default() -> Self {
        let time = get_time(); // Текущее время со старта приложения.
        Self {
            start_time: time,
            last_update: time,
            ship: Ship::default(),
            asteroid_timer: 0.0,
            asteroids: Vec::with_capacity(100), // Создаём пустой вектор,
                                                // способный вместить в себя до 100 астероидов без дополнительных аллокаций.
        }
    }
}

impl Game {
    /// Логика обновления игрового процесса.
    pub fn update(&mut self) -> Option<f64> {
        if is_key_pressed(KeyCode::Escape) {
            // Если нажат Escape - выходим в меню.
            return Some(get_time() - self.start_time);
        }

        let elapsed_time = self.elapsed_time(); // Время, прошедшее с предыдущего кадра.

        self.asteroid_timer += elapsed_time; // Обновляем таймер появления астероидов.
        if self.asteroid_timer > 0.5 {
            // Если астероид не появлялся уже полсекунды,
            self.asteroid_timer = 0.0; // сбрасываем таймер
            self.asteroids.push(Asteroid::default()); // и создаём новый астероид.
        }

        // Забываем астероиды, вышедшие за пределы экрана.
        self.asteroids.retain(|asteroid| !asteroid.out_of_bounds());

        // Обновляем состояние астероиндов.
        for asteroid in &mut self.asteroids {
            asteroid.update(elapsed_time, self.ship.vertical_speed());
            if self.ship.is_collapse(asteroid.position, asteroid.radius) {
                // Если астероид столкнулся с кораблём, то завершаем игру.
                return Some(self.game_time());
            }
        }

        self.ship.update(elapsed_time); // Обновляем состояние корабля.

        self.last_update = get_time(); // Запоминаем время завершения обновления кадра.
        None // Игра продолжается.
    }

    /// Отображаем игру.
    pub fn draw(&self, best_time: f64) {
        self.draw_time(best_time); // Отображаем текст с лучшим и текущим временем.
        self.ship.draw(); // Отображаем корабль.

        // Отображаем астероиды.
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }

    /// Время в текущей игре.
    fn game_time(&self) -> f64 {
        get_time() - self.start_time
    }

    /// Время, прошедшее с последнего обновления.
    fn elapsed_time(&self) -> f64 {
        get_time() - self.last_update
    }

    /// Отображаем текст с лучшим и текущим временем.
    fn draw_time(&self, best_time: f64) {
        let font_size = 24.0;
        let text = format!("Best time: {:.2}", best_time);
        let text_size = measure_text(&text, None, font_size as _, 1.0);
        draw_text(&text, 0.0, screen_height(), font_size, BLACK);

        let time = self.game_time();
        let text = format!("Your time: {:.2}", time);

        // Если текущее время лучше рекордного, отображаем его зелёным цветом.
        let color = if time > best_time { GREEN } else { BLACK };

        draw_text(
            &text,
            0.0,
            screen_height() - text_size.height,
            font_size,
            color,
        );
    }
}

/// Состояние корабля.
pub struct Ship {
    /// Положение по горизонтали.
    position: f32,
    /// Скорость по горизонтали.
    speed: f32,
    /// Скорость по вертикали (с которой, относительно корабля, движутся астероиды)
    vertical_speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: screen_width() / 2.0, // Изначально корабль находится по центру окна.
            speed: 0.0,
            vertical_speed: 100.0,
        }
    }
}

impl Ship {
    // Параметры корабля.
    const SHIP_WIDTH: f32 = 25.0;
    const SHIP_HEIGHT: f32 = 50.0;
    const SHIP_OFFSET: f32 = 30.0;

    /// Логика обновления корабля.
    pub fn update(&mut self, elapsed_time: f64) {
        const ACCELERATION: f32 = 200.0;
        const VERTICAL_ACCELERATION: f32 = 50.0;
        const DECELERATION: f32 = 180.0;
        let elapsed_time = elapsed_time as f32;

        // Замедляем корабль по горизонтали.
        self.speed /= DECELERATION * elapsed_time;

        // Если нажата А, то ускоряем корабль влево.
        if is_key_down(KeyCode::A) {
            self.speed -= ACCELERATION * elapsed_time;
        }

        // Если нажата D, то ускоряем корабль вправо.
        if is_key_down(KeyCode::D) {
            self.speed += ACCELERATION * elapsed_time;
        }

        // Перемещаем корабль.
        self.position += self.speed;

        // Не даём кораблю выйти за пределы окна.
        self.position = self.position.clamp(
            Self::SHIP_WIDTH / 2.0,
            screen_width() - Self::SHIP_WIDTH / 2.0,
        );

        // Ускоряем корабль по вертикали.
        self.vertical_speed += VERTICAL_ACCELERATION * elapsed_time;
    }

    /// Отображаем корабль.
    pub fn draw(&self) {
        // Вычисляем точки треугольника.
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

        // Отображаем треугольник.
        draw_triangle(top, right, left, WHITE)
    }

    /// Столкнулся ли корабль с кругом с центром в `point` и радиусом `radius`.
    pub fn is_collapse(&self, point: Vec2, radius: f32) -> bool {
        // Вычисляем приблизительный радиус корабля.
        let ship_radius = (Self::SHIP_WIDTH + Self::SHIP_HEIGHT) / 4.0;

        // Вычисляем положение центра корабля.
        let ship_center = Vec2::new(self.position, screen_height() - Self::SHIP_OFFSET);

        // Проверяем, не пересекаются ли радиусы корабля и круга.
        (point - ship_center).length() < radius + ship_radius
    }

    /// Скорость корабля по вертикали.
    pub fn vertical_speed(&self) -> f32 {
        self.vertical_speed
    }
}

/// Состояние астероида.
struct Asteroid {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Default for Asteroid {
    fn default() -> Self {
        // Располагаем астероид случайно, немного выше видимого экрана.
        let x = f32::gen_range(0.0, screen_width());
        let y = -2.0 * Self::MAX_RADIUS;

        // Задаём случайную скорость астероиду.
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
    // Параметры астероидов
    const MIN_RADIUS: f32 = 25.0;
    const MAX_RADIUS: f32 = 100.0;
    const MAX_SPEED: f32 = 200.0;

    /// Проверка выхода астероида далеко за границы экрана.
    pub fn out_of_bounds(&self) -> bool {
        let (x, y) = (self.position.x, self.position.y);
        let left = -3.0 * Self::MAX_RADIUS;
        let right = screen_width() + 3.0 * Self::MAX_RADIUS;
        let bottom = screen_height() + 3.0 * Self::MAX_RADIUS;
        x < left || x > right || y > bottom
    }

    /// Обновление состояния астероида.
    pub fn update(&mut self, elapsed_time: f64, ship_speed: f32) {
        let elapsed_time = elapsed_time as f32;
        self.position += self.speed * elapsed_time;
        self.position.y += ship_speed * elapsed_time;
    }

    /// Отображение астероида.
    pub fn draw(&self) {
        // Отображаем астероид в виде круга.
        draw_circle(self.position.x, self.position.y, self.radius, LIGHTGRAY);
    }
}
