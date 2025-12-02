mod battle;
mod display;
mod neopets;

use battle::battle_loop;
use display::{BattleDisplay, BattleDisplayConfig};
use neopets::load_neopets;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    let fighter1 = &neopets_set[0];
    let fighter2 = &neopets_set[1];
    
    // Run the epic animated battle
    let events = battle_loop(fighter1, fighter2, &mut rand::rng());
    
    let config = BattleDisplayConfig::default();
    let battle_display = BattleDisplay::with_config(fighter1, fighter2, config);
    
    battle_display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
    battle_display.display_battle_summary(&events);
}