//! Nine Lives Cat Sudoku UI Layer
//!
//! This crate contains the user interface components, systems, and resources
//! for the Nine Lives Cat Sudoku game. It handles:
//! - UI components (Cell, ClearButton)
//! - Presentation resources (CatEmojis)
//! - Rendering systems
//! - Application states

use bevy::prelude::*;
use nine_lives_core::{BoardState, GRID_SIZE};

// --- UI Components ---

/// A component to tag a UI entity as a grid cell, storing its position.
#[derive(Component)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

/// A component to tag the "Clear Board" button entity.
#[derive(Component)]
pub struct ClearButton;

// --- UI Resources ---

/// A Bevy resource that holds the ASCII art for the cats.
/// This is presentation data, so it belongs in the UI layer.
#[derive(Resource)]
pub struct CatEmojis {
    pub emojis: Vec<String>,
}

// --- Application States ---

/// Defines the different states of the application, like loading assets vs. running the game.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    Ready,
}

// --- UI Systems ---

/// A system that loads the cat ASCII art into the `CatEmojis` resource.
pub fn setup_cat_emojis(mut commands: Commands) {
    let emojis = vec![
        " /\\_/\\\n( ^.^ )\n \\_1_/".to_string(),
        " /\\_/\\\n( o.o )\n \\_2_/".to_string(),
        " /\\_/\\\n( -.- )\n \\_3_/".to_string(),
        " /\\_/\\\n( >:< )\n \\_4_/".to_string(),
        " /\\_/\\\n( @.@ )\n \\_5_/".to_string(),
        " /\\_/\\\n( u.u )\n \\_6_/".to_string(),
        " /\\_/\\\n( *.* )\n \\_7_/".to_string(),
        " /\\_/\\\n( x.x )\n \\_8_/".to_string(),
        " /\\_/\\\n( $.$ )\n \\_9_/".to_string(),
    ];
    commands.insert_resource(CatEmojis { emojis });
}

/// A system to update the text in the cells when the board state changes. This is the "View".
pub fn update_cell_text(
    board: Res<BoardState>,
    cat_emojis: Res<CatEmojis>,
    cell_query: Query<(&Cell, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (cell, children) in &cell_query {
        // Get the first child of the cell, which should be the Text entity.
        if let Some(text_entity) = children.iter().next() {
            if let Ok(mut text) = text_query.get_mut(text_entity) {
                let new_text_value = match board.cells[cell.row][cell.col] {
                    Some(idx) => cat_emojis.emojis[idx].clone(),
                    None => " ".to_string(), // Empty cells are just blank.
                };

                // Only update the text if it has actually changed.
                if **text != new_text_value {
                    **text = new_text_value;
                }
            }
        }
    }
}

/// A simple placeholder system to get the game running with basic functionality
pub fn setup_grid(mut commands: Commands) {
    // Just spawn a camera for now - we'll add UI later once compilation works
    commands.spawn(Camera2d);

    // Print to console to show the game is working
    println!("Nine Lives Cat Sudoku is starting up!");
    println!(
        "Game board initialized with {} x {} grid",
        GRID_SIZE, GRID_SIZE
    );
}

/// A system that transitions the app from `Loading` to `Ready` once resources are loaded.
pub fn transition_to_ready(
    mut app_state: ResMut<NextState<AppState>>,
    cat_emojis: Option<Res<CatEmojis>>,
) {
    // We transition once the CatEmojis resource exists.
    if cat_emojis.is_some() {
        app_state.set(AppState::Ready);
    }
}

/// UI Plugin for Nine Lives Cat Sudoku.
/// This plugin handles all UI-related functionality including states, systems, and resources.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup_cat_emojis)
            .add_systems(OnEnter(AppState::Ready), setup_grid)
            .add_systems(
                Update,
                (
                    update_cell_text.run_if(resource_changed::<BoardState>),
                    transition_to_ready,
                )
                    .run_if(in_state(AppState::Ready)),
            );
    }
}

/// Adds all UI systems, states, and resources to the provided Bevy App.
/// This is a convenience function that adds the UiPlugin.
pub fn add_ui(app: &mut App) {
    app.add_plugins(UiPlugin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_emojis_creation() {
        // Test the setup_cat_emojis system by creating a test app
        let mut app = App::new();
        app.add_systems(Startup, setup_cat_emojis);
        app.update(); // Run startup systems
        
        let cat_emojis = app.world().get_resource::<CatEmojis>().unwrap();
        assert_eq!(cat_emojis.emojis.len(), 9);
        assert!(cat_emojis.emojis[0].contains("^.^"));
        assert!(cat_emojis.emojis[8].contains("$.$ "));
    }

    #[test] 
    fn test_cell_component() {
        let cell = Cell { row: 5, col: 3 };
        assert_eq!(cell.row, 5);
        assert_eq!(cell.col, 3);
    }
}
