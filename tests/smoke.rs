// Comprehensive integration tests for Nine Lives Cat Sudoku.
// These tests validate the complete game flow and feature integration.

use nine_lives_core::*;

#[test]
fn test_board_creation() {
    let board = BoardState::new();
    // Basic smoke test: creating a board should work
    assert_eq!(board.cells.len(), GRID_SIZE);
    assert_eq!(board.cells[0].len(), GRID_SIZE);
}

#[test]
fn test_cell_cycling_with_history() {
    let mut board = BoardState::new();
    
    // Test basic cell cycling with move tracking
    if let Some(move1) = board.cycle_cell(0, 0, 9) {
        assert_eq!(board.cells[0][0], Some(0));
        assert_eq!(move1.old_value, None);
        assert_eq!(move1.new_value, Some(0));
    }
    
    if let Some(move2) = board.cycle_cell(0, 0, 9) {
        assert_eq!(board.cells[0][0], Some(1));
        assert_eq!(move2.old_value, Some(0));
        assert_eq!(move2.new_value, Some(1));
    }
}

#[test]
fn test_game_history_functionality() {
    let mut history = GameHistory::new();
    
    // Test empty history
    assert!(!history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.position_info(), (0, 0));
    
    // Add a move
    let test_move = Move {
        row: 0,
        col: 0,
        old_value: None,
        new_value: Some(1),
        timestamp: std::time::Instant::now(),
    };
    
    history.add_move(test_move.clone());
    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.position_info(), (1, 1));
    
    // Test undo
    assert_eq!(history.peek_undo().unwrap().new_value, Some(1));
    history.mark_undone();
    assert!(!history.can_undo());
    assert!(history.can_redo());
    
    // Test redo
    assert_eq!(history.peek_redo().unwrap().new_value, Some(1));
    history.mark_redone();
    assert!(history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_hint_system_complete() {
    let mut hint_system = HintSystem::new(3);
    
    // Test initial state
    assert_eq!(hint_system.hints_remaining, 3);
    assert!(hint_system.can_use_hint());
    
    // Use hints
    assert!(hint_system.use_hint());
    assert_eq!(hint_system.hints_remaining, 2);
    
    assert!(hint_system.use_hint());
    assert!(hint_system.use_hint());
    assert_eq!(hint_system.hints_remaining, 0);
    assert!(!hint_system.can_use_hint());
    
    // Can't use more hints
    assert!(!hint_system.use_hint());
    
    // Reset hints
    hint_system.reset(5);
    assert_eq!(hint_system.hints_remaining, 5);
    assert!(hint_system.can_use_hint());
}

#[test]
fn test_puzzle_generation_and_solution() {
    let mut board = BoardState::new();
    
    // Generate a puzzle and get the solution
    let solution = board.generate_easy_puzzle();
    
    // Verify the board has some empty cells (it's a puzzle, not complete)
    let empty_cells = board.cells.iter()
        .flatten()
        .filter(|cell| cell.is_none())
        .count();
    assert!(empty_cells > 0, "Puzzle should have empty cells");
    
    // Verify the solution is complete
    let solution_filled = solution.cells.iter()
        .flatten()
        .all(|&val| val < 9); // All values should be 0-8 (valid cat indices)
    assert!(solution_filled, "Solution should be complete");
    
    // Verify no conflicts in given cells
    assert!(board.get_conflicts().is_empty(), "Generated puzzle should have no conflicts");
}

#[test]
fn test_game_session_timing() {
    let mut session = GameSession::new();
    
    // Test initial state
    assert_eq!(session.move_count, 0);
    assert!(!session.is_paused);
    
    // Test move counting
    session.increment_move();
    session.increment_move();
    assert_eq!(session.move_count, 2);
    
    // Test pause/resume
    session.pause();
    assert!(session.is_paused);
    assert!(session.pause_start.is_some());
    
    session.resume();
    assert!(!session.is_paused);
    assert!(session.pause_start.is_none());
    
    // Test reset
    session.reset();
    assert_eq!(session.move_count, 0);
    assert!(!session.is_paused);
}

#[test]
fn test_full_game_flow_integration() {
    let mut board = BoardState::new();
    let mut history = GameHistory::new();
    let mut session = GameSession::new();
    let mut hint_system = HintSystem::new(3);
    
    // Generate a puzzle
    let solution = board.generate_easy_puzzle();
    session.reset();
    history.clear();
    hint_system.reset(3);
    
    // Make some moves
    if let Some(move1) = board.cycle_cell(0, 0, 9) {
        history.add_move(move1);
        session.increment_move();
    }
    
    if let Some(move2) = board.cycle_cell(1, 1, 9) {
        history.add_move(move2);
        session.increment_move();
    }
    
    assert_eq!(session.move_count, 2);
    assert!(history.can_undo());
    
    // Test undo
    if let Some(last_move) = history.peek_undo().cloned() {
        board.undo_move(&last_move);
        history.mark_undone();
    }
    
    // Test hint system
    let initial_hints = hint_system.hints_remaining;
    if let Some((row, col, value)) = get_next_hint(&board, &solution) {
        if hint_system.use_hint() {
            board.cells[row][col] = Some(value);
            board.cell_types[row][col] = Some(CellType::Player);
        }
    }
    assert_eq!(hint_system.hints_remaining, initial_hints - 1);
    
    // Verify game state consistency
    let game_state = board.compute_game_state();
    assert!(matches!(game_state, GameState::Playing | GameState::Won));
}

