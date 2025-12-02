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
    
    // Create and use the enhanced battle display
    let mut battle_display = BattleDisplay::new(fighter1, fighter2);
    battle_display.display_battle_events(&events);
}
