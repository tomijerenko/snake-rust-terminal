use rand::Rng;
use std::io::{stdin, stdout};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut game: Game = Game::new(60, 30, 1000);
    let _stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    game.spawn_snake();
    game.spawn_food();
    game.draw();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => game.move_snake(GlideDirection::Up),
            Key::Down => game.move_snake(GlideDirection::Down),
            Key::Right => game.move_snake(GlideDirection::Right),
            Key::Left => game.move_snake(GlideDirection::Left),
            Key::Esc => break,
            _ => {}
        }
        game.draw();
    }
}

struct Game {
    width: u16,
    height: u16,
    speed: u16,
    field: Vec<Vec<GameObject>>,
    snake: Vec<[usize; 2]>,
    food: [usize; 2],
}
impl Game {
    fn new(width: u16, height: u16, speed_milliseconds: u16) -> Self {
        let mut field: Vec<Vec<GameObject>> = Vec::new();

        // Initialize empty field
        for i in 0..height {
            let mut col: Vec<GameObject> = Vec::new();
            for j in 0..width {
                if i == 0 || i == height - 1 {
                    col.push(GameObject::Wall);
                } else if j == 0 || j == width - 1 {
                    col.push(GameObject::Wall);
                } else {
                    col.push(GameObject::Space);
                }
            }
            field.push(col);
        }

        Game {
            width: width,
            height: height,
            speed: speed_milliseconds,
            field: field,
            snake: Vec::new(),
            food: [0, 0],
        }
    }

    fn move_snake(&mut self, direction: GlideDirection) {
        let mut head: [usize; 2] = self.snake[0];
        let tail: [usize; 2] = self.snake.pop().unwrap();

        // Move head in glide direction
        match direction {
            GlideDirection::Up => head[1] -= 1,
            GlideDirection::Down => head[1] += 1,
            GlideDirection::Right => head[0] += 1,
            GlideDirection::Left => head[0] -= 1,
        }

        //Remove tail, add head
        match self.field[head[1]][head[0]] {
            GameObject::Food => {
                self.snake.push(tail);
                self.spawn_food();
            }
            GameObject::Wall => Self::_defeat(),
            GameObject::Snake => Self::_defeat(),
            GameObject::Space => self.field[tail[1]][tail[0]] = GameObject::Space,
            _ => {}
        };
        self.field[head[1]][head[0]] = GameObject::Snake;
        self.snake.insert(0, head);
    }

    fn spawn_snake(&mut self) {
        let head: [usize; 2] = [
            rand::thread_rng().gen_range(1..self.width - 1) as usize,
            rand::thread_rng().gen_range(1..self.height - 1) as usize,
        ];
        self.snake.push(head);
        self.field[head[1]][head[0]] = GameObject::Snake;
    }

    fn spawn_food(&mut self) {
        let mut food = self.snake[0];
        while self.snake[0][0] == food[0] && self.snake[0][1] == food[1] {
            food = [
                rand::thread_rng().gen_range(1..self.width - 1) as usize,
                rand::thread_rng().gen_range(1..self.height - 1) as usize,
            ];
        }
        self.field[food[1]][food[0]] = GameObject::Food;
    }

    fn draw(&self) {
        print!("{}[2J\r\n", 27 as char); // Clear the terminal screen
        for i in 0..self.height {
            for j in 0..self.width {
                match self.field[i as usize][j as usize] {
                    GameObject::Wall => print!("*"),
                    GameObject::Snake => print!("x"),
                    GameObject::Food => print!("o"),
                    GameObject::Space => print!(" "),
                }
            }
            print!("\r\n");
        }
    }

    fn _defeat() {
        print!("You lost!\r\n");
        process::exit(0);
    }

    fn _victory() {}
}

enum GlideDirection {
    Up,
    Down,
    Right,
    Left,
}

enum GameObject {
    Wall,
    Snake,
    Food,
    Space,
}
