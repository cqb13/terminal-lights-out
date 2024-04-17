// based on https://www.keithschwarz.com/interesting/code/?dir=lights-out
use super::{Board, Game, Point, GRID_SIZE};
use crate::display::refresh_display;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};


pub fn setup() -> Game {
    let mut game = Game::new();
    let mut current_point = Point::new(2, 2);
    println!("Press 'S' to save and continue or 'Q' to quit");
    game.display_with_selector(&current_point);
    loop {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        let event = read().unwrap();
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    println!("Quitting...");
                    std::process::exit(0);
                }
                KeyCode::Char('s') => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    refresh_display(GRID_SIZE);
                    break;
                }
                KeyCode::Up => {
                    if current_point.valid_up() {
                        current_point.y = current_point.y - 1;
                    } else {
                        current_point.y = GRID_SIZE - 1;
                    }
                }
                KeyCode::Down => {
                    if current_point.valid_down() {
                        current_point.y = current_point.y + 1;
                    } else {
                        current_point.y = 0;
                    }
                }
                KeyCode::Left => {
                    if current_point.valid_left() {
                        current_point.x = current_point.x - 1;
                    } else {
                        current_point.x = GRID_SIZE - 1;
                    }
                }
                KeyCode::Right => {
                    if current_point.valid_right() {
                        current_point.x = current_point.x + 1;
                    } else {
                        current_point.x = 0;
                    }
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    game.toggle_single_light(&current_point);
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(GRID_SIZE);
        game.display_with_selector(&current_point);
    }
    refresh_display(1);

    game
}

// Solves the Lights Out puzzle using Gaussian elimination and back substitution.
pub fn solve_lights_out(mut board: &Board) {
    let mut toggle_matrix = make_toggle_matrix();
    let mut puzzle_vector = linearize_puzzle(board);
    perform_gaussian_elimination(toggle_matrix, puzzle_vector);
}

// Creates a toggle matrix (25x25 for default game) for the given puzzle size, indicating the effect of pressing each button.
// One row for each button
fn make_toggle_matrix(
) -> [[bool; GRID_SIZE as usize * GRID_SIZE as usize]; GRID_SIZE as usize * GRID_SIZE as usize] {
    let mut matrix =
        [[false; GRID_SIZE as usize * GRID_SIZE as usize]; GRID_SIZE as usize * GRID_SIZE as usize];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let index = (y * GRID_SIZE + x) as usize; // Flattening the grid to a single dimension
            matrix[index][index] = true;

            let point = Point::new(x as i32, y as i32);

            if point.valid_left() {
                matrix[index][index - 1] = true;
            }
            if point.valid_right() {
                matrix[index][index + 1] = true;
            }
            if point.valid_up() {
                matrix[index][index - GRID_SIZE as usize] = true;
            }
            if point.valid_down() {
                matrix[index][index + GRID_SIZE as usize] = true;
            }
        }
    }

    matrix
}

// Converts the puzzle matrix into a linear vector in row-major order.
fn linearize_puzzle(board: &Board) -> Vec<bool> {
    let mut linear_vector: Vec<bool> = Vec::new();

    for row in board {
        for square in row {
            linear_vector.push(square.to_boolean());
        }
    }

    linear_vector
}

fn perform_gaussian_elimination(
    mut toggle_matrix: [[bool; GRID_SIZE as usize * GRID_SIZE as usize];
        GRID_SIZE as usize * GRID_SIZE as usize],
    mut puzzle_vector: Vec<bool>,
) {

}

// Performs back substitution on a row-reduced toggle matrix to find a solution vector.
fn back_substitute() {}

// Converts a solution vector back into a list of (row, col) pairs indicating button presses.
fn convert_solution_to_button_presses() {}
