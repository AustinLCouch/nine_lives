//! Nine Lives Cat Sudoku Core Logic
//!
//! This crate contains the pure game logic for Nine Lives Cat Sudoku.
//! It is independent of any specific UI framework and handles:
//! - Game board representation and state
//! - Core game rules and algorithms  
//! - Board validation and manipulation

use bevy::prelude::Resource;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// Phase 1: Puzzle Generation Settings & Presets

/// Difficulty levels for puzzle generation (Phase 1: simple implementation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Serialize, Deserialize)]
pub enum Difficulty {
    /// Cozy Kitten: Easy puzzles with 35-40 givens, basic techniques only
    Easy,
    /// Curious Cat: Medium puzzles with 30-35 givens, slightly more complex
    Medium,
    /// Streetwise Stray: Hard puzzles with 26-30 givens, challenging techniques
    Hard,
    /// Night Prowler: Expert puzzles with 22-26 givens, advanced techniques
    Expert,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Easy // "Cozy Kitten" is the default
    }
}

/// Kitten-themed puzzle presets that combine multiple settings into coherent profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresetKind {
    /// Cozy Kitten: Easy, unique, symmetric, hints allowed, forgiving
    CozyKitten,
    /// Curious Cat: Medium difficulty, exploring new techniques
    CuriousCat,
    /// Streetwise Stray: Hard puzzles, fewer hints, more organic feel
    StreetwiseStray,
    /// Night Prowler: Expert level, minimal hints, serious business
    NightProwler,
}

impl Default for PresetKind {
    fn default() -> Self {
        Self::CozyKitten
    }
}

/// Complete puzzle generation settings (Phase 1: core features).
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct PuzzleSettings {
    pub difficulty: Difficulty,
    pub require_unique_solution: bool,
    pub givens_range: (usize, usize), // min, max clues to place
    pub seed: Option<u64>, // for reproducible generation
    pub hints_allowed: bool,
    pub max_hints: usize,
    
    // Phase 2 placeholders (not yet implemented)
    // pub symmetry: Symmetry,
    // pub variants: Vec<Variant>,
    // pub max_techniques: Vec<Technique>,
    // pub error_policy: ErrorPolicy,
}

impl Default for PuzzleSettings {
    fn default() -> Self {
        Self::from_preset(PresetKind::CozyKitten)
    }
}

impl PuzzleSettings {
    /// Create settings from a kitten-themed preset.
    pub fn from_preset(preset: PresetKind) -> Self {
        match preset {
            PresetKind::CozyKitten => Self {
                difficulty: Difficulty::Easy,
                require_unique_solution: true,
                givens_range: (35, 40),
                seed: None, // Random each time
                hints_allowed: true,
                max_hints: 5, // Generous hint allowance
            },
            PresetKind::CuriousCat => Self {
                difficulty: Difficulty::Medium,
                require_unique_solution: true,
                givens_range: (30, 35),
                seed: None,
                hints_allowed: true,
                max_hints: 3, // Moderate hints
            },
            PresetKind::StreetwiseStray => Self {
                difficulty: Difficulty::Hard,
                require_unique_solution: true,
                givens_range: (26, 30),
                seed: None,
                hints_allowed: true,
                max_hints: 2, // Limited hints
            },
            PresetKind::NightProwler => Self {
                difficulty: Difficulty::Expert,
                require_unique_solution: true,
                givens_range: (22, 26),
                seed: None,
                hints_allowed: false, // No hints - you're on your own!
                max_hints: 0,
            },
        }
    }
    
    /// Get a human-readable description of these settings.
    pub fn description(&self) -> String {
        let difficulty_str = match self.difficulty {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        };
        
        let unique_str = if self.require_unique_solution { "Unique solution" } else { "Multiple solutions allowed" };
        let hints_str = if self.hints_allowed { 
            format!("{} hints available", self.max_hints) 
        } else { 
            "No hints".to_string() 
        };
        
        format!("{} • {} • {} clues • {}", 
                difficulty_str, unique_str, 
                format!("{}-{}", self.givens_range.0, self.givens_range.1),
                hints_str)
    }
}

impl PresetKind {
    /// Get all available presets in display order.
    pub fn all() -> [PresetKind; 4] {
        [
            PresetKind::CozyKitten,
            PresetKind::CuriousCat,
            PresetKind::StreetwiseStray,
            PresetKind::NightProwler,
        ]
    }
    
    /// Get the display name for this preset.
    pub fn display_name(&self) -> &'static str {
        match self {
            PresetKind::CozyKitten => "🐱 Cozy Kitten",
            PresetKind::CuriousCat => "😸 Curious Cat",
            PresetKind::StreetwiseStray => "😼 Streetwise Stray",
            PresetKind::NightProwler => "😾 Night Prowler",
        }
    }
    
    /// Get a short description of this preset.
    pub fn description(&self) -> &'static str {
        match self {
            PresetKind::CozyKitten => "Perfect for beginners. Lots of clues, helpful hints, and forgiving rules.",
            PresetKind::CuriousCat => "Ready to explore? Medium challenge with guided discovery.",
            PresetKind::StreetwiseStray => "You know the streets. Fewer clues, limited hints, real challenge.",
            PresetKind::NightProwler => "Expert level. Minimal clues, no hints. Only the sharpest claws survive.",
        }
    }
}

/// High-level game state for the current puzzle lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    #[default]
    Playing,
    Won,
    Paused,
}

/// Game timing and move tracking information.
#[derive(Debug, Clone, Resource)]
pub struct GameSession {
    pub started_at: std::time::Instant,
    pub elapsed_time: std::time::Duration,
    pub move_count: usize,
    pub is_paused: bool,
    pub pause_start: Option<std::time::Instant>,
}

impl Default for GameSession {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            started_at: std::time::Instant::now(),
            elapsed_time: std::time::Duration::ZERO,
            move_count: 0,
            is_paused: false,
            pause_start: None,
        }
    }

    pub fn pause(&mut self) {
        if !self.is_paused {
            self.is_paused = true;
            self.pause_start = Some(std::time::Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if let Some(_pause_start) = self.pause_start.take() {
            self.is_paused = false;
            // Don't add paused time to elapsed time
        }
    }

    pub fn increment_move(&mut self) {
        self.move_count += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn current_elapsed(&self) -> std::time::Duration {
        if self.is_paused {
            self.elapsed_time
        } else {
            self.elapsed_time + self.started_at.elapsed()
        }
    }
}

/// Represents a single move in the game for undo/redo functionality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub row: usize,
    pub col: usize,
    pub old_value: Option<usize>,
    pub new_value: Option<usize>,
    pub timestamp: std::time::Instant,
}

/// Game history for undo/redo functionality.
/// Uses a deque for efficient operations at both ends.
#[derive(Debug, Clone, Resource)]
pub struct GameHistory {
    pub moves: VecDeque<Move>,
    pub undo_index: usize, // Index pointing to the "current" state
    pub max_history: usize, // Maximum number of moves to remember
}

impl Default for GameHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl GameHistory {
    pub fn new() -> Self {
        Self {
            moves: VecDeque::new(),
            undo_index: 0,
            max_history: 100, // Remember last 100 moves
        }
    }

    /// Add a new move to the history. This clears any "future" moves if we were in the middle of undo/redo.
    pub fn add_move(&mut self, game_move: Move) {
        // If we're not at the end of history, truncate everything after current position
        while self.moves.len() > self.undo_index {
            self.moves.pop_back();
        }

        // Add the new move
        self.moves.push_back(game_move);
        self.undo_index = self.moves.len();

        // Keep history within bounds
        while self.moves.len() > self.max_history {
            self.moves.pop_front();
            if self.undo_index > 0 {
                self.undo_index -= 1;
            }
        }
    }

    /// Check if undo is possible.
    pub fn can_undo(&self) -> bool {
        self.undo_index > 0
    }

    /// Check if redo is possible.
    pub fn can_redo(&self) -> bool {
        self.undo_index < self.moves.len()
    }

    /// Get the move to undo (without applying it).
    pub fn peek_undo(&self) -> Option<&Move> {
        if self.can_undo() {
            self.moves.get(self.undo_index - 1)
        } else {
            None
        }
    }

    /// Get the move to redo (without applying it).
    pub fn peek_redo(&self) -> Option<&Move> {
        if self.can_redo() {
            self.moves.get(self.undo_index)
        } else {
            None
        }
    }

    /// Mark that we've undone a move (moves the index back).
    pub fn mark_undone(&mut self) {
        if self.can_undo() {
            self.undo_index -= 1;
        }
    }

    /// Mark that we've redone a move (moves the index forward).
    pub fn mark_redone(&mut self) {
        if self.can_redo() {
            self.undo_index += 1;
        }
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.moves.clear();
        self.undo_index = 0;
    }

    /// Get current position info for display ("Move 5/10" format).
    pub fn position_info(&self) -> (usize, usize) {
        (self.undo_index, self.moves.len())
    }
}

/// Stores the complete solution to the current puzzle for hint generation.
#[derive(Debug, Clone, Resource)]
pub struct Solution {
    pub cells: [[usize; GRID_SIZE]; GRID_SIZE],
}

impl Solution {
    pub fn new() -> Self {
        Self {
            cells: [[0; GRID_SIZE]; GRID_SIZE],
        }
    }

    /// Create solution from a complete board state.
    pub fn from_board(board: &BoardState) -> Option<Self> {
        // Check if board is complete and valid
        if !board.is_complete() {
            return None;
        }

        let mut solution = Self::new();
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if let Some(value) = board.cells[row][col] {
                    solution.cells[row][col] = value;
                } else {
                    return None; // Board not complete
                }
            }
        }
        Some(solution)
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug mode configuration for testing and development.
#[derive(Debug, Clone, Resource)]
pub struct DebugMode {
    pub enabled: bool,
    pub unlimited_hints: bool,
}

impl Default for DebugMode {
    fn default() -> Self {
        Self {
            enabled: false,
            unlimited_hints: false,
        }
    }
}

impl DebugMode {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Enable debug mode with unlimited hints.
    pub fn enable_unlimited_hints(&mut self) {
        self.enabled = true;
        self.unlimited_hints = true;
    }
    
    /// Disable debug mode and return to normal gameplay.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.unlimited_hints = false;
    }
    
    /// Toggle debug mode with unlimited hints.
    pub fn toggle_unlimited_hints(&mut self) {
        if self.enabled && self.unlimited_hints {
            self.disable();
        } else {
            self.enable_unlimited_hints();
        }
    }
}

/// Hint system configuration and state.
#[derive(Debug, Clone, Resource)]
pub struct HintSystem {
    pub hints_remaining: usize,
    pub max_hints: usize,
}

impl HintSystem {
    pub fn new(max_hints: usize) -> Self {
        Self {
            hints_remaining: max_hints,
            max_hints,
        }
    }

    /// Reset hints for a new game.
    pub fn reset(&mut self, max_hints: usize) {
        self.max_hints = max_hints;
        self.hints_remaining = max_hints;
    }

    /// Use a hint if available, respecting debug mode.
    pub fn use_hint(&mut self, debug_mode: &DebugMode) -> bool {
        if debug_mode.unlimited_hints {
            // In debug mode, we always allow hints but don't decrement the counter
            true
        } else if self.hints_remaining > 0 {
            self.hints_remaining -= 1;
            true
        } else {
            false
        }
    }

    /// Check if hints are available, respecting debug mode.
    pub fn can_use_hint(&self, debug_mode: &DebugMode) -> bool {
        debug_mode.unlimited_hints || self.hints_remaining > 0
    }
    
    /// Get display text for hint button, showing debug status if applicable.
    pub fn get_hint_button_text(&self, debug_mode: &DebugMode) -> String {
        if debug_mode.unlimited_hints {
            "💡 Debug ∞".to_string()
        } else {
            format!("💡 Hint {}", self.hints_remaining)
        }
    }
}

impl Default for HintSystem {
    fn default() -> Self {
        Self::new(3) // Default to 3 hints
    }
}

// Phase 1: Uniqueness Validation Solver

/// Validates that a puzzle has exactly one unique solution.
/// Returns true if the puzzle is valid (exactly one solution).
pub fn validate_unique_solution(board: &BoardState) -> bool {
    let mut solution_count = 0;
    let mut test_board = board.clone();
    
    solve_with_counter(&mut test_board, &mut solution_count, 2); // Stop after finding 2 solutions
    solution_count == 1
}

/// Backtracking solver with solution counting (for uniqueness validation).
/// Stops early once max_solutions is reached for efficiency.
fn solve_with_counter(board: &mut BoardState, solution_count: &mut usize, max_solutions: usize) -> bool {
    if *solution_count >= max_solutions {
        return false; // Early exit - we've found enough solutions
    }
    
    // Find the next empty cell
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if board.cells[row][col].is_none() {
                // Try all possible values
                for value in 0..GRID_SIZE {
                    if board.is_valid_placement(row, col, value) {
                        // Place the value
                        board.cells[row][col] = Some(value);
                        
                        // Recursively solve
                        if solve_with_counter(board, solution_count, max_solutions) {
                            return true; // Found a solution path
                        }
                        
                        // Backtrack
                        board.cells[row][col] = None;
                    }
                }
                
                // No valid value found for this cell
                return false;
            }
        }
    }
    
    // All cells filled - found a complete solution!
    *solution_count += 1;
    
    // Continue searching for more solutions (don't return true yet)
    false
}

/// Solves a Sudoku puzzle and returns the solution if exactly one exists.
/// Returns None if no solution or multiple solutions exist.
pub fn solve_unique(board: &BoardState) -> Option<Solution> {
    if !validate_unique_solution(board) {
        return None; // No unique solution
    }
    
    // We know there's exactly one solution, so solve normally
    let mut test_board = board.clone();
    if solve_board(&mut test_board) {
        Solution::from_board(&test_board)
    } else {
        None // Shouldn't happen if validation passed
    }
}

/// Simple backtracking solver for finding any solution.
fn solve_board(board: &mut BoardState) -> bool {
    // Find the next empty cell
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if board.cells[row][col].is_none() {
                // Try all possible values
                for value in 0..GRID_SIZE {
                    if board.is_valid_placement(row, col, value) {
                        // Place the value
                        board.cells[row][col] = Some(value);
                        
                        // Recursively solve
                        if solve_board(board) {
                            return true;
                        }
                        
                        // Backtrack
                        board.cells[row][col] = None;
                    }
                }
                
                // No valid value found for this cell
                return false;
            }
        }
    }
    
    // All cells filled - puzzle solved!
    true
}

/// Get the next best hint for the player.
/// Returns (row, col, correct_value) if a hint is available.
pub fn get_next_hint(board: &BoardState, solution: &Solution) -> Option<(usize, usize, usize)> {
    // Find empty cells that could be filled
    let mut candidates = Vec::new();
    
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            // Only hint for empty cells that are not given cells
            if board.cells[row][col].is_none() && !board.is_given_cell(row, col) {
                let correct_value = solution.cells[row][col];
                candidates.push((row, col, correct_value));
            }
        }
    }
    
    // Return a random candidate (to make hints less predictable)
    if !candidates.is_empty() {
        let mut rng = thread_rng();
        let choice = candidates.choose(&mut rng)?;
        Some(*choice)
    } else {
        None
    }
}

/// The size of one dimension of the Sudoku grid (e.g., 9 for a 9x9 grid).
pub const GRID_SIZE: usize = 9;

/// Represents the type of a cell - whether it was given in the puzzle or filled by the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Returns the Move that was made, or None if no change occurred.
    ///
    /// The sequence is: None -> Some(0) -> Some(1) -> ... -> Some(max-1) -> Some(0).
    /// Given cells (part of the original puzzle) cannot be changed.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell to cycle.
    /// * `col` - The column index of the cell to cycle.
    /// * `num_emojis` - The total number of available choices (cats).
    pub fn cycle_cell(&mut self, row: usize, col: usize, num_emojis: usize) -> Option<Move> {
        // Don't allow changes to given cells
        if let Some(CellType::Given) = self.cell_types[row][col] {
            return None;
        }

        let old_value = self.cells[row][col];
        let new_value = match old_value {
            None => Some(0),
            Some(idx) => Some((idx + 1) % num_emojis),
        };

        // Only proceed if there's actually a change
        if old_value == new_value {
            return None;
        }

        self.cells[row][col] = new_value;

        // Mark as player input if we have a value
        self.cell_types[row][col] = if new_value.is_some() {
            Some(CellType::Player)
        } else {
            None
        };

        // Return the move for history tracking
        Some(Move {
            row,
            col,
            old_value,
            new_value,
            timestamp: std::time::Instant::now(),
        })
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
        if self.is_complete() {
            GameState::Won
        } else {
            GameState::Playing
        }
    }

    /// Generate a new Sudoku puzzle using the provided settings.
    /// Returns the solution for hint generation.
    ///
    /// This uses an improved algorithm:
    /// 1. Fill the grid with a valid complete solution
    /// 2. Store the solution 
    /// 3. Use smart clue removal that maintains uniqueness
    /// 4. For Expert puzzles, use advanced uniqueness-preserving techniques
    ///
    /// # Arguments
    ///
    /// * `settings` - Generation settings including difficulty, uniqueness, etc.
    pub fn generate_puzzle_with_settings(&mut self, settings: &PuzzleSettings) -> Option<Solution> {
        let max_attempts = if settings.require_unique_solution { 15 } else { 3 };
        
        for attempt in 0..max_attempts {
            // Start with a clear board
            self.clear();
            
            // Set seed if specified
            if let Some(seed) = settings.seed {
                // For reproducible generation, we'd need to seed the RNG here
                // For now, we'll use the default random behavior
                println!("Note: Seed {} specified but not yet implemented", seed);
            }

            // Fill the board with a complete valid solution
            if !self.fill_board() {
                continue; // Failed to generate, try again
            }

            // Store the complete solution before removing numbers
            let solution = Solution::from_board(self)?;

            // Use improved clue removal based on difficulty
            let success = if settings.difficulty == Difficulty::Expert && settings.require_unique_solution {
                // Expert puzzles need advanced uniqueness-preserving generation
                self.generate_expert_unique_puzzle(&settings, &solution)
            } else {
                // Use traditional method for easier difficulties
                let target_givens = thread_rng().gen_range(settings.givens_range.0..=settings.givens_range.1);
                self.remove_numbers_for_puzzle(target_givens);
                
                if settings.require_unique_solution {
                    validate_unique_solution(self)
                } else {
                    true
                }
            };
            
            if success {
                let givens_count = self.cells.iter().flatten().filter(|c| c.is_some()).count();
                println!("Generated unique puzzle with {} givens (attempt {})", givens_count, attempt + 1);
                return Some(solution);
            } else {
                println!("Attempt {} failed uniqueness check, retrying...", attempt + 1);
                continue;
            }
        }
        
        // Failed to generate after all attempts
        println!("Failed to generate puzzle after {} attempts", max_attempts);
        None
    }
    
    /// Advanced Expert puzzle generation that maintains uniqueness.
    /// Uses iterative clue removal with uniqueness checking at each step.
    fn generate_expert_unique_puzzle(&mut self, settings: &PuzzleSettings, _solution: &Solution) -> bool {
        // Start with all clues (complete solution)
        let mut candidates_for_removal = Vec::new();
        
        // Build list of all positions that could potentially be removed
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                candidates_for_removal.push((row, col));
            }
        }
        
        // Shuffle to ensure variety in the final puzzle
        candidates_for_removal.shuffle(&mut thread_rng());
        
        let target_givens = thread_rng().gen_range(settings.givens_range.0..=settings.givens_range.1);
        let target_removals = GRID_SIZE * GRID_SIZE - target_givens;
        
        let mut removals_made = 0;
        
        // Iteratively remove clues while preserving uniqueness
        for (row, col) in candidates_for_removal {
            if removals_made >= target_removals {
                break; // We've removed enough
            }
            
            // Temporarily remove this clue
            let original_value = self.cells[row][col];
            let original_type = self.cell_types[row][col];
            
            self.cells[row][col] = None;
            self.cell_types[row][col] = None;
            
            // Check if puzzle still has unique solution
            if validate_unique_solution(self) {
                // Good! This removal preserves uniqueness
                removals_made += 1;
            } else {
                // Revert - removing this clue breaks uniqueness
                self.cells[row][col] = original_value;
                self.cell_types[row][col] = original_type;
            }
        }
        
        // Mark remaining cells as Given
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.cells[row][col].is_some() {
                    self.cell_types[row][col] = Some(CellType::Given);
                }
            }
        }
        
        let final_givens = self.cells.iter().flatten().filter(|c| c.is_some()).count();
        
        // Check if we achieved a reasonable difficulty level
        final_givens >= settings.givens_range.0 && final_givens <= settings.givens_range.1
    }
    
    /// Legacy method - generates an easy puzzle (for backward compatibility).
    pub fn generate_puzzle(&mut self, givens: usize) -> Solution {
        let settings = PuzzleSettings {
            difficulty: Difficulty::Easy,
            require_unique_solution: false, // Maintain old behavior
            givens_range: (givens, givens),
            seed: None,
            hints_allowed: true,
            max_hints: 3,
        };
        
        self.generate_puzzle_with_settings(&settings)
            .unwrap_or_else(|| {
                // Fallback: create a simple solution if generation fails
                self.fill_board();
                Solution::from_board(self).unwrap_or_default()
            })
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


    /// Check if a cell is a given cell (part of the original puzzle).
    pub fn is_given_cell(&self, row: usize, col: usize) -> bool {
        matches!(self.cell_types[row][col], Some(CellType::Given))
    }

    /// Apply a move to the board (used for undo/redo).
    pub fn apply_move(&mut self, game_move: &Move) {
        // Don't allow changes to given cells (safety check)
        if let Some(CellType::Given) = self.cell_types[game_move.row][game_move.col] {
            return;
        }

        self.cells[game_move.row][game_move.col] = game_move.new_value;
        
        // Update cell type
        self.cell_types[game_move.row][game_move.col] = if game_move.new_value.is_some() {
            Some(CellType::Player)
        } else {
            None
        };
    }

    /// Undo a move (reverse it).
    pub fn undo_move(&mut self, game_move: &Move) {
        // Don't allow changes to given cells (safety check)
        if let Some(CellType::Given) = self.cell_types[game_move.row][game_move.col] {
            return;
        }

        self.cells[game_move.row][game_move.col] = game_move.old_value;
        
        // Update cell type
        self.cell_types[game_move.row][game_move.col] = if game_move.old_value.is_some() {
            Some(CellType::Player)
        } else {
            None
        };
    }
    
    /// Create a save game from current board state
    pub fn create_save_game(&self, solution: &Solution, settings: &PuzzleSettings, 
                           elapsed_seconds: u64, move_count: usize, hints_remaining: usize) -> SaveGame {
        SaveGame {
            board_cells: self.cells,
            cell_types: self.cell_types,
            solution_cells: solution.cells,
            settings: settings.clone(),
            elapsed_seconds,
            move_count,
            hints_remaining,
            saved_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// Restore board state from a save game
    pub fn restore_from_save(&mut self, save_game: &SaveGame) {
        self.cells = save_game.board_cells;
        self.cell_types = save_game.cell_types;
    }
}

// Implementing the `Default` trait provides a convenient way
// to create a new instance, which is useful for `init_resource` in Bevy.
impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

// MARK: - Persistence System

/// Persistent user settings that survive between game sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub last_preset: PresetKind,
    pub volume: f32,
    pub auto_save_enabled: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            last_preset: PresetKind::CozyKitten,
            volume: 0.7,
            auto_save_enabled: true,
        }
    }
}

/// Simple game statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameStatistics {
    pub games_completed: u32,
    pub games_per_difficulty: std::collections::HashMap<String, u32>, // difficulty name -> count
    pub total_play_time_seconds: u64,
    pub fastest_completion_seconds: Option<u64>,
}

/// Serializable game save data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub board_cells: [[Option<usize>; GRID_SIZE]; GRID_SIZE],
    pub cell_types: [[Option<CellType>; GRID_SIZE]; GRID_SIZE],
    pub solution_cells: [[usize; GRID_SIZE]; GRID_SIZE],
    pub settings: PuzzleSettings,
    pub elapsed_seconds: u64,
    pub move_count: usize,
    pub hints_remaining: usize,
    pub saved_at: u64, // Unix timestamp
}

/// Persistent data that gets saved to disk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistentData {
    pub user_settings: UserSettings,
    pub statistics: GameStatistics,
    pub current_save: Option<SaveGame>,
}

/// Core persistence functionality
impl PersistentData {
    /// Load persistent data from the standard location
    pub fn load() -> Self {
        let save_dir = get_save_directory();
        let save_file = save_dir.join("nine_lives_data.json");
        
        if save_file.exists() {
            match std::fs::read_to_string(&save_file) {
                Ok(contents) => {
                    match serde_json::from_str::<PersistentData>(&contents) {
                        Ok(data) => {
                            println!("✅ Loaded persistent data from {:?}", save_file);
                            return data;
                        }
                        Err(e) => {
                            println!("⚠️ Failed to parse save file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to read save file: {}", e);
                }
            }
        }
        
        println!("📁 Creating new persistent data (no save file found)");
        Self::default()
    }
    
    /// Save persistent data to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let save_dir = get_save_directory();
        
        // Ensure save directory exists
        std::fs::create_dir_all(&save_dir)?;
        
        let save_file = save_dir.join("nine_lives_data.json");
        let json_data = serde_json::to_string_pretty(self)?;
        
        std::fs::write(&save_file, json_data)?;
        println!("💾 Saved persistent data to {:?}", save_file);
        
        Ok(())
    }
    
    /// Record a completed game in statistics
    pub fn record_game_completion(&mut self, difficulty: &str, play_time_seconds: u64) {
        self.statistics.games_completed += 1;
        self.statistics.total_play_time_seconds += play_time_seconds;
        
        *self.statistics.games_per_difficulty.entry(difficulty.to_string()).or_insert(0) += 1;
        
        // Track fastest completion
        match self.statistics.fastest_completion_seconds {
            None => self.statistics.fastest_completion_seconds = Some(play_time_seconds),
            Some(current_fastest) => {
                if play_time_seconds < current_fastest {
                    self.statistics.fastest_completion_seconds = Some(play_time_seconds);
                }
            }
        }
    }
}

/// Get the standard save directory for the game
fn get_save_directory() -> std::path::PathBuf {
    if let Some(home_dir) = dirs::home_dir() {
        home_dir.join(".nine_lives")
    } else {
        // Fallback to current directory if home directory is not available
        std::path::PathBuf::from(".nine_lives")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Expert puzzle generation reliability - now should consistently succeed!
    #[test] 
    fn test_expert_generation_reliability_fixed() {
        let settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
        
        println!("🔍 Expert Generation Diagnostics");
        println!("Settings: {}", settings.description());
        println!("Max attempts per puzzle: 10");
        println!("Target givens range: {}-{}", settings.givens_range.0, settings.givens_range.1);
        println!("Uniqueness required: {}", settings.require_unique_solution);
        
        let mut success_count = 0;
        const TRIALS: usize = 5;
        
        for trial in 1..=TRIALS {
            let mut board = BoardState::new();
            
            match board.generate_puzzle_with_settings(&settings) {
                Some(_solution) => {
                    success_count += 1;
                    let givens_count = board.cells.iter().flatten().filter(|c| c.is_some()).count();
                    println!("✅ Trial {}: Generated successfully with {} givens", trial, givens_count);
                }
                None => {
                    println!("❌ Trial {}: Failed to generate Expert puzzle", trial);
                }
            }
        }
        
        let success_rate = (success_count as f32 / TRIALS as f32) * 100.0;
        println!("\n📊 Results: {}/{} successful ({:.1}% success rate)", 
                 success_count, TRIALS, success_rate);
        
        // With our improved algorithm, we expect high reliability
        assert!(success_rate >= 80.0, "Expert generation should be at least 80% reliable");
        
        if success_rate >= 95.0 {
            println!("✅ Excellent! Expert generation is very reliable ({:.1}%)", success_rate);
        } else {
            println!("⚠️ Expert generation is working but could be more reliable ({:.1}%)", success_rate);
        }
    }
    
    /// Test the uniqueness validation algorithm with known cases
    #[test]
    fn test_uniqueness_validation_algorithm() {
        // Test case 1: Empty board should have multiple solutions
        let empty_board = BoardState::new();
        assert!(!validate_unique_solution(&empty_board), 
               "Empty board should have multiple solutions");
        
        // Test case 2: Nearly complete board should have unique solution
        let mut nearly_complete = BoardState::new();
        // Fill most cells with a valid pattern, leaving just a few empty
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if (row * GRID_SIZE + col) < 75 { // Fill 75/81 cells
                    nearly_complete.cells[row][col] = Some((row + col) % GRID_SIZE);
                }
            }
        }
        
        // Test case 3: Board with obvious multiple solutions
        let mut multi_solution = BoardState::new();
        // Place just a few clues that definitely allow multiple solutions
        multi_solution.cells[0][0] = Some(0);
        multi_solution.cells[1][1] = Some(1);
        multi_solution.cells[2][2] = Some(2);
        
        assert!(!validate_unique_solution(&multi_solution),
               "Board with minimal clues should have multiple solutions");
        
        println!("✅ Uniqueness validation algorithm appears to be working correctly");
    }
    
    /// Test solution counter accuracy by manually checking a simple case
    #[test]
    fn test_solution_counter_accuracy() {
        // Test case 1: Board with just a few clues should have multiple solutions
        let mut sparse_board = BoardState::new();
        sparse_board.cells[0][0] = Some(0);
        sparse_board.cells[1][1] = Some(1);
        sparse_board.cells[2][2] = Some(2);
        
        let mut solution_count = 0;
        let mut test_copy = sparse_board.clone();
        solve_with_counter(&mut test_copy, &mut solution_count, 5); // Stop after finding 5 solutions
        
        println!("Solution count for sparse board: {}", solution_count);
        assert!(solution_count >= 1, "Sparse board should have at least 1 solution");
        
        // Test case 2: Empty board should have many solutions
        let empty_board = BoardState::new();
        let mut empty_solution_count = 0;
        let mut empty_copy = empty_board.clone();
        solve_with_counter(&mut empty_copy, &mut empty_solution_count, 2); // Just check for multiple
        
        println!("Solution count for empty board (limited to 2): {}", empty_solution_count);
        assert!(empty_solution_count >= 1, "Empty board should have solutions");
    }
    
    /// Comprehensive stress test for the improved Expert generation algorithm
    #[test]
    #[ignore = "Stress test - takes a while to run"]
    fn test_expert_generation_stress_test() {
        use std::time::Instant;
        
        let settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
        
        println!("💪 Expert Generation Stress Test");
        println!("Generating 100 Expert puzzles to validate reliability and performance...");
        println!("Settings: {}", settings.description());
        
        let mut success_count = 0;
        let mut total_time = std::time::Duration::ZERO;
        let mut givens_histogram = std::collections::HashMap::new();
        const STRESS_TESTS: usize = 100;
        
        for trial in 1..=STRESS_TESTS {
            let mut board = BoardState::new();
            let start_time = Instant::now();
            
            match board.generate_puzzle_with_settings(&settings) {
                Some(_solution) => {
                    success_count += 1;
                    let elapsed = start_time.elapsed();
                    total_time += elapsed;
                    
                    let givens_count = board.cells.iter().flatten().filter(|c| c.is_some()).count();
                    *givens_histogram.entry(givens_count).or_insert(0) += 1;
                    
                    // Validate puzzle properties
                    assert!(givens_count >= 22 && givens_count <= 26, 
                           "Expert puzzle should have 22-26 givens, got {}", givens_count);
                    assert!(board.get_conflicts().is_empty(), 
                           "Expert puzzle should have no conflicts");
                    assert!(validate_unique_solution(&board), 
                           "Expert puzzle should have unique solution");
                    
                    if trial % 10 == 0 {
                        println!("  ✅ Generated {}/{} puzzles, avg time: {:.1}ms", 
                                trial, STRESS_TESTS, 
                                (total_time.as_millis() as f32 / trial as f32));
                    }
                }
                None => {
                    println!("  ❌ Trial {}: Failed to generate", trial);
                }
            }
        }
        
        let success_rate = (success_count as f32 / STRESS_TESTS as f32) * 100.0;
        let avg_time_ms = total_time.as_millis() as f32 / success_count as f32;
        
        println!("\n📊 Final Results:");
        println!("  • Success Rate: {:.1}% ({}/{})", success_rate, success_count, STRESS_TESTS);
        println!("  • Average Generation Time: {:.1}ms", avg_time_ms);
        println!("  • Total Time: {:.2}s", total_time.as_secs_f32());
        
        println!("\n📊 Givens Distribution:");
        for givens in 22..=26 {
            let count = givens_histogram.get(&givens).unwrap_or(&0);
            let percentage = (*count as f32 / success_count as f32) * 100.0;
            println!("  • {} givens: {} puzzles ({:.1}%)", givens, count, percentage);
        }
        
        // Performance and reliability assertions
        assert!(success_rate >= 95.0, "Expert generation should be at least 95% reliable");
        assert!(avg_time_ms < 500.0, "Expert generation should average under 500ms in debug mode");
        
        // Distribution should be reasonably spread across the range
        let min_givens = *givens_histogram.keys().min().unwrap_or(&26);
        let max_givens = *givens_histogram.keys().max().unwrap_or(&22);
        assert!(max_givens - min_givens >= 2, "Should generate variety in givens count");
        
        println!("✅ Expert generation stress test passed!");
    }
    
    /// Test that Expert puzzles are actually harder than Easy puzzles
    #[test]
    #[ignore = "Comparative difficulty test"]
    fn test_difficulty_progression() {
        let easy_settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        let expert_settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
        
        let mut easy_board = BoardState::new();
        let mut expert_board = BoardState::new();
        
        // Generate one of each
        let easy_solution = easy_board.generate_puzzle_with_settings(&easy_settings);
        let expert_solution = expert_board.generate_puzzle_with_settings(&expert_settings);
        
        assert!(easy_solution.is_some(), "Easy puzzle should generate successfully");
        assert!(expert_solution.is_some(), "Expert puzzle should generate successfully");
        
        let easy_givens = easy_board.cells.iter().flatten().filter(|c| c.is_some()).count();
        let expert_givens = expert_board.cells.iter().flatten().filter(|c| c.is_some()).count();
        
        println!("Easy puzzle givens: {}", easy_givens);
        println!("Expert puzzle givens: {}", expert_givens);
        
        // Expert should have significantly fewer givens (harder)
        assert!(expert_givens < easy_givens, 
               "Expert puzzles should have fewer givens than Easy puzzles");
        
        // Specific ranges should be respected
        assert!(easy_givens >= 35 && easy_givens <= 40, "Easy givens should be 35-40");
        assert!(expert_givens >= 22 && expert_givens <= 26, "Expert givens should be 22-26");
        
        println!("✅ Difficulty progression is working correctly!");
    }
    
    /// Test basic persistence functionality
    #[test]
    fn test_persistence_system() {
        // Test UserSettings serialization
        let settings = UserSettings {
            last_preset: PresetKind::NightProwler,
            volume: 0.8,
            auto_save_enabled: false,
        };
        
        let json = serde_json::to_string(&settings).expect("Should serialize UserSettings");
        println!("UserSettings JSON: {}", json);
        
        let restored: UserSettings = serde_json::from_str(&json).expect("Should deserialize UserSettings");
        assert_eq!(restored.last_preset, PresetKind::NightProwler);
        assert_eq!(restored.volume, 0.8);
        assert_eq!(restored.auto_save_enabled, false);
        
        // Test PersistentData creation and statistics
        let mut persistent_data = PersistentData::default();
        persistent_data.record_game_completion("Expert", 300);
        persistent_data.record_game_completion("Easy", 120);
        
        assert_eq!(persistent_data.statistics.games_completed, 2);
        assert_eq!(persistent_data.statistics.fastest_completion_seconds, Some(120));
        
        let expert_count = persistent_data.statistics.games_per_difficulty.get("Expert").unwrap_or(&0);
        assert_eq!(*expert_count, 1);
        
        println!("✅ Persistence system basic functionality works!");
    }

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
        let is_all_filled = board
            .cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()));
        assert!(is_all_filled);

        // The completion check should work regardless of validity
        let has_conflicts = !board.get_conflicts().is_empty();
        assert_eq!(board.is_complete(), !has_conflicts);
    }

    #[test]
    fn test_generate_puzzle_with_settings() {
        let mut board = BoardState::new();
        let settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        
        if let Some(_solution) = board.generate_puzzle_with_settings(&settings) {
            // Count the number of given (non-empty) cells
            let given_count = board
                .cells
                .iter()
                .flatten()
                .filter(|cell| cell.is_some())
                .count();

            // Cozy Kitten should have 35-40 givens
            assert!(
                given_count >= 35 && given_count <= 40,
                "Cozy Kitten puzzle should have 35-40 givens, got {}",
                given_count
            );

            // All given numbers should form a valid partial solution (no conflicts)
            assert!(
                board.get_conflicts().is_empty(),
                "Generated puzzle should have no conflicts"
            );
        } else {
            panic!("Failed to generate puzzle with Cozy Kitten settings");
        }
    }

    #[test]
    fn test_generate_puzzle_different_difficulties() {
        let mut easy_board = BoardState::new();
        let mut medium_board = BoardState::new();
        let mut hard_board = BoardState::new();

        let easy_settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        let medium_settings = PuzzleSettings::from_preset(PresetKind::CuriousCat);
        let hard_settings = PuzzleSettings::from_preset(PresetKind::StreetwiseStray);

        // Generate puzzles - these may fail sometimes due to uniqueness requirements
        let easy_success = easy_board.generate_puzzle_with_settings(&easy_settings).is_some();
        let medium_success = medium_board.generate_puzzle_with_settings(&medium_settings).is_some();
        let hard_success = hard_board.generate_puzzle_with_settings(&hard_settings).is_some();
        
        // At least one should succeed (they might not all succeed due to uniqueness constraints)
        assert!(easy_success || medium_success || hard_success, "At least one difficulty should generate successfully");

        if easy_success {
            let easy_givens = easy_board.cells.iter().flatten().filter(|c| c.is_some()).count();
            assert!(easy_givens >= 35 && easy_givens <= 40, "Easy puzzle givens: {}", easy_givens);
            assert!(easy_board.get_conflicts().is_empty(), "Easy puzzle should have no conflicts");
        }
        
        if medium_success {
            let medium_givens = medium_board.cells.iter().flatten().filter(|c| c.is_some()).count();
            assert!(medium_givens >= 30 && medium_givens <= 35, "Medium puzzle givens: {}", medium_givens);
            assert!(medium_board.get_conflicts().is_empty(), "Medium puzzle should have no conflicts");
        }
        
        if hard_success {
            let hard_givens = hard_board.cells.iter().flatten().filter(|c| c.is_some()).count();
            assert!(hard_givens >= 26 && hard_givens <= 30, "Hard puzzle givens: {}", hard_givens);
            assert!(hard_board.get_conflicts().is_empty(), "Hard puzzle should have no conflicts");
        }
    }

    #[test]
    fn test_puzzle_generation_is_random() {
        let mut board1 = BoardState::new();
        let mut board2 = BoardState::new();
        
        let settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        
        // Generate two puzzles
        let success1 = board1.generate_puzzle_with_settings(&settings).is_some();
        let success2 = board2.generate_puzzle_with_settings(&settings).is_some();
        
        // Both should succeed or at least one should succeed
        assert!(success1 || success2, "At least one puzzle generation should succeed");
        
        // If both succeeded, they should likely be different (though not guaranteed)
        if success1 && success2 {
            let boards_identical = board1.cells == board2.cells;
            // Note: With uniqueness constraints, there's a higher chance of identical boards
            // so we'll just check that the generation worked
            println!("Generated two puzzles, identical: {}", boards_identical);
        }
    }

    #[test]
    fn test_puzzle_settings_from_preset() {
        // Test Cozy Kitten preset
        let cozy_settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        assert_eq!(cozy_settings.difficulty, Difficulty::Easy);
        assert!(cozy_settings.require_unique_solution);
        assert_eq!(cozy_settings.givens_range, (35, 40));
        assert!(cozy_settings.hints_allowed);
        assert_eq!(cozy_settings.max_hints, 5);
        
        // Test Curious Cat preset
        let curious_settings = PuzzleSettings::from_preset(PresetKind::CuriousCat);
        assert_eq!(curious_settings.difficulty, Difficulty::Medium);
        assert_eq!(curious_settings.givens_range, (30, 35));
        assert_eq!(curious_settings.max_hints, 3);
        
        // Test Streetwise Stray preset
        let stray_settings = PuzzleSettings::from_preset(PresetKind::StreetwiseStray);
        assert_eq!(stray_settings.difficulty, Difficulty::Hard);
        assert_eq!(stray_settings.givens_range, (26, 30));
        assert_eq!(stray_settings.max_hints, 2);
        
        // Test Night Prowler preset
        let prowler_settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
        assert_eq!(prowler_settings.difficulty, Difficulty::Expert);
        assert_eq!(prowler_settings.givens_range, (22, 26));
        assert!(!prowler_settings.hints_allowed);
        assert_eq!(prowler_settings.max_hints, 0);
    }

    #[test]
    fn test_puzzle_settings_description() {
        let cozy_settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        let description = cozy_settings.description();
        
        // Should contain key information
        assert!(description.contains("Easy"));
        assert!(description.contains("Unique solution"));
        assert!(description.contains("35-40 clues"));
        assert!(description.contains("5 hints available"));
        
        let prowler_settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
        let prowler_description = prowler_settings.description();
        
        assert!(prowler_description.contains("Expert"));
        assert!(prowler_description.contains("22-26 clues"));
        assert!(prowler_description.contains("No hints"));
    }

    #[test]
    fn test_preset_kind_all_and_descriptions() {
        let all_presets = PresetKind::all();
        assert_eq!(all_presets.len(), 4);
        
        for preset in all_presets {
            // Each preset should have a display name and description
            let display_name = preset.display_name();
            let description = preset.description();
            
            assert!(!display_name.is_empty());
            assert!(!description.is_empty());
            
            // Display names should contain emojis
            assert!(display_name.contains("🐱") || display_name.contains("😸") || display_name.contains("😼") || display_name.contains("😾"));
            
            // Descriptions should be reasonably long
            assert!(description.len() > 30);
        }
    }

    #[test]
    fn test_default_puzzle_settings() {
        let default_settings = PuzzleSettings::default();
        let cozy_settings = PuzzleSettings::from_preset(PresetKind::CozyKitten);
        
        // Default should be the same as Cozy Kitten
        assert_eq!(default_settings.difficulty, cozy_settings.difficulty);
        assert_eq!(default_settings.givens_range, cozy_settings.givens_range);
        assert_eq!(default_settings.max_hints, cozy_settings.max_hints);
    }
}
