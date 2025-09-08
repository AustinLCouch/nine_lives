//! Nine Lives Cat Sudoku UI Layer
//!
//! This crate contains the user interface components, systems, and resources
//! for the Nine Lives Cat Sudoku game. It handles:
//! - UI components (Cell, ClearButton)
//! - Presentation resources (CatEmojis)
//! - Rendering systems
//! - Application states

use bevy::prelude::*;
use nine_lives_core::{BoardState, GRID_SIZE, GameState};
use std::collections::HashSet;

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

/// A component to tag the "New Game" button entity.
#[derive(Component)]
pub struct NewGameButton;

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

// --- Helper Functions ---

/// Returns the background color for a cell based on its position
/// Creates a visual distinction between the 3x3 sudoku boxes
fn get_cell_background_color(row: usize, col: usize) -> Color {
    let box_row = row / 3;
    let box_col = col / 3;

    // Alternate colors for the 3x3 boxes to make them visually distinct
    if (box_row + box_col) % 2 == 0 {
        Color::srgb(0.9, 0.9, 0.9) // Light gray
    } else {
        Color::srgb(0.8, 0.8, 0.8) // Slightly darker gray
    }
}

// --- UI Systems ---

/// A system that loads the cat ASCII art into the `CatEmojis` resource.
/// Now using the user's new detailed multi-line ASCII kitten designs!
pub fn setup_cat_emojis(mut commands: Commands) {
    let emojis = vec![
        r"   /\_/\  
  ( o.o ) 
  >  ^  < 
   / | \  
  (  1  )"
            .to_string(),
        r"  /\_____/\
 (  • ᴥ •  )
 (  > 2 <  )
  \__|__|_/"
            .to_string(),
        r"   /\_/\  
  ( =ω= ) 
  (  3  ) 
  /  |  \ 
 <__^__^__>"
            .to_string(),
        r"   /\_/\  
  ( o.o ) 
  /| 4 |\ 
  \_   _/ 
    \_/"
        .to_string(),
        r"   /\_/\  
  ( ^_^ ) 
  (  5  ) 
  /  |  \ 
 <__|__|__>"
            .to_string(),
        r"   /\_/\  
  ( o.o ) 
  (  6  ) 
  /  |  \ 
  \__^__/"
            .to_string(),
        r"   /\_/\  
  ( -.- ) 
  (  7  ) 
  /  |  \ 
 <__v__v__>"
            .to_string(),
        r"   /\_/\  
  ( >w< ) 
  (  8  ) 
  /  |  \ 
  \__|__/"
            .to_string(),
        r"   /\_/\  
  ( o_o ) 
  (  9  ) 
  /  |  \ 
 <__*__*__>"
            .to_string(),
    ];
    commands.insert_resource(CatEmojis { emojis });
}

/// A system to update the text in the cells when the board state changes. This is the "View".
pub fn update_cell_text(
    board: Res<BoardState>,
    cat_emojis: Res<CatEmojis>,
    cell_query: Query<(&Cell, &Children)>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (cell, children) in &cell_query {
        // Get the first child of the cell, which should be the Text entity.
        if let Some(text_entity) = children.iter().next() {
            if let Ok((mut text, mut color)) = text_query.get_mut(text_entity) {
                let new_text_value = match board.cells[cell.row][cell.col] {
                    Some(idx) => cat_emojis.emojis[idx].clone(),
                    None => " ".to_string(), // Empty cells are just blank.
                };

                // Only update the text if it has actually changed.
                if text.0 != new_text_value {
                    text.0 = new_text_value;
                }

                // Style: Given numbers are much darker and bolder, player numbers are bright blue
                if board.is_given_cell(cell.row, cell.col) {
                    // Very dark, almost black text for givens (permanent puzzle numbers)
                    color.0 = Color::srgb(0.0, 0.0, 0.0);
                } else {
                    // Bright blue for player entries (clearly different)
                    color.0 = Color::srgb(0.1, 0.3, 0.8);
                }
            }
        }
    }
}

/// A system to update cell colors based on Sudoku validation.
///
/// This provides visual feedback by:
/// - Highlighting conflicting cells in red
/// - Highlighting the entire board in green when completed
/// - Using normal colors otherwise
pub fn update_cell_colors(
    board: Res<BoardState>,
    game_state: Res<GameState>,
    mut cell_query: Query<(&Cell, &mut BackgroundColor)>,
) {
    let conflicts = board.get_conflicts();
    let conflict_set: HashSet<(usize, usize)> = conflicts.into_iter().collect();
    let is_complete = matches!(*game_state, GameState::Won);

    for (cell, mut bg_color) in &mut cell_query {
        let base_color = get_cell_background_color(cell.row, cell.col);

        if is_complete {
            // Green tint for completion - celebrate!
            *bg_color = BackgroundColor(Color::srgb(0.6, 0.9, 0.6));
        } else if conflict_set.contains(&(cell.row, cell.col)) {
            // Red tint for conflicts - show mistakes
            *bg_color = BackgroundColor(Color::srgb(1.0, 0.7, 0.7));
        } else if board.is_given_cell(cell.row, cell.col) {
            // Slightly darker/more solid background for given cells (permanent puzzle numbers)
            // Convert to linear space, darken, then back to sRGB
            let [r, g, b, a] = base_color.to_linear().to_f32_array();
            let darker_base = Color::linear_rgba(
                r * 0.7, // Make significantly darker (30% of original)
                g * 0.7,
                b * 0.7,
                a,
            );
            *bg_color = BackgroundColor(darker_base);
        } else {
            // Normal alternating colors for player-fillable cells
            *bg_color = BackgroundColor(base_color);
        }
    }
}

/// System to add hover effects to buttons for better user feedback.
pub fn update_button_colors(
    mut new_game_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<NewGameButton>, Changed<Interaction>),
    >,
    mut clear_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<ClearButton>,
            Changed<Interaction>,
            Without<NewGameButton>,
        ),
    >,
) {
    // Handle New Game button (green theme)
    for (interaction, mut bg_color) in &mut new_game_query {
        match interaction {
            Interaction::Pressed => bg_color.0 = Color::srgb(0.2, 0.4, 0.2),
            Interaction::Hovered => bg_color.0 = Color::srgb(0.4, 0.7, 0.4),
            Interaction::None => bg_color.0 = Color::srgb(0.3, 0.6, 0.3),
        }
    }

    // Handle Clear button (red theme)
    for (interaction, mut bg_color) in &mut clear_query {
        match interaction {
            Interaction::Pressed => bg_color.0 = Color::srgb(0.4, 0.2, 0.2),
            Interaction::Hovered => bg_color.0 = Color::srgb(0.7, 0.4, 0.4),
            Interaction::None => bg_color.0 = Color::srgb(0.6, 0.3, 0.3),
        }
    }
}

/// System to add subtle hover effects to game cells.
pub fn update_cell_hover_effects(
    board: Res<BoardState>,
    mut cell_query: Query<
        (&Cell, &Interaction, &mut BorderColor),
        (With<Button>, Changed<Interaction>),
    >,
) {
    for (cell, interaction, mut border_color) in &mut cell_query {
        match interaction {
            Interaction::Hovered => {
                // Only show interactive hover on cells that can be changed (not given cells)
                if !board.is_given_cell(cell.row, cell.col) {
                    border_color.0 = Color::srgb(0.2, 0.6, 1.0); // Bright blue hover for player cells
                } else {
                    border_color.0 = Color::srgb(0.6, 0.6, 0.6); // Darker border to show it's not interactive
                }
            }
            Interaction::None => {
                if board.is_given_cell(cell.row, cell.col) {
                    border_color.0 = Color::srgb(0.3, 0.3, 0.3); // Darker borders for given cells
                } else {
                    border_color.0 = Color::srgb(0.4, 0.4, 0.4); // Normal border for player cells
                }
            }
            Interaction::Pressed => {
                // Handled by the cell click system in the controller
                if board.is_given_cell(cell.row, cell.col) {
                    border_color.0 = Color::srgb(0.3, 0.3, 0.3); // Keep darker for given cells
                } else {
                    border_color.0 = Color::srgb(0.4, 0.4, 0.4); // Normal for player cells
                }
            }
        }
    }
}

/// System that creates the visual 9x9 sudoku grid with clickable cells
pub fn setup_grid(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Create the main UI root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Nine Lives: Cat Sudoku"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Game grid container
            parent
                .spawn((
                    Node {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(9, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(9, 1.0),
                        column_gap: Val::Px(2.0),
                        row_gap: Val::Px(2.0),
                        width: Val::Px(720.0),
                        height: Val::Px(630.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|grid_parent| {
                    // Create 9x9 grid of cells
                    for row in 0..GRID_SIZE {
                        for col in 0..GRID_SIZE {
                            grid_parent
                                .spawn((
                                    Button,
                                    Cell { row, col },
                                    Node {
                                        width: Val::Px(75.0),
                                        height: Val::Px(65.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BackgroundColor(get_cell_background_color(row, col)),
                                    BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                                ))
                                .with_children(|cell_parent| {
                                    // Text node for displaying the multi-line cat ASCII art
                                    cell_parent.spawn((
                                        Text::new(" "),
                                        TextFont {
                                            font_size: 8.0,
                                            ..default()
                                        },
                                        TextColor(Color::BLACK),
                                        Node {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    }
                });

            // Buttons container
            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|buttons_parent| {
                    // New Game button
                    buttons_parent
                        .spawn((
                            Button,
                            NewGameButton,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(40.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.6, 0.3)),
                            BorderColor(Color::srgb(0.4, 0.8, 0.4)),
                        ))
                        .with_children(|button_parent| {
                            button_parent.spawn((
                                Text::new("New Game"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Clear button
                    buttons_parent
                        .spawn((
                            Button,
                            ClearButton,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(40.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                            BorderColor(Color::srgb(0.8, 0.4, 0.4)),
                        ))
                        .with_children(|button_parent| {
                            button_parent.spawn((
                                Text::new("Clear Board"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });

    println!("Nine Lives Cat Sudoku UI initialized!");
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
                    update_cell_text
                        .run_if(resource_changed::<BoardState>)
                        .run_if(in_state(AppState::Ready)),
                    update_cell_colors
                        .run_if(|b: Res<BoardState>, s: Res<GameState>| {
                            b.is_changed() || s.is_changed()
                        })
                        .run_if(in_state(AppState::Ready)),
                    update_button_colors.run_if(in_state(AppState::Ready)),
                    update_cell_hover_effects.run_if(in_state(AppState::Ready)),
                    transition_to_ready.run_if(in_state(AppState::Loading)),
                ),
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
        // Test the new multi-line ASCII art designs
        assert!(cat_emojis.emojis[0].contains("o.o")); // First kitten has o.o eyes
        assert!(cat_emojis.emojis[0].contains("(  1  )")); // First kitten has number 1
        assert!(cat_emojis.emojis[8].contains("o_o")); // Ninth kitten has o_o eyes 
        assert!(cat_emojis.emojis[8].contains("(  9  )")); // Ninth kitten has number 9
    }

    #[test]
    fn test_cell_component() {
        let cell = Cell { row: 5, col: 3 };
        assert_eq!(cell.row, 5);
        assert_eq!(cell.col, 3);
    }
}
