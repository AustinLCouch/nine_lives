use bevy::prelude::*;
use nine_lives_core::PresetKind;
use nine_lives_ui::{
    PresetButton, SelectedPreset, handle_preset_selection, setup_selected_preset,
    sync_preset_button_highlights,
};

/// Test that demonstrates the preset button highlighting issue.
/// This test should currently fail, showing that only one button's highlighting
/// is correctly updated when the selected preset changes.
#[test]
fn test_preset_button_highlighting_sync() {
    let mut app = App::new();

    // Add minimal plugins for UI testing
    app.add_plugins(MinimalPlugins);

    // Setup the selected preset resource
    app.add_systems(Startup, setup_selected_preset);

    // Add the preset selection handling systems
    app.add_systems(
        Update,
        (handle_preset_selection, sync_preset_button_highlights),
    );

    // Run startup to initialize SelectedPreset resource
    app.update();

    // Get the presets list to know the indices
    let presets = PresetKind::all();
    let cozy_kitten_idx = presets
        .iter()
        .position(|p| matches!(p, PresetKind::CozyKitten))
        .unwrap();
    let curious_cat_idx = presets
        .iter()
        .position(|p| matches!(p, PresetKind::CuriousCat))
        .unwrap();

    // Spawn test preset buttons manually
    let cozy_button = app
        .world_mut()
        .spawn((
            Button,
            PresetButton {
                preset_id: cozy_kitten_idx,
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.3)), // Normal style initially
            BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            Interaction::None,
        ))
        .id();

    let curious_button = app
        .world_mut()
        .spawn((
            Button,
            PresetButton {
                preset_id: curious_cat_idx,
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.3)), // Normal style initially
            BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            Interaction::None,
        ))
        .id();

    // Run one update to let systems settle
    app.update();

    // Initially, CozyKitten should be selected (it's the default)
    let selected = app.world().resource::<SelectedPreset>();
    assert!(matches!(selected.preset, PresetKind::CozyKitten));

    // Now simulate clicking the CuriousCat button
    // Set interaction to Pressed to trigger the selection
    let mut interaction = app
        .world_mut()
        .get_mut::<Interaction>(curious_button)
        .unwrap();
    *interaction = Interaction::Pressed;

    // Run update to process the interaction
    app.update();

    // Reset interaction back to None (simulating release)
    let mut interaction = app
        .world_mut()
        .get_mut::<Interaction>(curious_button)
        .unwrap();
    *interaction = Interaction::None;

    // Run another update
    app.update();

    // Now CuriousCat should be selected
    let selected = app.world().resource::<SelectedPreset>();
    assert!(matches!(selected.preset, PresetKind::CuriousCat));

    // Check button colors - this is where the current implementation fails
    let cozy_bg = app.world().get::<BackgroundColor>(cozy_button).unwrap();
    let curious_bg = app.world().get::<BackgroundColor>(curious_button).unwrap();

    // The CozyKitten button should now have normal styling (not selected)
    let expected_normal_bg = Color::srgb(0.2, 0.2, 0.3);
    assert_eq!(
        cozy_bg.0, expected_normal_bg,
        "CozyKitten button should have normal styling when not selected"
    );

    // The CuriousCat button should have selected styling
    let expected_selected_bg = Color::srgb(0.2, 0.4, 0.2);
    assert_eq!(
        curious_bg.0, expected_selected_bg,
        "CuriousCat button should have selected styling when selected"
    );
}

/// Test helper to verify that preset buttons exist and have correct initial states
#[test]
fn test_preset_button_setup() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Startup, setup_selected_preset);

    app.update();

    // Verify the SelectedPreset resource exists and has default value
    let selected = app.world().resource::<SelectedPreset>();
    assert!(matches!(selected.preset, PresetKind::CozyKitten)); // Default is CozyKitten
}
