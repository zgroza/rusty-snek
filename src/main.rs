use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, ExecutableCommand};
use crossterm::event::{Event, KeyEvent, KeyCode};
use rand::Rng;

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;
const SNAKE_CHAR: char = 'O';
const FOOD_CHAR: char = 'X';

struct Snake {
    body: Vec<(u16, u16)>,
    growSpace: (u16, u16),
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![(WIDTH / 2, HEIGHT / 2)],
            growSpace: (WIDTH / 2 + 1, HEIGHT / 2 + 1),
            direction: Direction::Right,
        }
    }

    fn move_forward(&mut self) {
        let (head_x, head_y) = self.body[0];
        let (new_x, new_y) = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };
        self.body.insert(0, (new_x, new_y));
        self.growSpace = self.body.pop().unwrap();
    }

    fn grow(&mut self) {
        self.body.push(self.growSpace);
    }

    fn check_collision(&self) -> bool {
        let (head_x, head_y) = self.body[0];
        head_x == 0 || head_x == WIDTH - 1 || head_y == 0 || head_y == HEIGHT - 1 ||
        self.body[1..].contains(&(head_x, head_y))
    }
}

fn main() {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let mut snake = Snake::new();
    let mut food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
    let mut score = 0;

    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    for y in 0..HEIGHT {
        for x in [0, WIDTH - 1] {
            stdout.execute(cursor::MoveTo(x, y)).unwrap();
            print!("#");
        }
    }
    for x in 1..WIDTH - 1 {
        for y in [0, HEIGHT - 1] {
            stdout.execute(cursor::MoveTo(x, y)).unwrap();
            print!("#");
        }
    }
    
    'main_loop: loop {
        stdout.execute(cursor::MoveTo(snake.growSpace.0, snake.growSpace.1)).unwrap();
        print!(" ");
        stdout.execute(cursor::MoveTo(food.0, food.1)).unwrap();
        print!("{}", FOOD_CHAR);
        for &(x, y) in &snake.body {
            stdout.execute(cursor::MoveTo(x, y)).unwrap();
            print!("{}", SNAKE_CHAR);
        }
        stdout.flush().unwrap();

        if crossterm::event::poll(Duration::from_millis(200 - snake.body.len() as u64)).unwrap() {
            match crossterm::event::read() {
                Ok(Event::Key(KeyEvent { code, .. })) => {
                    snake.direction = match code {
                        KeyCode::Up => Direction::Up,
                        KeyCode::Down => Direction::Down,
                        KeyCode::Left => Direction::Left,
                        KeyCode::Right => Direction::Right,
                        KeyCode::Esc => {break 'main_loop;},
                        _ => snake.direction,
                    };
                },
                _ => {}
            }
        }

        snake.move_forward();

        if snake.body[0] == food {
            snake.grow();
            food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
            score += 1;
        }

        if snake.check_collision() {
            break;
        }
    }

    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    println!("Game over! Your score is {}", score);
}
