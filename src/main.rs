// nine_lives_workspace/nine_lives/src/main.rs

//! This crate is the main application for the Nine Lives Cat Sudoku game.
//! It uses the Bevy game engine for rendering, UI, and input handling.
//! All core game state and logic are provided by the `nine_lives_logic` crate.

use bevy::prelude::*;
// Import the game's core data structures from our logic crate.
use nine_lives_logic::{BoardState, GRID_SIZE};

// --- Bevy Components and Resources (View/Controller Layer) ---

/// A component to tag a UI entity as a grid cell, storing its position.
#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
}

/// A component to tag the "Clear Board" button entity.
#[derive(Component)]
struct ClearButton;

/// A Bevy resource that holds the ASCII art for the cats.
/// This is presentation data, so it belongs in the Bevy crate, not the logic crate.
#[derive(Resource)]
struct CatEmojis {
    emojis: Vec<String>,
}

// --- Application States ---

/// Defines the different states of the application, like loading assets vs. running the game.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    Loading,
    Ready,
}

// --- Main Application Setup ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nine Lives: Cat Sudoku".to_string(),
                resolution: (700., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        // Manage the application's state (Loading vs. Ready).
        .init_state::<AppState>()
        // Initialize our `BoardState` from the logic crate as a global resource.
        .init_resource::<BoardState>()
        // Systems that run once at the very beginning.
        .add_systems(Startup, setup_cat_emojis)
        // Systems that run when entering a specific state.
        .add_systems(OnEnter(AppState::Ready), setup_grid)
        // Systems that run every frame, but only in the `Ready` state.
        .add_systems(
            Update,
            (
                // This system only runs if the `BoardState` resource has changed.
                update_cell_text.run_if(resource_changed::<BoardState>),
                cell_click_system,
                clear_button_system,
                transition_to_ready,
            )
                .run_if(in_state(AppState::Ready)),
        )
        .run();
}

// --- Systems ---

/// A system that loads the cat ASCII art into the `CatEmojis` resource.
fn setup_cat_emojis(mut commands: Commands) {
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

/// A system that transitions the app from `Loading` to `Ready` once resources are loaded.
fn transition_to_ready(
    mut app_state: ResMut<NextState<AppState>>,
    cat_emojis: Option<Res<CatEmojis>>,
) {
    // We transition once the CatEmojis resource exists.
    if cat_emojis.is_some() {
        app_state.set(AppState::Ready);
    }
}

/// A system that handles clicks on the grid cells. This is part of the "Controller".
fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &Cell), Changed<Interaction>>,
    cat_emojis: Res<CatEmojis>,
    mut board: ResMut<BoardState>, // We get mutable access to the game state.
) {
    for (interaction, cell) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // The Bevy system calls the method on the BoardState to update the game state.
            // The logic for *how* to cycle is neatly contained in the logic crate.
            board.cycle_cell(cell.row, cell.col, cat_emojis.emojis.len());
        }
    }
}

/// A system that handles clicks on the "Clear Board" button. This is also a "Controller".
fn clear_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClearButton>)>,
    mut board: ResMut<BoardState>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // The system calls the `clear` method from our logic crate.
            board.clear();
        }
    }
}

/// A system to update the text in the cells when the board state changes. This is the "View".
fn update_cell_text(
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
fn setup_grid(mut commands: Commands) {
    // Just spawn a camera for now - we'll add UI later once compilation works
    commands.spawn(Camera2d);
    
    // Print to console to show the game is working
    println!("Nine Lives Cat Sudoku is starting up!");
    println!("Game board initialized with {} x {} grid", GRID_SIZE, GRID_SIZE);
}
