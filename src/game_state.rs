use std::{
    collections::LinkedList,
    ops::{Add, AddAssign, Mul},
    time::{Duration, Instant},
};

use crossterm::{
    event::{KeyCode, KeyEvent},
    style::{StyledContent, Stylize},
};

use crate::canvas::Canvas;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn to_speed(self) -> Coord {
        match self {
            Self::DOWN => Coord { x: 0, y: 1 },
            Self::UP => Coord { x: 0, y: -1 },
            Self::LEFT => Coord { x: -1, y: 0 },
            Self::RIGHT => Coord { x: 1, y: 0 },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<i16> for Coord {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct GameState {
    dir: Direction,
    dir_list: LinkedList<Direction>,
    size: Coord,
    food: Coord,
    snake: LinkedList<Coord>,
    speed: Coord,
    speed_factor: i16,
    last_update: Instant,
    game_over: bool,
    tick_delay: Duration,
}

impl GameState {
    pub fn new(size_x: i16, size_y: i16, tps: u64) -> GameState {
        let dir = Direction::RIGHT;
        let speed = dir.to_speed();
        let size = Coord {
            x: size_x,
            y: size_y,
        };
        let init_y = size_y / 2;
        let init_x = size_x / 4;
        let snake = LinkedList::from([
            Coord {
                x: init_x,
                y: init_y,
            },
            Coord {
                x: init_x - 1,
                y: init_y,
            },
            Coord {
                x: init_x - 2,
                y: init_y,
            },
        ]);

        let tick_delay = 1000 / tps;

        let mut state = GameState {
            dir: Direction::RIGHT,
            dir_list: LinkedList::new(),
            size,
            food: Coord { x: 0, y: 0 },
            snake,
            speed,
            last_update: Instant::now(),
            speed_factor: 1,
            game_over: false,
            tick_delay: Duration::from_millis(tick_delay),
        };

        state.spawn_food();
        state
    }

    fn spawn_food(&mut self) {
        'outer: loop {
            let x = rand::random_range(1..self.size.x - 1);
            let y = rand::random_range(1..self.size.y - 1);

            for s in self.snake.iter() {
                if s.x == x && s.y == y {
                    continue 'outer;
                }
            }
            self.food = Coord { x, y };
            break;
        }
    }

    pub fn handle_input(&mut self, ev: KeyEvent) {
        let dir = match self.dir_list.back() {
            Some(&dir) => dir,
            None => self.dir,
        };
        if ev.code == KeyCode::Up && ev.is_press() && dir != Direction::DOWN && dir != Direction::UP {
            self.dir_list.push_back(Direction::UP);
        }

        if ev.code == KeyCode::Down && ev.is_press() && dir != Direction::UP && dir != Direction::DOWN {
            self.dir_list.push_back(Direction::DOWN);
        }

        if ev.code == KeyCode::Left && ev.is_press() && dir != Direction::RIGHT && dir != Direction::LEFT {
            self.dir_list.push_back(Direction::LEFT);
        }

        if ev.code == KeyCode::Right && ev.is_press() && dir != Direction::LEFT && dir != Direction::RIGHT {
            self.dir_list.push_back(Direction::RIGHT);
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        if self.last_update.elapsed() < self.tick_delay {
            return;
        }

        if let Some(dir) = self.dir_list.pop_front() {
            self.dir = dir;
        }

        self.speed = self.dir.to_speed();

        // First move snake
        let tail = self.snake.pop_back().unwrap();
        let mut head = *self.snake.front().unwrap();
        head += self.speed * self.speed_factor;
        head = self.warp(head);

        if head == self.food {
            self.snake.push_back(tail);
            self.spawn_food();
        }

        for &s in self.snake.iter() {
            if s == head {
                self.game_over = true;
                break;
            }
        }
        self.snake.push_front(head);

        self.last_update = Instant::now();
    }

    pub fn draw(&self, canvas: &mut Canvas<StyledContent<char>>) {
        canvas.draw(self.food.x as u16, self.food.y as u16, 'F'.yellow());

        for s in self.snake.iter() {
            canvas.draw(s.x as u16, s.y as u16, 'S'.green());
        }

        if self.game_over {
            let head = self.snake.front().unwrap();

            canvas.draw(head.x as u16, head.y as u16, 'X'.red());
        }
    }

    fn warp(&self, mut pos: Coord) -> Coord {
        if pos.x <= 0 {
            pos.x = self.size.x - 2;
        }
        if pos.y <= 0 {
            pos.y = self.size.y - 2;
        }

        if pos.x >= self.size.x - 1 {
            pos.x = 1;
        }
        if pos.y >= self.size.y - 1 {
            pos.y = 1;
        }

        pos
    }

    pub fn set_tps(&mut self, tps: u64) {
        self.tick_delay = Duration::from_millis(1000 / tps);
    }
}
