mod battle;
mod neopets;
mod utils;

use battle::battle_loop;
use neopets::load_neopets;
use utils::inspect_seed;
use utils::inspect_seed_for_d20;

fn main() {
    //let neopets_set = load_neopets("assets/neopets.json");
    //battle_loop(&neopets_set[0], &neopets_set[1], &mut rand::rng());
    inspect_seed_for_d20(42, 10);
}
