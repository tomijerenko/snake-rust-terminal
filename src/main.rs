use rand::Rng;
use std::io::{Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('c') => { println!("c") }
            Key::Esc => { println!("ESC") }
            _ => {
                break
            }
        }
    }

    //let game: Game = Game::new(60, 30, 1000);
    //game.draw();
}

enum GameObject {
    Wall,
    Snake,
    Food,
    Space
}

struct Game {
    width: u16,
    height: u16,
    speed: u16,
    field: Vec<Vec<GameObject>>,
    snake: Vec<[usize; 2]>,
}
impl Game {
    fn new(width: u16, height: u16, speed_milliseconds: u16) -> Self {
        let mut field: Vec<Vec<GameObject>> = Vec::new();
        let mut snake: Vec<[usize; 2]> = Vec::new();
        let mut food: [usize; 2];

        // Initialize empty field
        for i in 0..height {
            let mut col: Vec<GameObject> = Vec::new();
            for j in 0..width {
                if i == 0 || i == height-1 {
                    col.push(GameObject::Wall);
                } else if j == 0 || j == width-1 {
                    col.push(GameObject::Wall);
                } else {
                    col.push(GameObject::Space);
                }
            }
            field.push(col);
        }

        // Initialize and spawn a snake
        snake.push([
            rand::thread_rng().gen_range(1..width-1) as usize,
            rand::thread_rng().gen_range(1..height-1) as usize,
        ]);
        field[snake[0][1]][snake[0][0]] = GameObject::Snake;

        // Initialize and spawn food
        // Ensure they don't spawn in the same place
        food = [snake[0][0], snake[0][1]];
        while snake[0][0] == food[0] && snake[0][1] == food[1] {
            food = [
                rand::thread_rng().gen_range(1..width-1) as usize,
                rand::thread_rng().gen_range(1..height-1) as usize,
            ];
        }
        field[food[1]][food[0]] = GameObject::Food;

        Game {
            width:  width,
            height: height,
            speed: speed_milliseconds,
            field: field,
            snake: snake,
        }

    }

    fn draw(&self) {
        print!("{}[2J", 27 as char); // Clear the terminal screen

        for i in 0..self.height {
            for j in 0..self.width {
                match self.field[i as usize][j as usize] {
                    GameObject::Wall => print!("*"),
                    GameObject::Snake => print!("x"),
                    GameObject::Food => print!("o"),
                    GameObject::Space => print!(" ")
                }
            }
            println!();
        }
    }
}

struct Snake {}
impl Snake {
    fn glide(&self) {}
    fn eat(&self) {}
    fn grow(&self) {}
    fn change_direction(&self) {}
}

enum GlideDirection {
    Up,
    Down,
    Right,
    Left
}

