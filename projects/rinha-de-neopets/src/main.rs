mod battle;
mod neopets;

use battle::battle_loop;
use neopets::load_neopets;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    battle_loop(&neopets_set[0], &neopets_set[1], &mut rand::rng());
}
