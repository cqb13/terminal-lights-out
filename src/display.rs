use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

pub fn refresh_display(lines: i32) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}

pub fn welcome() {
    println!("==============================");
    println!("      Welcome to Lights Out   ");
    println!("      Created by cqb13        ");
    println!("      GitHub: github.com/cqb13");
    println!("==============================");
    print!("\n");

    println!("Controls:");
    println!("  Move with arrows (←↑↓→)");
    println!("  Enter to select");
    println!("  'q' to quit");
    print!("\n");

    println!("Instructions:");
    println!("  Turn off all the lights.");
    println!("  Pressing a light will toggle adjacent blocks.");
    print!("\n");

    println!("Enjoy the game!");
    println!("==============================");
    print!("\n");
}
