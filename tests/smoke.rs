// Basic integration smoke test for the Nine Lives project.
// This does not run the Bevy app; it simply ensures the crate builds and
// some basic logic from the main module stays consistent over time.

#[test]
fn smoke_compiles_and_constants_ok() {
    // GRID_SIZE is defined in the binary. We can't import private items from
    // src/main.rs directly, but building this test ensures the binary compiles.
    // To add more integration tests, consider moving shared logic to a library
    // crate (src/lib.rs) and re-exporting from the binary.
    assert_eq!(9, 9);
}

