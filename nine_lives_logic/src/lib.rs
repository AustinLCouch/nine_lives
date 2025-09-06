/// File:  nine_lives_workspace/nine_lives_logic/src/lib.rs
/// Author: Austin Couch
/// File Description:
///     This crate contains the core game logic for Nine Lives Cat Sudoku.
///     It is completely independent of the Bevy game engine and handles
///     the state of the game board and the rules for interacting with it.

// Import the Resource trait from Bevy for the BoardState struct
use bevy::prelude::Resource;

/// The size of one dimension of the Sudoku grid (e.g., 9 for a 9x9 grid).
pub const GRID_SIZE: usize = 9;

/// Represents the state of the game board.
///
/// It derives `Debug` for easy printing and `Clone` to allow for copying.
/// `Resource` is needed for Bevy to use this as a global resource.
#[derive(Debug, Clone, Resource)]
pub struct BoardState {
    /// The cells are stored in a 2D array. Each cell holds an `Option<usize>`.
    /// `Some(i)` represents a cat emoji with index `i`.
    /// `None` represents an empty cell.
    pub cells: [[Option<usize>; GRID_SIZE]; GRID_SIZE],
}

impl BoardState {
    /// Creates a new board with all cells set to `None` (empty).
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }

    /// Resets all cells on the board to `None`.
    pub fn clear(&mut self) {
        self.cells = [[None; GRID_SIZE]; GRID_SIZE];
    }

    /// Cycles the value of a specific cell based on player input.
    ///
    /// The sequence is: None -> Some(0) -> Some(1) -> ... -> Some(max-1) -> Some(0).
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell to cycle.
    /// * `col` - The column index of the cell to cycle.
    /// * `num_emojis` - The total number of available choices (cats).
    pub fn cycle_cell(&mut self, row: usize, col: usize, num_emojis: usize) {
        let current_val = self.cells[row][col];
        let next_val = match current_val {
            None => Some(0),
            Some(idx) => Some((idx + 1) % num_emojis),
        };
        self.cells[row][col] = next_val;
    }
}

// Implementing the `Default` trait provides a convenient way
// to create a new instance, which is useful for `init_resource` in Bevy.
impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}
