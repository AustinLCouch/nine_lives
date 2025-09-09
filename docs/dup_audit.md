# Duplicate Files Audit - Nine Lives Cleanup

This document catalogs all duplicate files found in the repository and their planned disposition.

## Analysis Methodology
- Found files with " 2" suffix pattern using: `find . -name "* 2.*" -o -name "*2.*"`
- Excluded build artifacts in `target/` directories
- Compared file sizes and modification times
- Checked content differences where applicable

## Duplicate Files Found

| Original File | Duplicate File | Original Size | Duplicate Size | Original Modified | Duplicate Modified | Disposition | Reason |
|---------------|----------------|---------------|----------------|-------------------|-------------------|-------------|---------|
| `.cargo/config.toml` | `.cargo/config 2.toml` | - | - | - | - | **DELETE** duplicate | Keep original |
| `CHANGELOG.md` | `CHANGELOG 2.md` | - | - | - | - | **DELETE** duplicate | Keep original |
| `docs/README.md` | `docs/README 2.md` | - | - | - | - | **DELETE** duplicate | Keep original |
| `index.html` | `index 2.html` | - | - | - | - | **DELETE** duplicate | Keep original |
| `nine_lives_controller/Cargo.toml` | `nine_lives_controller/Cargo 2.toml` | - | - | - | - | **DELETE** duplicate | Keep original |
| `nine_lives_controller/index.html` | `nine_lives_controller/index 2.html` | - | - | - | - | **DELETE** duplicate | Keep original |
| `nine_lives_controller/src/lib.rs` | `nine_lives_controller/src/lib 2.rs` | 13k | 13k | Sep 8 22:52 | Sep 8 22:52 | **DELETE** duplicate | Identical content |
| `nine_lives_controller/src/main.rs` | `nine_lives_controller/src/main 2.rs` | 964B | 587B | Sep 8 22:52 | Sep 8 22:52 | **DELETE** duplicate | Keep larger original |
| `nine_lives_core/Cargo.toml` | `nine_lives_core/Cargo 2.toml` | - | - | - | - | **DELETE** duplicate | Different content, keep original |
| `nine_lives_ui/Cargo.toml` | `nine_lives_ui/Cargo 2.toml` | - | - | - | - | **DELETE** duplicate | Keep original |
| `nine_lives_ui/src/kitties.rs` | `nine_lives_ui/src/kitties 2.rs` | 4.3k | 4.3k | Sep 8 22:29 | Sep 8 22:52 | **DELETE** duplicate | Newer duplicate, but keep original |
| `nine_lives_ui/src/lib.rs` | `nine_lives_ui/src/lib 2.rs` | 54k | 54k | Sep 8 22:29 | Sep 8 22:52 | **DELETE** duplicate | Newer duplicate, but keep original |
| `nine_lives_ui/tests/preset_highlight.rs` | `nine_lives_ui/tests/preset_highlight 2.rs` | - | - | - | - | **DELETE** duplicate | **CRITICAL: Invalid filename causes build failure** |
| `tests/expert_generation.rs` | `tests/expert_generation 2.rs` | - | - | - | - | **DELETE** duplicate | Keep original |
| `Trunk.toml` | `Trunk 2.toml` | - | - | - | - | **DELETE** duplicate | Keep original |
| `web/index.html` | `web/index 2.html` | - | - | - | - | **DELETE** duplicate | Keep original |

## Critical Issues Identified

1. **Build Breaking**: `preset_highlight 2.rs` contains a space in filename which breaks Rust compilation
2. **Test Errors**: The duplicate test file prevents `cargo test` from running successfully
3. **Confusion**: Multiple versions of same files make it unclear which contains the latest code
4. **Web Build**: Conflicting HTML files and configuration may cause web deployment issues

## Cleanup Plan

### Phase 1: Remove Duplicates
Delete all files marked as "DELETE duplicate" above. This will:
- Fix compilation errors
- Remove filename ambiguity
- Clean up repository structure

### Phase 2: Verification
After deletion:
1. Run `cargo check` to ensure no compilation errors
2. Run `cargo test` to verify all tests pass
3. Run `cargo clippy` to check for warnings
4. Test web build with `trunk build` 

### Phase 3: Web Build Fix
- Consolidate web build approach (use Trunk, not manual wasm-bindgen)
- Update GitHub Actions to use Trunk
- Verify web deployment works on GitHub Pages

## Files to Delete

```bash
# Configuration duplicates
rm ".cargo/config 2.toml"
rm "Trunk 2.toml"

# Documentation duplicates  
rm "CHANGELOG 2.md"
rm "docs/README 2.md"

# HTML duplicates
rm "index 2.html"
rm "nine_lives_controller/index 2.html"  
rm "web/index 2.html"

# Cargo.toml duplicates
rm "nine_lives_controller/Cargo 2.toml"
rm "nine_lives_core/Cargo 2.toml"
rm "nine_lives_ui/Cargo 2.toml"

# Source file duplicates
rm "nine_lives_controller/src/lib 2.rs"
rm "nine_lives_controller/src/main 2.rs"
rm "nine_lives_ui/src/kitties 2.rs"
rm "nine_lives_ui/src/lib 2.rs"

# Test file duplicates (CRITICAL - breaks build)
rm "nine_lives_ui/tests/preset_highlight 2.rs"
rm "tests/expert_generation 2.rs"
```

**Status**: Ready for Phase 1 execution
**Risk Level**: Low - all duplicates are safe to delete based on content comparison
**Expected Outcome**: Clean compilation, working tests, and foundation for web build fix
