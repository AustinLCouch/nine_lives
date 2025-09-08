# Nine Lives Cat Sudoku - API Analysis & Contracts

## Current API Surface Analysis

### nine_lives_core (Model Layer)

#### Current Public Types:
- `BoardState` - Main game state with cells and cell types
- `GameState` - Simple Playing/Won enum
- `CellType` - Given/Player distinction
- `GRID_SIZE` - Constant (9)

#### Current Public Methods:
- `BoardState::new()` - Create empty board
- `BoardState::clear()` - Reset to empty
- `BoardState::cycle_cell()` - Player input handling
- `BoardState::is_valid_placement()` - Sudoku rule validation
- `BoardState::get_conflicts()` - Find rule violations
- `BoardState::is_complete()` - Win detection
- `BoardState::compute_game_state()` - State calculation
- `BoardState::generate_*_puzzle()` - Easy/medium/hard generation
- `BoardState::is_given_cell()` - Check if cell is part of original puzzle

#### Areas for Extension (Advanced Features):
✅ **Stable** - No breaking changes needed:
- Basic validation and rule checking
- Cell state management
- Win detection

🔧 **Needs Extension** - Backwards compatible additions:
- GameState enum (needs more states)
- Move history tracking
- Hint system support
- Solution storage
- Statistics tracking

⚠️ **Potential Breaking Changes**:
- Puzzle generation API (needs more configuration)
- Save/load serialization support

### nine_lives_ui (View Layer)

#### Current Public Types:
- `Cell` - UI component for grid cells
- `ClearButton`, `NewGameButton` - UI components
- `CatEmojis` - ASCII art resource
- `AppState` - Loading/Ready application phases

#### Current Public Systems:
- `setup_cat_emojis()` - Load art resources
- `setup_grid()` - Create UI layout
- `update_cell_text()` - Sync cell content with board
- `update_cell_colors()` - Visual feedback for conflicts/completion
- `update_button_colors()` - Button hover effects
- `update_cell_hover_effects()` - Cell interaction feedback
- `transition_to_ready()` - State management

#### Areas for Extension:
✅ **Stable** - Core UI layout and basic feedback systems

🔧 **Needs Extension**:
- New UI components (Hint button, Undo/Redo, Settings menu)
- Theme system and visual customization
- Animation system integration
- Additional game state visual feedback

### nine_lives_controller (Controller Layer)

#### Current Public Functions:
- `cell_click_system()` - Handle cell interactions
- `clear_button_system()` - Clear board functionality
- `new_game_button_system()` - Generate new puzzles
- `game_state_system()` - Keep game state in sync
- `add_controller()` - Register systems with app
- `run_game()` - Main entry point

#### Areas for Extension:
🔧 **Major Extensions Needed**:
- Keyboard input handling (Undo/Redo shortcuts)
- Hint button system
- Settings menu controller
- Save/load game handling
- Statistics tracking

## Architectural Contracts for Advanced Features

### Contract 1: Dependency Flow Preservation
**Rule**: Controller → UI → Core (unidirectional dependencies)

- Core crate MUST remain UI-agnostic (no Bevy UI dependencies beyond Resource derive)
- UI crate MAY depend on Core for game state types
- Controller crate MAY depend on both UI and Core
- New features MUST respect this hierarchy

### Contract 2: State Management Pattern
**Rule**: All game state lives in Core, UI reflects state reactively

```rust
// ✅ Correct: Core owns state, UI reflects it
#[derive(Resource)]
struct GameHistory { moves: Vec<Move> } // Core

fn update_undo_button_availability(history: Res<GameHistory>) {} // UI

// ❌ Wrong: UI managing game state
struct UiHistoryPanel { undo_available: bool } // UI
```

### Contract 3: Pure Function Bias
**Rule**: Prefer pure functions in Core for testability

```rust
// ✅ Correct: Pure function
pub fn can_undo(history: &GameHistory) -> bool { !history.moves.is_empty() }

// ❌ Avoid: Side effects in core logic
pub fn undo_move(&mut self) { /* UI updates mixed with logic */ }
```

### Contract 4: Event-Driven Architecture
**Rule**: Use Bevy events for cross-layer communication

```rust
// ✅ Correct: Event-based communication
#[derive(Event)]
pub struct UndoRequested;

pub fn handle_undo_key(input: Res<ButtonInput<KeyCode>>, mut events: EventWriter<UndoRequested>) {}
pub fn process_undo(mut events: EventReader<UndoRequested>, mut board: ResMut<BoardState>) {}
```

## Feature Implementation Strategy

### Phase 1: Core Extensions (Backwards Compatible)
1. **Enhanced GameState**
   ```rust
   #[derive(Resource, Default)]
   pub enum GameState {
       #[default]
       Playing,
       Won,
       Paused,
   }
   ```

2. **Move History System**
   ```rust
   #[derive(Clone, Debug)]
   pub struct Move {
       pub row: usize,
       pub col: usize,
       pub old_value: Option<usize>,
       pub new_value: Option<usize>,
       pub timestamp: std::time::Instant,
   }

   #[derive(Resource, Default)]
   pub struct GameHistory {
       pub moves: Vec<Move>,
       pub undo_index: usize,
   }
   ```

3. **Hint System Support**
   ```rust
   #[derive(Resource)]
   pub struct Solution {
       pub cells: [[usize; GRID_SIZE]; GRID_SIZE],
   }

   pub fn get_next_hint(board: &BoardState, solution: &Solution) -> Option<(usize, usize, usize)> {}
   ```

### Phase 2: UI Extensions
1. **New Components**
   ```rust
   #[derive(Component)]
   pub struct UndoButton;

   #[derive(Component)]
   pub struct RedoButton;

   #[derive(Component)]
   pub struct HintButton;
   ```

2. **Theme System**
   ```rust
   #[derive(Resource)]
   pub struct Theme {
       pub name: String,
       pub primary_color: Color,
       pub secondary_color: Color,
       pub cat_art_set: Vec<String>,
   }
   ```

### Phase 3: Controller Integration
1. **Input System Extensions**
2. **Event Handlers for New Features**
3. **Save/Load Integration**

## Breaking Change Management

### Planned Breaking Changes:
1. **Puzzle Generation API** - Will change from simple methods to configuration-based
   ```rust
   // Old API (will be deprecated)
   pub fn generate_easy_puzzle(&mut self)

   // New API (v2.0.0)
   pub fn generate_puzzle(&mut self, config: GeneratorConfig) -> Result<Solution, GenerationError>
   ```

### Mitigation Strategy:
- Keep old methods as deprecated wrappers
- Provide migration guide in CHANGELOG.md
- Use feature flags for new functionality
- Extensive testing for compatibility

## Quality Gates

### Testing Requirements:
- ✅ All existing tests must continue passing
- ✅ New features must have >90% test coverage
- ✅ Integration tests for end-to-end workflows
- ✅ Property-based tests for puzzle generation
- ✅ Benchmark tests for performance regression detection

### Performance Requirements:
- ✅ Puzzle generation: <100ms for Easy, <500ms for Hard
- ✅ Undo/Redo operations: <1ms
- ✅ UI updates: 60fps maintenance
- ✅ Memory usage: <50MB for typical gameplay

This analysis provides the foundation for implementing all advanced features while maintaining architectural integrity and backwards compatibility where possible.
