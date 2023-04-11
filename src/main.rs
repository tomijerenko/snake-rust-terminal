use rand::Rng;
use std::io::{stdin, stdout};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    clear_screen();

    let (tx, rx) = channel::<GameKeys>();
    let _stdout = stdout().into_raw_mode();
    let mut game: Game = Game::new(30, 15);
    let speed_milliseconds: u16 = 1000;

    let keyboard_handle = thread::spawn(move || {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up => tx.send(GameKeys::Up).unwrap(),
                Key::Down => tx.send(GameKeys::Down).unwrap(),
                Key::Right => tx.send(GameKeys::Right).unwrap(),
                Key::Left => tx.send(GameKeys::Left).unwrap(),
                Key::Esc => break,
                _ => continue,
            }
        }
        tx.send(GameKeys::Esc).unwrap();
    });

    // The game starts here
    clear_screen();
    game.spawn_snake();
    game.spawn_food();
    loop {
        if let Some(key) = rx.try_iter().last() {
            if let GameKeys::Esc = key {
                break;
            } else {
                game.set_direction(key);
            }
        }
        if !game.move_snake() {
            drop(rx);
            println!("YOU LOST!");
            break;
        }
        game.draw();
        thread::sleep(Duration::from_millis(speed_milliseconds as u64));
    }

    keyboard_handle.is_finished();
}

struct Game {
    width: u16,
    height: u16,
    field: Vec<Vec<GameObject>>,
    snake: Vec<[usize; 2]>,
    direction: GlideDirection,
    points: u16,
}
impl Game {
    fn new(width: u16, height: u16) -> Self {
        let mut field: Vec<Vec<GameObject>> = Vec::new();

        // Initialize empty field
        for i in 0..height {
            let mut col: Vec<GameObject> = Vec::new();
            for j in 0..width {
                if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                    col.push(GameObject::Wall);
                } else {
                    col.push(GameObject::Space);
                }
            }
            field.push(col);
        }

        Game {
            width,
            height,
            field,
            snake: Vec::new(),
            direction: GlideDirection::Right,
            points: 0,
        }
    }

    fn move_snake(&mut self) -> bool {
        let mut can_move = true;
        let mut head: [usize; 2] = self.snake[0];
        let tail: [usize; 2] = self.snake.pop().unwrap();

        // Move head in glide direction
        match self.direction {
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
            GameObject::Wall => can_move = false,
            GameObject::Snake => can_move = false,
            GameObject::Space => self.field[tail[1]][tail[0]] = GameObject::Space,
        };
        self.field[head[1]][head[0]] = GameObject::Snake;
        self.snake.insert(0, head);

        can_move
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

    fn set_direction(&mut self, direction: GameKeys) {
        match direction {
            GameKeys::Up => self.direction = GlideDirection::Up,
            GameKeys::Down => self.direction = GlideDirection::Down,
            GameKeys::Right => self.direction = GlideDirection::Right,
            GameKeys::Left => self.direction = GlideDirection::Left,
            _ => (),
        }
    }

    fn draw(&self) {
        clear_screen();
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
}

fn clear_screen() {
    print!("{}[2J\r\n", 27 as char);
}

enum GameObject {
    Wall,
    Snake,
    Food,
    Space,
}

enum GlideDirection {
    Up,
    Down,
    Right,
    Left,
}

enum GameKeys {
    Up,
    Down,
    Right,
    Left,
    Esc,
}
