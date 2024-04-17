# Lights Out Game in Rust

This game is a light toggle puzzle built in Rust. The goal is to toggle all lights off on a 5x5 grid to solve the puzzle. The game randomly generates a starting board with a number of lights turned on, and the player must find a sequence of moves to turn all lights off.

## Features

- **Grid Generation:** Randomly generates game boards.
- **Game Solver:** Includes an algorithm to find solutions to any given game state.
- **Interactive Play:** Allows users to play manually and see the effects of their actions in real-time.
- **Solution Calculation:** Automatically calculates and displays the minimum number of moves required to solve the game.

## Usage

To play or solve the Lights Out game, follow these steps:

1. Clone the repository to your local machine.

```sh
git clone https://github.com/cqb13/terminal-lights-out.git
```

2. Run the game using Cargo:

```sh
cargo run
```

Upon running, the program will prompt you to choose between playing the game manually or solving a puzzle.

### Play Mode

In Play mode, you interact with the game board by toggling lights to turn off all lights. The game displays the current board state and lets you choose lights to toggle.

### Solve Mode

In Solve mode, the game automatically calculates and displays the solution to the entered puzzle. The solution consists of a sequence of moves (represented by numbers) to turn off all lights.

## Contributing

Contributions are welcome! If you have suggestions for improvements or find a bug, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
