# Testing Strategy

Nine Lives Cat Sudoku follows a comprehensive testing approach across all architectural layers.

## Test Architecture

### Unit Tests by Crate
- **`nine_lives_core`**: 19 unit tests covering pure game logic
- **`nine_lives_ui`**: 2 unit tests for UI components  
- **`nine_lives_controller`**: 3 unit tests for event handling
- **Integration Tests**: 7 tests covering end-to-end workflows

**Total: 31 comprehensive tests**

## Running Tests

### All Tests
```bash
# Run complete test suite
cargo test

# Run with output for debugging
cargo test -- --nocapture
```

### Crate-Specific Tests
```bash
# Core game logic tests
cargo test -p nine_lives_core

# UI component tests  
cargo test -p nine_lives_ui

# Controller/input tests
cargo test -p nine_lives_controller
```

### Integration Tests
```bash
# Smoke tests for basic functionality
cargo test --test smoke
```

## Test Categories

### Core Logic Tests (`nine_lives_core`)

#### Board State Management
- Cell cycling and state changes
- Board clearing and initialization
- Game state transitions

#### Sudoku Validation
- Rule checking (rows, columns, boxes)
- Conflict detection
- Win condition validation

#### Puzzle Generation
- Algorithm correctness
- Difficulty progression
- Uniqueness validation
- Performance benchmarks

#### Move History System
- Undo/redo functionality
- History buffer management
- State consistency

#### Hint System
- Hint allocation by difficulty
- Hint depletion tracking
- Solution-based hint generation

### UI Tests (`nine_lives_ui`)

#### Component Rendering
- Cell display and styling
- Button state management
- Theme application

#### Visual Feedback
- Conflict highlighting
- Hover effects
- State transitions

### Controller Tests (`nine_lives_controller`)

#### Input Processing
- Click event handling
- Button interactions
- Keyboard shortcuts

#### System Integration
- Event propagation
- State synchronization
- Resource management

### Integration Tests (`tests/`)

#### End-to-End Workflows
- Complete game sessions
- New game generation
- Undo/redo cycles
- Hint system usage
- Win condition flow

## Test Quality Standards

### Coverage Requirements
- **Core Logic**: >95% coverage (mission-critical game rules)
- **UI Components**: >80% coverage (visual feedback)
- **Integration**: 100% of major user workflows

### Test Types

#### Unit Tests
```rust
#[test]
fn test_sudoku_rule_validation() {
    let mut board = BoardState::new();
    board.cycle_cell(0, 0); // Place first cat
    board.cycle_cell(0, 1); // Place same cat in same row
    
    let conflicts = board.get_conflicts();
    assert!(!conflicts.is_empty()); // Should detect conflict
}
```

#### Property-Based Tests
```rust
#[test]
fn test_puzzle_generation_properties() {
    for _ in 0..100 {
        let mut board = BoardState::new();
        board.generate_easy_puzzle();
        
        // Properties that should always hold
        assert!(board.is_valid_board());
        assert!(board.has_unique_solution());
        assert!(board.count_givens() >= 35);
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_complete_game_workflow() {
    // Test full game cycle: generate → solve → win
    let mut board = BoardState::new();
    board.generate_easy_puzzle();
    
    // Simulate solving
    while !board.is_complete() {
        // ... solving logic
    }
    
    assert_eq!(board.compute_game_state(), GameState::Won);
}
```

### Performance Tests
```rust
#[test]
fn test_generation_performance() {
    let start = std::time::Instant::now();
    
    let mut board = BoardState::new();
    board.generate_hard_puzzle();
    
    let duration = start.elapsed();
    assert!(duration < Duration::from_millis(500), 
            "Puzzle generation too slow: {:?}", duration);
}
```

## Test Data Management

### Fixtures
- Pre-generated valid puzzles for consistent testing
- Known conflict scenarios
- Edge cases (empty boards, full boards, invalid states)

### Mock Objects
- Bevy system testing with mock apps
- UI component testing with mock resources
- Event simulation for controller tests

## Continuous Integration

### Pre-Commit Checks
```bash
# Standard CI pipeline
cargo check
cargo test
cargo build --release
```

### Test Categories in CI
1. **Fast Tests** (< 1 second): Unit tests, basic validation
2. **Medium Tests** (< 10 seconds): Integration tests, generation tests
3. **Slow Tests** (< 60 seconds): Stress tests, performance benchmarks

### Coverage Tracking
- Measure test coverage per crate
- Track coverage trends over time
- Enforce minimum coverage thresholds

## Debugging Test Failures

### Common Debugging Techniques
```bash
# Show test output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Show backtraces on panic
RUST_BACKTRACE=1 cargo test

# Run tests in single thread for debugging
cargo test -- --test-threads=1
```

### Test Isolation
- Each test creates fresh state
- No shared mutable state between tests
- Deterministic test ordering

## Future Testing Improvements

### Phase 1 (Current)
✅ Comprehensive unit and integration tests  
✅ Performance benchmarks  
✅ Property-based testing for puzzle generation

### Phase 2 (Planned)
- [ ] Visual regression testing for UI changes
- [ ] Fuzzing tests for robustness
- [ ] Automated performance regression detection
- [ ] Cross-platform testing automation

### Phase 3 (Advanced)
- [ ] User behavior simulation tests
- [ ] Accessibility testing automation
- [ ] Load testing for resource usage
- [ ] Mutation testing for test quality validation

## Best Practices

### Writing Good Tests
1. **Clear Names**: Test names should describe the expected behavior
2. **Single Responsibility**: Each test should verify one specific behavior
3. **Arrange-Act-Assert**: Structure tests clearly
4. **Independent**: Tests should not depend on each other
5. **Fast**: Optimize for quick feedback cycles

### Test Maintenance
- Keep tests updated with code changes
- Remove obsolete tests when features are removed
- Refactor tests when they become unclear
- Document complex test scenarios

This testing strategy ensures Nine Lives Cat Sudoku maintains high quality and reliability across all features and architectural layers.
