// A minimal Bevy app to test basic functionality
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Try to create a simple text entity
    commands.spawn(TextBundle::from_section(
        "Hello, World!",
        TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}
