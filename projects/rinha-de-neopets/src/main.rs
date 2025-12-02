mod battle;
mod display;
mod neopets;

use battle::battle_loop;
use display::BattleDisplay;
use neopets::load_neopets;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    let fighter1 = &neopets_set[0];
    let fighter2 = &neopets_set[1];
    
    println!("{} vs {}", fighter1.name, fighter2.name);
    
    let events = battle_loop(fighter1, fighter2, &mut rand::rng());
    
    // Create battle display (stateless - just for presentation)
    let battle_display = BattleDisplay::new(fighter1, fighter2);
    
    // Display the battle with initial health state
    battle_display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
    
    // Display battle summary (calculated from events, not from state)
    battle_display.display_battle_summary(&events);
}
