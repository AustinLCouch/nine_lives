//! Nine Lives Cat Sudoku Core Logic
//!
//! This crate contains the pure game logic for Nine Lives Cat Sudoku.
//! It is independent of any specific UI framework and handles:
//! - Game board representation and state
//! - Core game rules and algorithms  
//! - Board validation and manipulation

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = BoardState::new();
        assert_eq!(board.cells[0][0], None);
        assert_eq!(board.cells[8][8], None);
    }

    #[test]
    fn test_cycle_cell() {
        let mut board = BoardState::new();
        board.cycle_cell(0, 0, 3);
        assert_eq!(board.cells[0][0], Some(0));
        
        board.cycle_cell(0, 0, 3);
        assert_eq!(board.cells[0][0], Some(1));
        
        board.cycle_cell(0, 0, 3);
        assert_eq!(board.cells[0][0], Some(2));
        
        board.cycle_cell(0, 0, 3);
        assert_eq!(board.cells[0][0], Some(0));
    }

    #[test]
    fn test_clear_board() {
        let mut board = BoardState::new();
        board.cycle_cell(1, 1, 5);
        board.cycle_cell(2, 3, 5);
        
        board.clear();
        assert_eq!(board.cells[1][1], None);
        assert_eq!(board.cells[2][3], None);
    }
}
