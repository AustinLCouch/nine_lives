//! Nine Lives Cat Sudoku Controller Layer
//!
//! This crate contains the controller logic for the Nine Lives Cat Sudoku game.
//! It orchestrates the interaction between the core game logic and the UI layer.
//! Responsibilities:
//! - Event handling (user input)
//! - Game state transitions
//! - Application orchestration
//! - Connecting model and view layers

use bevy::prelude::*;
use nine_lives_core::BoardState;
use nine_lives_ui::{AppState, CatEmojis, Cell, ClearButton};

// --- Controller Systems ---

/// A system that handles clicks on the grid cells. This is part of the "Controller".
pub fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &Cell), Changed<Interaction>>,
    cat_emojis: Res<CatEmojis>,
    mut board: ResMut<BoardState>, // We get mutable access to the game state.
) {
    for (interaction, cell) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // The Bevy system calls the method on the BoardState to update the game state.
            // The logic for *how* to cycle is neatly contained in the core crate.
            board.cycle_cell(cell.row, cell.col, cat_emojis.emojis.len());
        }
    }
}

/// A system that handles clicks on the "Clear Board" button. This is also a "Controller".
pub fn clear_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClearButton>)>,
    mut board: ResMut<BoardState>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // The system calls the `clear` method from our core crate.
            board.clear();
        }
    }
}

/// Adds controller systems to the provided Bevy App.
pub fn add_controller(app: &mut App) {
    app.add_systems(
        Update,
        (
            cell_click_system,
            clear_button_system,
        )
            .run_if(in_state(AppState::Ready)),
    );
}

/// Main entry point for running the Nine Lives Cat Sudoku game.
/// This function sets up the complete application by:
/// 1. Creating a Bevy App with default plugins
/// 2. Initializing the core game state (BoardState)
/// 3. Adding the UI layer (view)
/// 4. Adding the controller layer (event handling)
/// 5. Running the game loop
pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nine Lives: Cat Sudoku".to_string(),
                resolution: (700., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize the core game state from the model layer
        .init_resource::<BoardState>()
        // Add the UI layer (view)
        .add_plugins(nine_lives_ui::UiPlugin)
        // Add controller systems
        .add_systems(
            Update,
            (
                cell_click_system,
                clear_button_system,
            )
                .run_if(in_state(AppState::Ready)),
        )
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use nine_lives_core::GRID_SIZE;

    #[test]
    fn test_controller_systems() {
        // Test that controller systems can be added to an app without panicking
        let mut app = App::new();
        add_controller(&mut app);
        // If we get here, the systems were added successfully
        assert!(true);
    }

    #[test]
    fn test_cell_click_logic() {
        // Test the cell click logic by simulating the system behavior
        let mut board = BoardState::new();
        let cat_emojis = CatEmojis {
            emojis: vec!["cat1".to_string(), "cat2".to_string(), "cat3".to_string()],
        };

        // Simulate cycling a cell
        board.cycle_cell(0, 0, cat_emojis.emojis.len());
        assert_eq!(board.cells[0][0], Some(0));
        
        board.cycle_cell(0, 0, cat_emojis.emojis.len());
        assert_eq!(board.cells[0][0], Some(1));
    }

    #[test]
    fn test_clear_board_logic() {
        // Test the clear board logic
        let mut board = BoardState::new();
        board.cycle_cell(1, 1, 3);
        board.cycle_cell(2, 2, 3);
        
        // Now clear the board
        board.clear();
        assert_eq!(board.cells[1][1], None);
        assert_eq!(board.cells[2][2], None);
    }
}
