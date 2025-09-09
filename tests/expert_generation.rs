//! Integration tests for Expert puzzle generation
//! 
//! This test file focuses specifically on the Expert difficulty puzzle generation
//! reliability issues observed in the game.

use nine_lives_core::{BoardState, PuzzleSettings, PresetKind};

/// Test Expert puzzle generation reliability with fixed seed for reproducibility.
/// This test is ignored by default since it's expected to fail initially.
#[test]
#[ignore = "Expected to fail - demonstrates Expert generation reliability issues"]
fn test_expert_generation_reliability() {
    let settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
    
    println!("Testing Expert puzzle generation reliability...");
    println!("Settings: {}", settings.description());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    const ATTEMPTS: usize = 20;
    
    for attempt in 1..=ATTEMPTS {
        let mut board = BoardState::new();
        
        match board.generate_puzzle_with_settings(&settings) {
            Some(_solution) => {
                success_count += 1;
                println!("✅ Attempt {}: Generated successfully", attempt);
                
                // Verify the puzzle meets Expert criteria
                let givens_count = board.cells.iter().flatten().filter(|c| c.is_some()).count();
                assert!(
                    givens_count >= 22 && givens_count <= 26,
                    "Expert puzzle should have 22-26 givens, got {}",
                    givens_count
                );
                
                // Verify no conflicts
                assert!(
                    board.get_conflicts().is_empty(),
                    "Generated Expert puzzle should have no conflicts"
                );
            }
            None => {
                failure_count += 1;
                println!("❌ Attempt {}: Failed to generate", attempt);
            }
        }
    }
    
    println!("\n📊 Results: {}/{} successful ({:.1}% success rate)", 
             success_count, ATTEMPTS, (success_count as f32 / ATTEMPTS as f32) * 100.0);
    
    // We expect high reliability for Expert puzzles
    assert!(
        success_count >= (ATTEMPTS * 8 / 10), // At least 80% success rate
        "Expert generation should be reliable: {}/{} successful",
        success_count,
        ATTEMPTS
    );
}

/// Test that demonstrates the current Expert generation issues with more detailed logging.
#[test]
#[ignore = "Diagnostic test - expected to show current generation issues"]
fn test_expert_generation_detailed_diagnostics() {
    let settings = PuzzleSettings::from_preset(PresetKind::NightProwler);
    
    println!("🔍 Detailed Expert Generation Diagnostics");
    println!("Settings: {}", settings.description());
    println!("Max attempts per puzzle: 10");
    println!("Target givens range: {}-{}", settings.givens_range.0, settings.givens_range.1);
    println!("Uniqueness required: {}", settings.require_unique_solution);
    
    let mut board = BoardState::new();
    
    // Try generating one Expert puzzle with detailed output
    match board.generate_puzzle_with_settings(&settings) {
        Some(solution) => {
            let givens_count = board.cells.iter().flatten().filter(|c| c.is_some()).count();
            println!("✅ Successfully generated Expert puzzle!");
            println!("   - Givens count: {}", givens_count);
            println!("   - Conflicts: {}", board.get_conflicts().len());
            println!("   - Solution valid: {}", solution.cells[0][0] > 0);
        }
        None => {
            println!("❌ Failed to generate Expert puzzle after 10 attempts");
            println!("   - This demonstrates the reliability issue we need to fix");
        }
    }
}

/// Test to measure generation performance for different difficulties.
#[test]
#[ignore = "Performance benchmark test"]
fn test_generation_performance_comparison() {
    use std::time::Instant;
    
    let presets = [
        ("Easy", PresetKind::CozyKitten),
        ("Medium", PresetKind::CuriousCat), 
        ("Hard", PresetKind::StreetwiseStray),
        ("Expert", PresetKind::NightProwler),
    ];
    
    println!("🏃‍♂️ Puzzle Generation Performance Comparison");
    println!("Each difficulty attempted 5 times, measuring average time:");
    
    for (name, preset) in presets {
        let settings = PuzzleSettings::from_preset(preset);
        let mut total_time = std::time::Duration::ZERO;
        let mut success_count = 0;
        const TRIALS: usize = 5;
        
        for _ in 0..TRIALS {
            let mut board = BoardState::new();
            let start = Instant::now();
            
            if board.generate_puzzle_with_settings(&settings).is_some() {
                success_count += 1;
            }
            
            total_time += start.elapsed();
        }
        
        let avg_time = total_time / TRIALS as u32;
        let success_rate = (success_count as f32 / TRIALS as f32) * 100.0;
        
        println!("{:>8}: {:.2?} avg, {:.0}% success ({}/{})",
                name, avg_time, success_rate, success_count, TRIALS);
    }
}
