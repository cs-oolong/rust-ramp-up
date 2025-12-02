mod battle;
mod display;
mod neopets;

use battle::battle_loop;
use display::{BattleDisplay, BattleDisplayConfig};
use neopets::load_neopets;
use colored::*;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    let fighter1 = &neopets_set[0];
    let fighter2 = &neopets_set[1];
    
    println!("{} vs {}", fighter1.name, fighter2.name);
    
    // Demo: Show the difference between fast and animated modes
    demo_animation_comparison(fighter1, fighter2);
    
    // Run the actual battle with full animations
    println!("\n\n{}", "=== REAL BATTLE WITH EPIC ANIMATIONS ===".bright_yellow().bold());
    let events = battle_loop(fighter1, fighter2, &mut rand::rng());
    
    let config = BattleDisplayConfig::default();
    let battle_display = BattleDisplay::with_config(fighter1, fighter2, config);
    
    battle_display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
    battle_display.display_battle_summary(&events);
}

fn demo_animation_comparison(fighter1: &neopets::Neopet, fighter2: &neopets::Neopet) {
    // Create sample events for demo
    let sample_events = vec![
        battle::BattleEvent::Roll {
            turn: 1,
            actor: "Pikachu".to_string(),
            dice: 20,
            final_value: 25,
            is_positive_crit: true,
            is_negative_crit: false,
            goal: "attack".to_string(),
        },
        battle::BattleEvent::Attack {
            turn: 1,
            actor: "Pikachu".to_string(),
            target: "Charizard".to_string(),
            raw_damage: 25,
            shield_value: 10,
            actual_damage: 15,
        },
        battle::BattleEvent::SpellCast {
            turn: 2,
            actor: "Charizard".to_string(),
            target: "Pikachu".to_string(),
            spell_name: "Fire Blast".to_string(),
        },
    ];
    
    // Example 1: Fast mode (no animations)
    println!("\n{}", "Example 1: Fast Mode (No Animations)".bright_cyan().bold());
    println!("{}", "─".repeat(60).bright_black());
    
    let mut fast_config = BattleDisplayConfig::default();
    fast_config.enable_delays = false;
    fast_config.use_spinners = false;
    fast_config.streaming_effect = false;
    
    let display_fast = BattleDisplay::with_config(fighter1, fighter2, fast_config);
    display_fast.display_battle_events(&sample_events, Some((85, 110)));
    
    // Example 2: Full animations
    println!("\n\n{}", "Example 2: Epic Animations (Full Effects)".bright_cyan().bold());
    println!("{}", "─".repeat(60).bright_black());
    
    let full_config = BattleDisplayConfig::default();
    let display_full = BattleDisplay::with_config(fighter1, fighter2, full_config);
    display_full.display_battle_events(&sample_events, Some((85, 110)));
    
    println!("\n{}", "=== ANIMATION COMPARISON COMPLETE ===".bright_yellow().bold());
}