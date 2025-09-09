/// Nine Lives Cat Sudoku Controller Layer
///
/// This crate contains the controller logic for the Nine Lives Cat Sudoku game.
///It orchestrates the interaction between the core game logic and the UI layer.
/// Responsibilities:
/// - Event handling (user input)
/// - Game state transitions
/// - Application orchestration
/// - Connecting model and view layers
use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use nine_lives_core::{
    BoardState, DebugMode, GameHistory, GameSession, GameState, HintSystem, PuzzleSettings,
    Solution, get_next_hint,
};
use nine_lives_ui::{
    AppState, CatEmojis, Cell, ClearButton, HintButton, NewGameButton, RedoButton, UndoButton,
};

// --- Controller Systems ---

/// A system that handles clicks on the grid cells. This is part of the "Controller".
pub fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &Cell), Changed<Interaction>>,
    cat_emojis: Res<CatEmojis>,
    mut board: ResMut<BoardState>, // We get mutable access to the game state.
    mut session: ResMut<GameSession>,
    mut history: ResMut<GameHistory>,
) {
    for (interaction, cell) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Try to cycle the cell and track the move in history
            if let Some(game_move) = board.cycle_cell(cell.row, cell.col, cat_emojis.emojis.len()) {
                // Add move to history for undo/redo
                history.add_move(game_move);
                // Track move count in the session
                session.increment_move();
            }
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

/// A system that handles clicks on the "New Game" button.
/// This transitions back to the customization screen where the user can select new settings.
pub fn new_game_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("🔄 New Game button pressed - returning to customization screen");

            // Transition back to customization screen
            app_state.set(AppState::Customization);
        }
    }
}

/// System that handles clicks on the "Undo" button.
pub fn undo_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UndoButton>)>,
    mut board: ResMut<BoardState>,
    mut history: ResMut<GameHistory>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed
            && let Some(game_move) = history.peek_undo().cloned() {
                // Apply the reverse of the move
                board.undo_move(&game_move);
                // Mark as undone in history
                history.mark_undone();
                println!("Undid move at ({}, {})", game_move.row, game_move.col);
            }
    }
}

/// System that handles clicks on the "Redo" button.
pub fn redo_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RedoButton>)>,
    mut board: ResMut<BoardState>,
    mut history: ResMut<GameHistory>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed
            && let Some(game_move) = history.peek_redo().cloned() {
                // Reapply the move
                board.apply_move(&game_move);
                // Mark as redone in history
                history.mark_redone();
                println!("Redid move at ({}, {})", game_move.row, game_move.col);
            }
    }
}

/// System that handles clicks on the "Hint" button.
pub fn hint_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HintButton>)>,
    mut board: ResMut<BoardState>,
    solution: Res<Solution>,
    mut hint_system: ResMut<HintSystem>,
    debug_mode: Res<DebugMode>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if hint_system.use_hint(&debug_mode) {
                if let Some((row, col, correct_value)) = get_next_hint(&board, &solution) {
                    // Apply the hint directly to the board
                    board.cells[row][col] = Some(correct_value);
                    board.cell_types[row][col] = Some(nine_lives_core::CellType::Player);

                    if debug_mode.unlimited_hints {
                        println!(
                            "DEBUG HINT: Placed cat #{} at ({}, {}). [Unlimited hints enabled]",
                            correct_value + 1,
                            row + 1,
                            col + 1
                        );
                    } else {
                        println!(
                            "Hint: Placed cat #{} at ({}, {}). {} hints remaining.",
                            correct_value + 1,
                            row + 1,
                            col + 1,
                            hint_system.hints_remaining
                        );
                    }
                } else {
                    println!("No hints available - puzzle may be complete!");
                }
            } else {
                println!("No hints remaining!");
            }
        }
    }
}

/// System to handle debug mode toggle (Cmd+D or Ctrl+D).
pub fn debug_mode_system(input: Res<ButtonInput<KeyCode>>, mut debug_mode: ResMut<DebugMode>) {
    let cmd_pressed = input.pressed(KeyCode::SuperLeft) || input.pressed(KeyCode::SuperRight);
    let ctrl_pressed = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);

    // Use Cmd on Mac, Ctrl on other platforms
    let modifier_pressed = if cfg!(target_os = "macos") {
        cmd_pressed
    } else {
        ctrl_pressed
    };

    if modifier_pressed && input.just_pressed(KeyCode::KeyD) {
        debug_mode.toggle_unlimited_hints();
        if debug_mode.unlimited_hints {
            println!("🐛=== DEBUG MODE ACTIVATED ===");
            println!("   • Unlimited hints enabled");
            println!("   • Perfect for testing and solving puzzles");
            println!("   • Press ⌘D/Ctrl+D again to disable");
            println!("================================");
        } else {
            println!("✅=== DEBUG MODE DISABLED ===");
            println!("   • Back to normal gameplay");
            println!("   • Limited hints restored");
            println!("===============================");
        }
    }
}

/// System to handle keyboard shortcuts (Undo: Cmd+Z, Redo: Cmd+Shift+Z).
pub fn keyboard_shortcuts_system(
    input: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<BoardState>,
    mut history: ResMut<GameHistory>,
) {
    let cmd_pressed = input.pressed(KeyCode::SuperLeft) || input.pressed(KeyCode::SuperRight);
    let ctrl_pressed = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
    let shift_pressed = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

    // Use Cmd on Mac, Ctrl on other platforms
    let modifier_pressed = if cfg!(target_os = "macos") {
        cmd_pressed
    } else {
        ctrl_pressed
    };

    if modifier_pressed && input.just_pressed(KeyCode::KeyZ) {
        if shift_pressed {
            // Redo (Cmd+Shift+Z or Ctrl+Shift+Z)
            if let Some(game_move) = history.peek_redo().cloned() {
                board.apply_move(&game_move);
                history.mark_redone();
                println!(
                    "Keyboard: Redid move at ({}, {})",
                    game_move.row, game_move.col
                );
            }
        } else {
            // Undo (Cmd+Z or Ctrl+Z)
            if let Some(game_move) = history.peek_undo().cloned() {
                board.undo_move(&game_move);
                history.mark_undone();
                println!(
                    "Keyboard: Undid move at ({}, {})",
                    game_move.row, game_move.col
                );
            }
        }
    }

    // Alternative Redo shortcut: Cmd+Y or Ctrl+Y
    if modifier_pressed && input.just_pressed(KeyCode::KeyY)
        && let Some(game_move) = history.peek_redo().cloned() {
            board.apply_move(&game_move);
            history.mark_redone();
            println!(
                "Keyboard: Redid move at ({}, {})",
                game_move.row, game_move.col
            );
        }
}

/// Keeps GameState in sync with BoardState when it changes.
pub fn game_state_system(board: Res<BoardState>, mut state: ResMut<GameState>) {
    if board.is_changed() {
        *state = board.compute_game_state();
    }
}

/// Adds controller systems to the provided Bevy App.
pub fn add_controller(app: &mut App) {
    app.add_systems(
        Update,
        (
            cell_click_system,
            clear_button_system,
            new_game_button_system,
            game_state_system,
        )
            .run_if(in_state(AppState::Ready)),
    );
}

/// Configure DefaultPlugins with platform-specific settings
fn configure_default_plugins() -> PluginGroupBuilder {
    #[cfg(target_arch = "wasm32")]
    {
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nine Lives: Cat Sudoku".to_string(),
                resolution: (700., 800.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nine Lives: Cat Sudoku".to_string(),
                resolution: (700., 800.).into(),
                ..default()
            }),
            ..default()
        })
    }
}

/// Initialize WASM-specific features
#[cfg(target_arch = "wasm32")]
fn init_wasm() {
    use wasm_bindgen::prelude::*;

    // Set up panic hook for better error reporting
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Enable console logging
    #[cfg(feature = "web-sys")]
    {
        web_sys::console::log_1(&"Nine Lives: Initializing WASM...".into());
    }
}

/// Main entry point for running the Nine Lives Cat Sudoku game.
/// This function sets up the complete application by:
/// 1. Creating a Bevy App with default plugins
/// 2. Initializing the core game state (BoardState)
/// 3. Adding the UI layer (view)
/// 4. Adding the controller layer (event handling)
/// 5. Running the game loop
pub fn run_game() {
    // Initialize WASM-specific features
    #[cfg(target_arch = "wasm32")]
    init_wasm();
    let mut app = App::new();

    // Configure plugins with platform-specific settings
    app.add_plugins(configure_default_plugins());

    app
        // Initialize the core game state from the model layer
        .init_resource::<BoardState>()
        .init_resource::<GameState>()
        .init_resource::<GameSession>()
        .init_resource::<GameHistory>()
        .init_resource::<Solution>()
        .init_resource::<HintSystem>()
        .init_resource::<DebugMode>()
        .init_resource::<PuzzleSettings>()
        // Add the UI layer (view)
        .add_plugins(nine_lives_ui::UiPlugin)
        // Add controller systems
        .add_systems(
            Update,
            (
                cell_click_system,
                clear_button_system,
                new_game_button_system,
                undo_button_system,
                redo_button_system,
                hint_button_system,
                keyboard_shortcuts_system,
                debug_mode_system,
                game_state_system,
            )
                .run_if(in_state(AppState::Ready)),
        )
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_systems() {
        // Test that controller systems can be added to an app without panicking
        let mut app = App::new();
        add_controller(&mut app);
        // If we get here without panicking, the systems were added successfully
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
