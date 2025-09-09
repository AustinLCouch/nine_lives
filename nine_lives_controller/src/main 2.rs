//! Nine Lives Cat Sudoku - Main Application Entry Point
//!
//! This is the main binary for the Nine Lives Cat Sudoku game.
//! It uses the MVC architecture with three separate crates:
//! - `nine_lives_core` (Model): Pure game logic and data structures
//! - `nine_lives_ui` (View): User interface, rendering, and presentation
//! - `nine_lives_controller` (Controller): Event handling and application orchestration

use nine_lives_controller::run_game;

fn main() {
    // Run the Nine Lives Cat Sudoku game
    // The controller orchestrates the entire application
    run_game();
}
