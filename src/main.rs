use bevy::prelude::*;

// Number of rows and columns in Sudoku
const GRID_SIZE: usize = 9;

// Component to identify each cell
#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
}

// Component to identify the clear button
#[derive(Component)]
struct ClearButton;

// Resource to store cat emojis
#[derive(Resource)]
struct CatEmojis {
    emojis: Vec<String>,
}

// Resource to store the current state of the board
#[derive(Resource)]
struct BoardState {
    // Each cell stores Option<cat_index>
    cells: [[Option<usize>; GRID_SIZE]; GRID_SIZE],
}

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
        .init_state::<AppState>()
        .insert_resource(BoardState {
            cells: [[None; GRID_SIZE]; GRID_SIZE],
        })
        .add_systems(Startup, setup_cat_emojis)
        .add_systems(OnEnter(AppState::Ready), setup_grid)
        .add_systems(
            Update,
            (
                update_cell_text,
                cell_click_system,
                clear_button_system,
                transition_to_ready,
            ),
        )
        .run();
}

// AppState to wait for setup before building UI
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    Loading,
    Ready,
}

// Setup ASCII art cats holding their numbers
fn setup_cat_emojis(mut commands: Commands) {
    let emojis = vec![
        r"/\_/\
( ^.^ )
 \_1_/"
            .to_string(), // Cat holding 1
        r"/\_/\
( o.o )
 \_2_/"
            .to_string(), // Cat holding 2
        r"/\_/\
( -.^ )
 \_3_/"
            .to_string(), // Cat holding 3
        r"/\_/\
( >:< )
 \_4_/"
            .to_string(), // Cat holding 4
        r"/\_/\
( @.@ )
 \_5_/"
            .to_string(), // Cat holding 5
        r"/\_/\
( u.u )
 \_6_/"
            .to_string(), // Cat holding 6
        r"/\_/\
( *.* )
 \_7_/"
            .to_string(), // Cat holding 7
        r"/\_/\
( x.x )
 \_8_/"
            .to_string(), // Cat holding 8
        r"/\_/\
( $.$ )
 \_9_/"
            .to_string(), // Cat holding 9
    ];

    commands.insert_resource(CatEmojis { emojis });
}

// Emojis are immediately available, so transition to Ready state
fn transition_to_ready(
    mut app_state: ResMut<NextState<AppState>>,
    cat_emojis: Option<Res<CatEmojis>>,
    current_state: Res<State<AppState>>,
) {
    if cat_emojis.is_some() && *current_state.get() == AppState::Loading {
        app_state.set(AppState::Ready);
    }
}

// Build the Sudoku grid UI
fn setup_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // Root node (vertical)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.95, 1.0)),
            Name::new("Root"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Nine Lives: ASCII Cat Sudoku"),
                TextFont {
                    font: asset_server.load("fonts/MonoLisa.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.2, 0.5)),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Clear Board button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    ClearButton,
                    Name::new("ClearButton"),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Clear Board"),
                        TextFont {
                            font: asset_server.load("fonts/MonoLisa.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Grid
            parent
                .spawn((
                    Node {
                        width: Val::Px(630.0),
                        height: Val::Px(630.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.8, 0.95)),
                    Name::new("Grid"),
                ))
                .with_children(|grid| {
                    for row in 0..GRID_SIZE {
                        grid.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0 / GRID_SIZE as f32),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            Name::new(format!("Row{}", row)),
                        ))
                        .with_children(|row_node| {
                            for col in 0..GRID_SIZE {
                                row_node
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(100.0 / GRID_SIZE as f32),
                                            height: Val::Percent(100.0),
                                            margin: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                                        Cell { row, col },
                                        Name::new(format!("Cell{}_{}", row, col)),
                                    ))
                                    .with_children(|cell| {
                                        // Text for ASCII cats with MonoLisa font
                                        cell.spawn((
                                            Text(
                                                r"   /\_/\
   ( ^.^ )
    \_?_/"
                                                    .to_string(),
                                            ),
                                            TextFont {
                                                font: asset_server.load("fonts/MonoLisa.ttf"),
                                                font_size: 11.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.3, 0.3, 0.3)),
                                        ));
                                    });
                            }
                        });
                    }
                });
        });
}

// Handle cell clicks: cycle through cat emojis
fn cell_click_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &Cell, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    cat_emojis: Res<CatEmojis>,
    mut board: ResMut<BoardState>,
    mut text_query: Query<&mut Text>,
) {
    for (_entity, interaction, cell, mut color, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Cell clicked: {}, {}", cell.row, cell.col);

            // Cycle the cat emoji index for this cell
            let current = board.cells[cell.row][cell.col];
            let next = match current {
                None => Some(0),
                Some(idx) => {
                    let next_idx = (idx + 1) % cat_emojis.emojis.len();
                    Some(next_idx)
                }
            };
            board.cells[cell.row][cell.col] = next;

            // Visual feedback
            *color = BackgroundColor(Color::srgb(0.9, 0.9, 1.0));

            // Update the text in the cell
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if let Some(idx) = next {
                        text.0 = cat_emojis.emojis[idx].clone();
                        println!("Setting text to: {}", cat_emojis.emojis[idx]);
                    } else {
                        text.0 = "".to_string();
                    }
                    break;
                }
            }
        }
    }
}

// Handle clear button clicks
fn clear_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClearButton>),
    >,
    mut board: ResMut<BoardState>,
    cell_query: Query<&Children, With<Cell>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Clear board button pressed!");

                // Clear the board state
                board.cells = [[None; GRID_SIZE]; GRID_SIZE];

                // Reset all cell texts to the placeholder cat
                for children in &cell_query {
                    if let Some(child) = children.iter().next() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.0 = r"   /\_/\
   ( ^.^ )
    \_?_/"
                                .to_string();
                        }
                    }
                }

                // Visual feedback - darker red when pressed
                *color = BackgroundColor(Color::srgb(0.6, 0.1, 0.1));
            }
            Interaction::Hovered => {
                // Lighter red when hovered
                *color = BackgroundColor(Color::srgb(0.9, 0.3, 0.3));
            }
            Interaction::None => {
                // Default red color
                *color = BackgroundColor(Color::srgb(0.8, 0.2, 0.2));
            }
        }
    }
}

// Update all cell text to match board state (e.g., after reload)
fn update_cell_text(
    cat_emojis: Res<CatEmojis>,
    board: Res<BoardState>,
    mut query: Query<(&Cell, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if !board.is_changed() {
        return;
    }
    for (cell, children) in &mut query {
        if let Some(child) = children.iter().next() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = if let Some(idx) = board.cells[cell.row][cell.col] {
                    cat_emojis.emojis[idx].clone()
                } else {
                    "".to_string()
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_size_is_9() {
        assert_eq!(GRID_SIZE, 9);
    }

    #[test]
    fn board_cells_default_none() {
        let board = BoardState { cells: [[None; GRID_SIZE]; GRID_SIZE] };
        assert!(board
            .cells
            .iter()
            .flatten()
            .all(|c| c.is_none()));
    }
}
