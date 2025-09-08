//! ASCII Art Kitty Designs for Nine Lives Sudoku
//!
//! This module contains the ASCII art representations of the 9 different cats
//! used in the Nine Lives Cat Sudoku game. Each number 1-9 corresponds to
//! a unique kitty design.

use bevy::prelude::*;

/// Type alias for kitty ASCII art - an array of text lines
pub type KittyArt = &'static [&'static str];

/// Resource containing all the kitty ASCII art designs
#[derive(Resource)]
pub struct KittyArts {
    pub arts: [KittyArt; 9],
}

/// Beautiful detailed ASCII kitty designs - each kitten has its own personality!
pub const DEFAULT_KITTIES: [KittyArt; 9] = [
    // Kitten 1 - Wide-eyed and curious
    &[
        "   /\\_/\\  ",
        "  ( o.o ) ",
        "  >  ^  < ",
        "   / | \\  ",
        "  (  1  ) ",
    ],
    // Kitten 2 - Happy and content
    &[
        "  /\\_____/\\",
        " (  • ᴥ •  )",
        " (  > 2 <  )",
        "  \\__|__|_/ ",
    ],
    // Kitten 3 - Sleepy and serene
    &[
        "   /\\_/\\  ",
        "  ( =ω= ) ",
        "  (  3  ) ",
        "  /  |  \\ ",
        " <__^__^__>",
    ],
    // Kitten 4 - Alert and focused
    &[
        "   /\\_/\\  ",
        "  ( o.o ) ",
        "  /| 4 |\\ ",
        "  \\_   _/ ",
        "    \\_/   ",
    ],
    // Kitten 5 - Happy and playful
    &[
        "   /\\_/\\  ",
        "  ( ^_^ ) ",
        "  (  5  ) ",
        "  /  |  \\ ",
        " <__|__|__>",
    ],
    // Kitten 6 - Calm and observant
    &[
        "   /\\_/\\  ",
        "  ( o.o ) ",
        "  (  6  ) ",
        "  /  |  \\ ",
        "  \\__^__/",
    ],
    // Kitten 7 - Sleepy and peaceful
    &[
        "   /\\_/\\  ",
        "  ( -.- ) ",
        "  (  7  ) ",
        "  /  |  \\ ",
        " <__v__v__>",
    ],
    // Kitten 8 - Excited and energetic
    &[
        "   /\\_/\\  ",
        "  ( >w< ) ",
        "  (  8  ) ",
        "  /  |  \\ ",
        "  \\__|__/",
    ],
    // Kitten 9 - Mysterious and wise
    &[
        "   /\\_/\\  ",
        "  ( o_o ) ",
        "  (  9  ) ",
        "  /  |  \\ ",
        " <__*__*__>",
    ],
];

impl Default for KittyArts {
    fn default() -> Self {
        Self {
            arts: DEFAULT_KITTIES,
        }
    }
}

/// Get the ASCII art for a specific kitty number (0-8, corresponding to cats 1-9)
pub fn art_for_cell(value: usize) -> KittyArt {
    if value < DEFAULT_KITTIES.len() {
        DEFAULT_KITTIES[value]
    } else {
        // Fallback for invalid values
        &[" ? ", "???", " ? "]
    }
}

/// Convert kitty art to a single string with newlines
pub fn art_to_string(art: KittyArt) -> String {
    art.join("\n")
}

/// System to initialize the kitty arts resource
pub fn setup_kitty_arts(mut commands: Commands) {
    commands.init_resource::<KittyArts>();
    info!("Kitty ASCII arts initialized with {} designs", DEFAULT_KITTIES.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_art_for_cell() {
        let art = art_for_cell(0); // First kitty (number 1)
        assert_eq!(art.len(), 5); // Should have 5 lines
        assert!(art[0].contains("/\\_/\\")); // Should contain cat ears
        assert!(art[4].contains("1")); // Should contain number 1
    }

    #[test]
    fn test_all_kitties_have_correct_structure() {
        for (i, kitty) in DEFAULT_KITTIES.iter().enumerate() {
            // Most kitties have 5 lines, but kitten 2 has 4 lines
            let expected_lines = if i == 1 { 4 } else { 5 };
            assert_eq!(kitty.len(), expected_lines, "Kitty {} should have {} lines", i + 1, expected_lines);
            assert!(kitty[0].contains("/\\_"), "Kitty {} should have ears", i + 1);
            // Check that the kitty contains its number somewhere
            let kitty_text = kitty.join("");
            assert!(kitty_text.contains(&(i + 1).to_string()), "Kitty {} should contain its number", i + 1);
        }
    }

    #[test]
    fn test_art_to_string() {
        let art = art_for_cell(0);
        let art_string = art_to_string(art);
        assert!(art_string.contains("\n")); // Should contain newlines
        assert!(art_string.contains("o.o")); // Should contain the face (first kitten has o.o eyes)
    }
}
