mod display;
mod game;
mod solver;

use display::welcome;
use game::game_loop;
use rand::{self, Rng};
use solver::solve_lights_out;

pub const GRID_SIZE: i32 = 5;

pub type Board = [[Square; GRID_SIZE as usize]; GRID_SIZE as usize];
pub type NumberBoard = [[i32; GRID_SIZE as usize]; GRID_SIZE as usize];

pub struct Game {
    board: Board,
    shortest_solution: Option<i32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[Square::Off; GRID_SIZE as usize]; GRID_SIZE as usize],
            shortest_solution: None,
        }
    }

    pub fn board_to_numbers(&self) -> NumberBoard {
        let mut number_board: NumberBoard = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];

        for (y, row) in self.board.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                number_board[y][x] = square.to_number();
            }
        }

        number_board
    }

    pub fn generate_board(&mut self) {
        let mut rand = rand::thread_rng();
        let moves = rand.gen_range(10..30);

        for _ in 0..moves {
            let x = rand.gen_range(0..GRID_SIZE - 1);
            let y = rand.gen_range(0..GRID_SIZE - 1);

            self.toggle_light(&Point::new(x, y));
        }
    }

    pub fn toggle_light(&mut self, point: &Point) {
        self.board[point.y as usize][point.x as usize] =
            Square::opposite(&self.board[point.y as usize][point.x as usize]);

        if point.valid_left() {
            self.board[point.y as usize][(point.x - 1) as usize] =
                Square::opposite(&self.board[point.y as usize][(point.x - 1) as usize]);
        }

        if point.valid_right() {
            self.board[point.y as usize][(point.x + 1) as usize] =
                Square::opposite(&self.board[point.y as usize][(point.x + 1) as usize]);
        }

        if point.valid_up() {
            self.board[(point.y - 1) as usize][point.x as usize] =
                Square::opposite(&self.board[(point.y - 1) as usize][point.x as usize]);
        }

        if point.valid_down() {
            self.board[(point.y + 1) as usize][point.x as usize] =
                Square::opposite(&self.board[(point.y + 1) as usize][point.x as usize]);
        }
    }

    pub fn solved(&self) -> bool {
        for row in self.board {
            for square in row {
                if square == Square::On {
                    return false;
                }
            }
        }

        true
    }

    pub fn display_with_selector(&self, point: &Point) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if y == point.y as usize && x == point.x as usize {
                    print!("|{}|", square.to_visual());
                } else {
                    print!(" {} ", square.to_visual());
                }
            }
            println!();
        }
    }

    pub fn display(&self) {
        for row in self.board {
            for square in row {
                print!(" {} ", square.to_visual());
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn valid_left(&self) -> bool {
        if self.x == 0 {
            false
        } else {
            true
        }
    }

    pub fn valid_right(&self) -> bool {
        if self.x == GRID_SIZE - 1 {
            false
        } else {
            true
        }
    }

    pub fn valid_up(&self) -> bool {
        if self.y == 0 {
            false
        } else {
            true
        }
    }

    pub fn valid_down(&self) -> bool {
        if self.y == GRID_SIZE - 1 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Square {
    On,
    Off,
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Square::On => match other {
                Square::On => true,
                Square::Off => false,
            },
            Square::Off => match other {
                Square::Off => true,
                Square::On => false,
            },
        }
    }
}

impl Square {
    pub fn to_visual(&self) -> String {
        match self {
            Square::Off => "○".to_string(),
            Square::On => "⦿".to_string(),
        }
    }

    pub fn to_number(&self) -> i32 {
        match self {
            Square::Off => 0,
            Square::On => 1,
        }
    }
}

impl Square {
    pub fn opposite(square: &Square) -> Square {
        match square {
            Square::Off => Square::On,
            Square::On => Square::Off,
        }
    }
}

fn main() {
    welcome();
    //TODO: add two options, play and solve(shows solution steps)
    let mut game = Game::new();
    game.generate_board();
    game.display();
    solve_lights_out(&game.board);

    //game_loop(game);
}
