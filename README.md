# Nine Lives: Kitten Sudoku 🐱

A delightful cat-themed Sudoku game built with Rust and Bevy! Each number (1-9) is represented by a unique kitten emoji, bringing a purr-fect twist to the classic puzzle game.

## Features

- **Kitten-themed Numbers**: Each digit 1-9 has its own adorable cat emoji
- **Interactive UI**: Click cells and select kittens to fill in the puzzle
- **Win Detection**: Get congratulated with a "purrfect" message when you solve it
- **Responsive Design**: Clean, minimalist interface with cat-themed styling

## How to Play

1. **Run the game**: `cargo run`
2. **Select a kitten**: Click on one of the 9 kitten emojis on the right side
3. **Place the kitten**: Click on an empty cell (white background) to place your selected kitten
4. **Follow Sudoku rules**: Fill each row, column, and 3x3 box with all 9 different kittens
5. **Win**: Complete the puzzle correctly to see the victory message!

## Kitten Legend

- 😸 = 1 (Grinning cat)
- 😻 = 2 (Heart eyes cat)  
- 🙀 = 3 (Weary cat)
- 😿 = 4 (Crying cat)
- 😾 = 5 (Pouting cat)
- 🐱 = 6 (Cat face)
- 🐈 = 7 (Cat)
- 🦁 = 8 (Lion - honorary cat)
- 🐯 = 9 (Tiger - honorary cat)

## Controls

- **Left Click**: Select kitten or place kitten in cell
- **Hover**: Cells and buttons highlight when you hover over them
- **Visual Feedback**: Selected kitten buttons turn green

## Technical Details

- **Engine**: Bevy 0.14
- **Language**: Rust
- **Platform**: Desktop (Mac/PC), with web export support
- **UI**: Bevy's built-in UI system with responsive layout
- **Typography**: MonoLisa font for enhanced readability and style

## Development

To build and run:
```bash
cargo run
```

To build for release:
```bash
cargo build --release
```

## Future Enhancements

- Multiple difficulty levels
- Puzzle generator for endless gameplay  
- Hint system with cat puns
- Timer and scoring
- Sound effects (meows and purrs!)
- Actual kitten images instead of emojis
- Mobile touch controls
- User profiles and progress tracking

Enjoy playing Nine Lives: Kitten Sudoku! 🎉
