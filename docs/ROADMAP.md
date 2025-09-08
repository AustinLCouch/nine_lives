# Nine Lives Cat Sudoku - Development Roadmap

## Current State Analysis

**✅ Completed Foundation:**
- Complete MVC architecture with three separate crates
- Basic 9x9 grid UI with clickable cells  
- Cell cycling through 9 cat ASCII art designs
- Clear board functionality
- Beautiful UI with proper sudoku box coloring
- Comprehensive unit tests for core logic
- Bevy ECS integration and state management
- Proper separation of concerns

**❌ Missing Core Game Logic:**
- Sudoku validation rules
- Win/loss conditions
- Puzzle generation
- Visual feedback for game states

---

## Feature Prioritization Matrix

| Feature | User Impact | Technical Complexity | Priority Score | Category |
|---------|-------------|---------------------|---------------|----------|
| **Sudoku Validation Logic** | 5 | 2 | 8 | 🎯 Critical |
| **Solution Checking (Win Condition)** | 5 | 1 | 9 | 🎯 Critical |
| **Basic Visual Feedback** | 4 | 2 | 6 | 🎯 Critical |
| **Puzzle Generation (Easy)** | 4 | 3 | 5 | 🏆 High Priority |
| **Game State Management** | 3 | 2 | 4 | 🏆 High Priority |
| **Enhanced Visual Feedback** | 3 | 2 | 4 | 🏆 High Priority |
| **Persistence (Save/Load)** | 3 | 3 | 3 | 🎲 Medium Priority |
| **Hint System** | 2 | 3 | 2 | 🎲 Medium Priority |
| **Multiple Difficulty Levels** | 2 | 4 | 1 | 🔮 Low Priority |
| **Timer & Scoring** | 2 | 2 | 2 | 🔮 Low Priority |
| **Better Integration Tests** | 1 | 2 | 1 | 🔮 Low Priority |

**Scoring Formula:** `Priority Score = User Impact × 2 - Technical Complexity`

---

## 🚀 Development Milestones

### **Milestone 1: MVP Playable Sudoku** 
*Target: Make it an actual game*

**Goals:**
- Transform from "cat grid" to playable Sudoku
- Players can win or identify mistakes
- Basic game feedback

**Features:**
1. **Sudoku Validation Logic** (`nine_lives_core`)
   - `is_valid_placement(row, col, value) -> bool`
   - `get_conflicts() -> Vec<(usize, usize)>`
   - `is_complete() -> bool`
   - Validate row, column, and 3x3 box constraints

2. **Solution Checking** (`nine_lives_core`)
   - Win condition detection
   - Game completion state

3. **Basic Visual Feedback** (`nine_lives_ui`)
   - Red highlighting for conflicting cells
   - Green highlighting/celebration for completed puzzle
   - Update UI systems to show conflicts

**Affected Crates:** `nine_lives_core` (logic), `nine_lives_ui` (feedback)
**Estimated Effort:** 1-2 weeks
**Acceptance Criteria:**
- Players can see rule violations highlighted in red
- Game shows celebration when puzzle is correctly solved
- All existing functionality continues to work

---

### **Milestone 2: Generated Puzzles & Enhanced UX**
*Target: Replayable game with polish*

**Goals:**
- Generate actual Sudoku puzzles
- Better game state management
- Enhanced visual polish

**Features:**
1. **Puzzle Generation** (`nine_lives_core`)
   - Basic backtracking algorithm
   - Generate valid, solvable puzzles
   - Easy difficulty (35-40 given numbers)
   - "New Game" button functionality

2. **Game State Management** (`nine_lives_core` + `nine_lives_ui`)
   - GameState enum: Playing, Won, Invalid
   - State transitions and UI updates
   - Restart functionality

3. **Enhanced Visual Feedback** (`nine_lives_ui`)
   - Highlight completed rows/columns/boxes
   - Given numbers vs player numbers (different styling)
   - Hover effects and animations
   - Better cell highlighting

**Affected Crates:** All three crates
**Estimated Effort:** 2-3 weeks  
**Acceptance Criteria:**
- Players can generate new puzzles with "New Game" button
- Clear visual distinction between given and player numbers
- Smooth visual feedback for game progress

---

### **Milestone 3: Replayability & Polish** *(Optional/Future)*
*Target: Feature-complete game*

**Goals:**
- Multiple difficulty levels
- Helper features for players
- Data persistence

**Features:**
1. **Multiple Difficulty Levels** (`nine_lives_core`)
   - Easy (35-40 givens), Medium (30-35), Hard (25-30)
   - Difficulty selection UI
   - Different generation strategies

2. **Hint System** (`nine_lives_core` + `nine_lives_ui`)
   - "Next valid move" hints
   - Highlight available numbers for selected cell
   - Mistake correction suggestions

3. **Persistence & Scoring** (`nine_lives_controller`)
   - Save/load current game state
   - Timer and move counter
   - Personal best times
   - Statistics tracking

4. **Robust Integration Tests** (`tests/`)
   - End-to-end game flow tests
   - Puzzle generation validation
   - UI interaction tests

**Affected Crates:** All three crates + new test suite
**Estimated Effort:** 3-4 weeks
**Acceptance Criteria:**
- Players can choose difficulty levels
- Game saves progress automatically
- Comprehensive test coverage for all game flows

---

## 🛠️ Implementation Details

### **Quick Wins (Start Here):**

#### 1. Sudoku Validation Logic
```rust
// Add to nine_lives_core/src/lib.rs
impl BoardState {
    /// Check if placing a value at position would be valid
    pub fn is_valid_placement(&self, row: usize, col: usize, value: usize) -> bool {
        // Check row constraint
        for c in 0..GRID_SIZE {
            if c != col && self.cells[row][c] == Some(value) {
                return false;
            }
        }
        
        // Check column constraint  
        for r in 0..GRID_SIZE {
            if r != row && self.cells[r][col] == Some(value) {
                return false;
            }
        }
        
        // Check 3x3 box constraint
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if (r != row || c != col) && self.cells[r][c] == Some(value) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Get all positions with Sudoku rule violations
    pub fn get_conflicts(&self) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if let Some(value) = self.cells[row][col] {
                    if !self.is_valid_placement(row, col, value) {
                        conflicts.push((row, col));
                    }
                }
            }
        }
        
        conflicts
    }
    
    /// Check if the puzzle is completely and correctly solved
    pub fn is_complete(&self) -> bool {
        // All cells must be filled
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.cells[row][col].is_none() {
                    return false;
                }
            }
        }
        
        // No conflicts should exist
        self.get_conflicts().is_empty()
    }
}
```

#### 2. Visual Feedback Integration
```rust
// Add to nine_lives_ui/src/lib.rs  
pub fn update_cell_colors(
    board: Res<BoardState>,
    cell_query: Query<(&Cell, &mut BackgroundColor)>,
) {
    let conflicts = board.get_conflicts();
    let conflict_set: HashSet<(usize, usize)> = conflicts.into_iter().collect();
    
    for (cell, mut bg_color) in &cell_query {
        let base_color = get_cell_background_color(cell.row, cell.col);
        
        if conflict_set.contains(&(cell.row, cell.col)) {
            // Red tint for conflicts
            *bg_color = BackgroundColor(Color::srgb(1.0, 0.6, 0.6));
        } else if board.is_complete() {
            // Green tint for completion
            *bg_color = BackgroundColor(Color::srgb(0.6, 1.0, 0.6));
        } else {
            // Normal color
            *bg_color = BackgroundColor(base_color);
        }
    }
}
```

### **Architecture Considerations:**

- **Core Crate:** All game logic stays pure and UI-agnostic
- **UI Crate:** Only presentation and Bevy-specific code
- **Controller Crate:** Orchestration and event handling
- **Maintain unidirectional dependencies:** Controller → UI → Core

### **Testing Strategy:**
- Unit tests for all new core logic methods
- Integration tests for complete game flows  
- Bevy system tests for UI behavior
- Property-based tests for puzzle generation

---

## 🎯 Immediate Next Steps

**If you want to start coding right now:**

1. **Add validation logic** to `nine_lives_core/src/lib.rs`
2. **Add visual feedback system** to `nine_lives_ui/src/lib.rs` 
3. **Update controller** to use new validation in `nine_lives_controller/src/lib.rs`
4. **Test the new functionality** manually and with unit tests

**Quick validation:** After implementing validation, you should be able to:
- Fill in some cats and see conflicts highlighted in red
- Fill the entire grid correctly and see celebration feedback
- Clear board and start over

This roadmap transforms your excellent technical foundation into a fully playable and polished cat-themed Sudoku game! 🐱

---

*Last Updated: 2025-09-08*
*Project: Nine Lives Cat Sudoku*
*Architecture: MVC with Bevy ECS*
