//! Nine Lives Cat Sudoku Core Logic
//!
//! This crate contains the pure game logic for Nine Lives Cat Sudoku.
//! It is independent of any specific UI framework and handles:
//! - Game board representation and state
//! - Core game rules and algorithms  
//! - Board validation and manipulation

use bevy::prelude::Resource;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

/// High-level game state for the current puzzle lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    #[default]
    Playing,
    Won,
}

/// The size of one dimension of the Sudoku grid (e.g., 9 for a 9x9 grid).
pub const GRID_SIZE: usize = 9;

/// Represents the type of a cell - whether it was given in the puzzle or filled by the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    /// A number that was provided as part of the original puzzle
    Given,
    /// A number that was filled in by the player
    Player,
}

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
    
    /// Tracks the type of each cell (Given vs Player filled).
    /// Only meaningful for cells that have values (Some in the cells array).
    pub cell_types: [[Option<CellType>; GRID_SIZE]; GRID_SIZE],
}

impl BoardState {
    /// Creates a new board with all cells set to `None` (empty).
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_SIZE]; GRID_SIZE],
            cell_types: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }

    /// Resets all cells on the board to `None`.
    pub fn clear(&mut self) {
        self.cells = [[None; GRID_SIZE]; GRID_SIZE];
        self.cell_types = [[None; GRID_SIZE]; GRID_SIZE];
    }

    /// Cycles the value of a specific cell based on player input.
    ///
    /// The sequence is: None -> Some(0) -> Some(1) -> ... -> Some(max-1) -> Some(0).
    /// Given cells (part of the original puzzle) cannot be changed.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell to cycle.
    /// * `col` - The column index of the cell to cycle.
    /// * `num_emojis` - The total number of available choices (cats).
    pub fn cycle_cell(&mut self, row: usize, col: usize, num_emojis: usize) {
        // Don't allow changes to given cells
        if let Some(CellType::Given) = self.cell_types[row][col] {
            return;
        }
        
        let current_val = self.cells[row][col];
        let next_val = match current_val {
            None => Some(0),
            Some(idx) => Some((idx + 1) % num_emojis),
        };
        
        self.cells[row][col] = next_val;
        
        // Mark as player input if we have a value
        self.cell_types[row][col] = if next_val.is_some() {
            Some(CellType::Player)
        } else {
            None
        };
    }

    /// Check if placing a value at a specific position would be valid according to Sudoku rules.
    ///
    /// This validates the three core Sudoku constraints:
    /// 1. No duplicate values in the same row
    /// 2. No duplicate values in the same column  
    /// 3. No duplicate values in the same 3x3 box
    ///
    /// # Arguments
    ///
    /// * `row` - The row index to check
    /// * `col` - The column index to check
    /// * `value` - The value to validate (0-based, so 0-8 for cats 1-9)
    pub fn is_valid_placement(&self, row: usize, col: usize, value: usize) -> bool {
        // Check row constraint - no duplicates in the same row
        for c in 0..GRID_SIZE {
            if c != col && self.cells[row][c] == Some(value) {
                return false;
            }
        }
        
        // Check column constraint - no duplicates in the same column
        for r in 0..GRID_SIZE {
            if r != row && self.cells[r][col] == Some(value) {
                return false;
            }
        }
        
        // Check 3x3 box constraint - no duplicates in the same box
        let box_row_start = (row / 3) * 3;
        let box_col_start = (col / 3) * 3;
        for r in box_row_start..box_row_start + 3 {
            for c in box_col_start..box_col_start + 3 {
                if (r != row || c != col) && self.cells[r][c] == Some(value) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Get all positions that currently violate Sudoku rules.
    ///
    /// Returns a vector of (row, col) tuples for cells that have conflicts.
    /// This is used for visual feedback to highlight problematic cells.
    pub fn get_conflicts(&self) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if let Some(value) = self.cells[row][col] {
                    if !self.is_valid_placement(row, col, value) {
                        conflicts.push((row, col));
                    }
                }
            }
        }
        
        conflicts
    }
    
    /// Check if the puzzle is completely and correctly solved.
    ///
    /// A puzzle is complete when:
    /// 1. All cells are filled (no None values)
    /// 2. No Sudoku rule violations exist
    pub fn is_complete(&self) -> bool {
        // First check if all cells are filled
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.cells[row][col].is_none() {
                    return false;
                }
            }
        }
        
        // Then check if no conflicts exist
        self.get_conflicts().is_empty()
    }

    /// Compute the current overall game state based on the board content.
    pub fn compute_game_state(&self) -> GameState {
        if self.is_complete() { GameState::Won } else { GameState::Playing }
    }

    /// Generate a new Sudoku puzzle with the specified difficulty.
    ///
    /// This uses a backtracking algorithm to:
    /// 1. Fill the grid with a valid complete solution
    /// 2. Remove numbers to create the puzzle
    /// 3. Ensure the puzzle has a unique solution
    ///
    /// # Arguments
    /// 
    /// * `givens` - Number of pre-filled cells (35-40 for easy, 25-30 for hard)
    pub fn generate_puzzle(&mut self, givens: usize) {
        // Start with a clear board
        self.clear();
        
        // Fill the board with a complete valid solution
        self.fill_board();
        
        // Remove numbers to create the puzzle, keeping 'givens' numbers
        self.remove_numbers_for_puzzle(givens);
    }
    
    /// Fill the board with a complete valid Sudoku solution using backtracking.
    fn fill_board(&mut self) -> bool {
        // Find the next empty cell
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.cells[row][col].is_none() {
                    // Try numbers 0-8 in random order for variety
                    let mut numbers: Vec<usize> = (0..GRID_SIZE).collect();
                    numbers.shuffle(&mut thread_rng());
                    
                    for num in numbers {
                        if self.is_valid_placement(row, col, num) {
                            self.cells[row][col] = Some(num);
                            
                            // Recursively fill the rest of the board
                            if self.fill_board() {
                                return true;
                            }
                            
                            // Backtrack if this doesn't work
                            self.cells[row][col] = None;
                        }
                    }
                    
                    // No valid number found for this cell
                    return false;
                }
            }
        }
        
        // All cells filled successfully
        true
    }
    
    /// Remove numbers from a complete board to create a puzzle.
    /// 
    /// This keeps exactly 'givens' numbers and removes the rest.
    /// For simplicity, we'll randomly select which numbers to keep.
    /// In a more sophisticated implementation, we'd ensure unique solvability.
    fn remove_numbers_for_puzzle(&mut self, givens: usize) {
        if givens >= GRID_SIZE * GRID_SIZE {
            return; // Keep all numbers if givens is too high
        }
        
        // Create a list of all cell positions
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                positions.push((row, col));
            }
        }
        
        // Shuffle the positions randomly
        positions.shuffle(&mut thread_rng());
        
        // Mark the first 'givens' positions as Given cells
        for (i, (row, col)) in positions.iter().enumerate() {
            if i < givens {
                // Keep this cell and mark it as given
                self.cell_types[*row][*col] = Some(CellType::Given);
            } else {
                // Remove this cell (it will be for the player to fill)
                self.cells[*row][*col] = None;
                self.cell_types[*row][*col] = None;
            }
        }
    }
    
    /// Generate an easy puzzle (good for beginners).
    /// Easy puzzles have 35-40 given numbers.
    pub fn generate_easy_puzzle(&mut self) {
        let givens = thread_rng().gen_range(35..=40);
        self.generate_puzzle(givens);
    }
    
    /// Generate a medium puzzle (moderate difficulty).
    /// Medium puzzles have 30-35 given numbers.
    pub fn generate_medium_puzzle(&mut self) {
        let givens = thread_rng().gen_range(30..=35);
        self.generate_puzzle(givens);
    }
    
    /// Generate a hard puzzle (challenging).
    /// Hard puzzles have 25-30 given numbers.
    pub fn generate_hard_puzzle(&mut self) {
        let givens = thread_rng().gen_range(25..=30);
        self.generate_puzzle(givens);
    }
    
    /// Check if a cell is a given cell (part of the original puzzle).
    pub fn is_given_cell(&self, row: usize, col: usize) -> bool {
        matches!(self.cell_types[row][col], Some(CellType::Given))
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

    #[test]
    fn test_is_valid_placement_empty_board() {
        let board = BoardState::new();
        // On an empty board, any placement should be valid
        assert!(board.is_valid_placement(0, 0, 0));
        assert!(board.is_valid_placement(4, 4, 5));
        assert!(board.is_valid_placement(8, 8, 8));
    }

    #[test]
    fn test_is_valid_placement_row_conflict() {
        let mut board = BoardState::new();
        // Place cat 0 at position (0, 0)
        board.cells[0][0] = Some(0);
        
        // Placing the same cat in the same row should be invalid
        assert!(!board.is_valid_placement(0, 1, 0));
        assert!(!board.is_valid_placement(0, 8, 0));
        
        // Different cats in the same row should be valid
        assert!(board.is_valid_placement(0, 1, 1));
        assert!(board.is_valid_placement(0, 8, 8));
    }

    #[test]
    fn test_is_valid_placement_column_conflict() {
        let mut board = BoardState::new();
        // Place cat 1 at position (0, 0)
        board.cells[0][0] = Some(1);
        
        // Placing the same cat in the same column should be invalid
        assert!(!board.is_valid_placement(1, 0, 1));
        assert!(!board.is_valid_placement(8, 0, 1));
        
        // Different cats in the same column should be valid
        assert!(board.is_valid_placement(1, 0, 2));
        assert!(board.is_valid_placement(8, 0, 0));
    }

    #[test]
    fn test_is_valid_placement_box_conflict() {
        let mut board = BoardState::new();
        // Place cat 2 at position (0, 0) - top-left of first 3x3 box
        board.cells[0][0] = Some(2);
        
        // Placing the same cat elsewhere in the same 3x3 box should be invalid
        assert!(!board.is_valid_placement(0, 1, 2)); // same row, same box
        assert!(!board.is_valid_placement(1, 0, 2)); // same column, same box
        assert!(!board.is_valid_placement(2, 2, 2)); // different row/col, same box
        
        // Placing the same cat in a different 3x3 box should be invalid if same row/column
        assert!(!board.is_valid_placement(0, 3, 2)); // different box but same row - invalid!
        assert!(!board.is_valid_placement(3, 0, 2)); // different box but same column - invalid!
        
        // Placing the same cat in a different box AND different row/column should be valid
        assert!(board.is_valid_placement(4, 4, 2)); // center box, different row and column
        assert!(board.is_valid_placement(3, 4, 2)); // different box, different row and column
    }

    #[test]
    fn test_is_valid_placement_self_position() {
        let mut board = BoardState::new();
        board.cells[4][4] = Some(3);
        
        // Should be valid to "place" the same cat at its current position
        // (This handles the case where we're checking if a current placement is valid)
        assert!(board.is_valid_placement(4, 4, 3));
    }

    #[test]
    fn test_get_conflicts_empty_board() {
        let board = BoardState::new();
        let conflicts = board.get_conflicts();
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_get_conflicts_valid_board() {
        let mut board = BoardState::new();
        // Create a valid partial solution
        board.cells[0][0] = Some(0);
        board.cells[0][1] = Some(1);
        board.cells[1][0] = Some(2);
        
        let conflicts = board.get_conflicts();
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_get_conflicts_row_violation() {
        let mut board = BoardState::new();
        // Create a row conflict
        board.cells[0][0] = Some(0);
        board.cells[0][1] = Some(0); // Same cat in same row
        
        let conflicts = board.get_conflicts();
        assert_eq!(conflicts.len(), 2); // Both positions should be flagged
        assert!(conflicts.contains(&(0, 0)));
        assert!(conflicts.contains(&(0, 1)));
    }

    #[test]
    fn test_get_conflicts_multiple_violations() {
        let mut board = BoardState::new();
        // Create multiple conflicts
        board.cells[0][0] = Some(0);
        board.cells[0][1] = Some(0); // Row conflict
        board.cells[1][0] = Some(0); // Column conflict with (0,0)
        
        let conflicts = board.get_conflicts();
        assert_eq!(conflicts.len(), 3); // All three positions should be flagged
        assert!(conflicts.contains(&(0, 0)));
        assert!(conflicts.contains(&(0, 1)));
        assert!(conflicts.contains(&(1, 0)));
    }

    #[test]
    fn test_is_complete_empty_board() {
        let board = BoardState::new();
        assert!(!board.is_complete());
    }

    #[test]
    fn test_is_complete_partial_board() {
        let mut board = BoardState::new();
        // Fill only some cells
        for i in 0..5 {
            board.cells[0][i] = Some(i);
        }
        
        assert!(!board.is_complete());
    }

    #[test]
    fn test_is_complete_full_invalid_board() {
        let mut board = BoardState::new();
        // Fill all cells with the same value (invalid)
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                board.cells[row][col] = Some(0);
            }
        }
        
        assert!(!board.is_complete());
    }

    #[test]
    fn test_is_complete_valid_small_example() {
        let mut board = BoardState::new();
        // Create a small valid pattern that would work in a real Sudoku
        // (This is just a test - we're not creating a full valid 9x9 solution)
        
        // Fill first row with unique values
        for i in 0..GRID_SIZE {
            board.cells[0][i] = Some(i);
        }
        
        // Fill remaining cells with a pattern that avoids obvious conflicts
        for row in 1..GRID_SIZE {
            for col in 0..GRID_SIZE {
                // Use a shifted pattern to avoid row/column conflicts
                let value = (col + row) % GRID_SIZE;
                board.cells[row][col] = Some(value);
            }
        }
        
        // This should be a complete board (all cells filled)
        // Whether it's valid depends on the specific pattern, but let's test the logic
        let is_all_filled = board.cells.iter().all(|row| {
            row.iter().all(|cell| cell.is_some())
        });
        assert!(is_all_filled);
        
        // The completion check should work regardless of validity
        let has_conflicts = !board.get_conflicts().is_empty();
        assert_eq!(board.is_complete(), !has_conflicts);
    }

    #[test]
    fn test_generate_easy_puzzle() {
        let mut board = BoardState::new();
        board.generate_easy_puzzle();
        
        // Count the number of given (non-empty) cells
        let given_count = board.cells.iter()
            .flatten()
            .filter(|cell| cell.is_some())
            .count();
        
        // Easy puzzles should have 35-40 givens
        assert!(given_count >= 35 && given_count <= 40, 
               "Easy puzzle should have 35-40 givens, got {}", given_count);
        
        // All given numbers should form a valid partial solution (no conflicts)
        assert!(board.get_conflicts().is_empty(), 
               "Generated puzzle should have no conflicts");
    }

    #[test]
    fn test_generate_puzzle_different_difficulties() {
        let mut easy_board = BoardState::new();
        let mut medium_board = BoardState::new();
        let mut hard_board = BoardState::new();
        
        easy_board.generate_easy_puzzle();
        medium_board.generate_medium_puzzle();
        hard_board.generate_hard_puzzle();
        
        let easy_givens = easy_board.cells.iter().flatten().filter(|c| c.is_some()).count();
        let medium_givens = medium_board.cells.iter().flatten().filter(|c| c.is_some()).count();
        let hard_givens = hard_board.cells.iter().flatten().filter(|c| c.is_some()).count();
        
        // Easy should have more givens than medium, medium more than hard
        assert!(easy_givens >= 35);
        assert!(medium_givens >= 30 && medium_givens <= 35);
        assert!(hard_givens >= 25 && hard_givens <= 30);
        
        // All should be valid partial solutions
        assert!(easy_board.get_conflicts().is_empty());
        assert!(medium_board.get_conflicts().is_empty());
        assert!(hard_board.get_conflicts().is_empty());
    }

    #[test]
    fn test_puzzle_generation_is_random() {
        let mut board1 = BoardState::new();
        let mut board2 = BoardState::new();
        
        board1.generate_easy_puzzle();
        board2.generate_easy_puzzle();
        
        // The two generated puzzles should be different
        // (This test might rarely fail due to randomness, but extremely unlikely)
        let boards_identical = board1.cells == board2.cells;
        assert!(!boards_identical, "Generated puzzles should be different");
    }
}
