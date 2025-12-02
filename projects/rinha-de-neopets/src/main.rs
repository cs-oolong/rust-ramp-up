mod battle;
mod display;
mod neopets;

use battle::battle_loop;
use display::display_battle_events;
use neopets::load_neopets;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    let events = battle_loop(&neopets_set[0], &neopets_set[1], &mut rand::rng());
    
    // Display the battle with beautiful formatting
    display_battle_events(&events);
}
