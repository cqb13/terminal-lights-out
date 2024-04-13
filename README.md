## Game Overview

This game is a light toggle puzzle built in Rust. The goal is to toggle all lights off on a 5x5 grid to solve the puzzle. The game randomly generates a starting board with a number of lights turned on, and the player must find a sequence of moves to turn all lights off.

## Features

- **Random Board Generation**: The game board starts with a random configuration of lights either on or off.
- **Interactive Game Play**: Players can toggle lights on the board. Toggling a light will also toggle adjacent lights vertically and horizontally.
- **Visual Display**: The game provides a simple console-based visual representation of the game board with the current state of each light.

## Usage

1. **Starting the Game**: Run the game using a Rust compiler. Upon launch, the game will display a welcome message and the initial state of the game board.
2. **Playing the Game**: Follow the on-screen instructions to toggle lights. The game will update the display after each move.
3. **Winning the Game**: The game ends when all lights are turned off. A message will be displayed to congratulate the player.

## Requirements

- Rust Programming Language
- `rand` Crate for generating random numbers

## How to Run

Ensure you have Rust and Cargo installed on your machine. Clone the repository, navigate to the directory containing the game, and run:

```bash
cargo run
```

## Contributing

Contributions are welcome! If you'd like to contribute, please fork the repository and use a feature branch. Pull requests are warmly welcome.

## License

The source code for the game is available under the MIT license, allowing for personal and commercial use. See the `LICENSE` file for more details.
