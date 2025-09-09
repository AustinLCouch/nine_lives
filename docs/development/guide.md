# Development Guide

This guide covers the essential commands and workflows for developing Nine Lives Cat Sudoku.

## Development Commands

### Running the Game
```bash
# Run the game (primary development command)
cargo run

# Run in release mode for performance testing
cargo build --release
```

### Testing
```bash
# Run all tests across the workspace
cargo test

# Run tests for specific crate
cargo test -p nine_lives_core
cargo test -p nine_lives_ui  
cargo test -p nine_lives_controller

# Run integration smoke tests
cargo test --test smoke

# Run tests with output (useful for debugging)
cargo test -- --nocapture
```

### Building and Checking
```bash
# Check compilation without building
cargo check

# Build all workspace members
cargo build

# Build specific crate
cargo build -p nine_lives_core
```

## Development Workflow

### Setting Up Your Environment
1. **Install Rust**: Ensure you have Rust 1.70+ installed
2. **Clone Repository**: `git clone [repo-url] && cd nine_lives`
3. **First Build**: `cargo check` to verify everything compiles
4. **Run Tests**: `cargo test` to ensure all tests pass
5. **Launch Game**: `cargo run` to test the application

### Making Changes

When modifying the codebase, follow the MVC architecture:

- **Game Logic Changes**: Modify `nine_lives_core/`
- **UI/Visual Changes**: Modify `nine_lives_ui/` 
- **Input/Control Changes**: Modify `nine_lives_controller/`

Always maintain the unidirectional dependency flow: Controller → UI → Core

### Testing Your Changes
1. **Unit Tests**: `cargo test -p [crate_name]` for the specific crate you modified
2. **Integration Tests**: `cargo test --test smoke` for end-to-end validation
3. **Manual Testing**: `cargo run` to test the application interactively
4. **Full Test Suite**: `cargo test` before committing changes

## Code Quality

### Before Committing
```bash
# Ensure everything compiles
cargo check

# Run all tests
cargo test

# Build in release mode to catch release-specific issues
cargo build --release
```

### Development Tips
- Use `cargo watch -x run` for auto-recompilation during development
- Run `cargo clippy` for additional linting (if installed)
- Use `cargo fmt` to format code consistently (if installed)

## Troubleshooting

### Common Issues
- **Compilation Errors**: Check that you're in the workspace root and all dependencies are available
- **Test Failures**: Run `cargo test -- --nocapture` to see detailed test output
- **Runtime Errors**: Check console output when running `cargo run`

### Performance
- **Slow Compilation**: Use `cargo check` for faster feedback during development
- **Debug vs Release**: Use debug builds for development, release builds for performance testing

## Architecture Guidelines

When adding new features:

1. **Start with Core**: Implement pure game logic in `nine_lives_core`
2. **Add UI Layer**: Create UI components in `nine_lives_ui`
3. **Wire Controller**: Add input handling in `nine_lives_controller`
4. **Test Each Layer**: Write tests at each architectural layer
5. **Integration Test**: Add end-to-end tests in `tests/`

For detailed architecture information, see [MVC Overview](../architecture/mvc_overview.md).

## Related Documentation

- **[MVC Overview](../architecture/mvc_overview.md)** - Detailed architecture explanation
- **[Testing Strategy](testing.md)** - Comprehensive testing approaches
- **[Main WARP.md](../../WARP.md)** - Warp.dev specific guidance
