use super::{Game, Point, GRID_SIZE};
use crate::display::refresh_display;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

pub fn game_loop(mut game: Game) {
    let mut current_point = Point::new(2, 2);
    let mut moves = 0;
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
                    game.toggle_light(&current_point);
                    moves += 1;
                    if game.solved() {
                        refresh_display(GRID_SIZE);
                        game.display();
                        println!("Solved in {} moves", moves);
                        if game.shortest_solution.is_some() {
                            println!("Best solve: {} moves", game.shortest_solution.unwrap());
                        }
                        break;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(GRID_SIZE);
        game.display_with_selector(&current_point);
    }
}
