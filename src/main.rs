extern crate ncurses;

use std::{thread, time};
use ncurses::*;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    y: i32,
    x: i32,
}

struct Snake {
    direction: Direction,
    max_size: usize,
    positions: Vec<Position>,
}

impl Snake {
    fn get_head(&self) -> Position {
        self.positions.get(self.positions.len() - 1).unwrap().clone()
    }
}

static GAME_SPEED: u64 = 40;
static SNAKE_INIT_SIZE: usize = 4;
static SNAKE_INIT_DIRECTION: Direction = Direction::Right;

fn main() {
    let mut rng = rand::thread_rng();

    initscr();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // non blocking getch()
    nodelay(stdscr(), true);

    // get screen bounds
    let mut window_height = 0;
    let mut window_width = 0;
    getmaxyx(stdscr(), &mut window_height, &mut window_width);

    let start_pos = Position {
        y: window_height / 2,
        x: window_width / 2,
    };

    let mut snake = Snake {
        direction: SNAKE_INIT_DIRECTION,
        max_size: SNAKE_INIT_SIZE,
        positions: vec![start_pos],
    };

    let mut game_over = false;

    let mut candy = Position {
        y: rng.gen_range(2, window_height - 3),
        x: rng.gen_range(2, window_width - 3),
    };

    loop {
        // LOGIC

        // handle input
        let ch = getch();

        snake.direction = match ch {
            KEY_LEFT if snake.direction != Direction::Right => Direction::Left,
            KEY_RIGHT if snake.direction != Direction::Left => Direction::Right,
            KEY_UP if snake.direction != Direction::Down => Direction::Up,
            KEY_DOWN if snake.direction != Direction::Up => Direction::Down,
            _ => snake.direction,
        };

        match snake.direction {
            Direction::Left => {
                let head = snake.get_head();
                snake.positions.push(Position { x: head.x - 1, y: head.y });
            },
            Direction::Right => {
                let head = snake.get_head();
                snake.positions.push(Position { x: head.x + 1, y: head.y });
            },
            Direction::Up => {
                let head = snake.get_head();
                snake.positions.push(Position { x: head.x, y: head.y - 1 });
            },
            Direction::Down => {
                let head = snake.get_head();
                snake.positions.push(Position { x: head.x, y: head.y + 1 });
            }
        }

        if snake.positions.len() > snake.max_size {
            snake.positions.remove(0);
        }

        // COLLISIONS
        let head = snake.get_head();

        // candy noms?
        if head == candy {
            snake.max_size += 1;
            candy = Position {
                y: rng.gen_range(2, window_height - 3),
                x: rng.gen_range(2, window_width - 3),
            };
        }

        // walls hit?
        if head.x <= 1 || head.x >= window_width - 2 || head.y <= 1 || head.y >= window_height - 2 {
            game_over = true;
        }

        // self hit?
        for &position in snake.positions[0..snake.positions.len() - 1].into_iter() {
            if head == position {
                game_over = true;
                break;
            }
        }

        if game_over {
            break;
        }

        // DRAW
        clear();

        // outer walls
        box_(stdscr(), 0, 0);

        // candy
        mvprintw(candy.y, candy.x, "x");

        // snek
        for position in snake.positions.clone() {
            mvprintw(position.y, position.x, "o");
        }

        refresh();

        // zzZZZzzZZZzzz
        thread::sleep(time::Duration::from_millis(GAME_SPEED));
    }

    endwin();
}
