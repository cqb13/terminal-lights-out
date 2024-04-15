// based on https://www.keithschwarz.com/interesting/code/?dir=lights-out
use super::{Board, NumberBoard, Point, GRID_SIZE};

// Solves the Lights Out puzzle using Gaussian elimination and back substitution.
pub fn solve_lights_out(mut board: &Board) {
    let toggle_matrix = make_toggle_matrix();
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
fn linearize_puzzle() {}

fn perform_gaussian_elimination() {}

// Performs back substitution on a row-reduced toggle matrix to find a solution vector.
fn back_substitute() {}

// Converts a solution vector back into a list of (row, col) pairs indicating button presses.
fn convert_solution_to_button_presses() {}
