mod neopets;
mod battle;
mod utils;

use neopets::load_neopets;
use battle::battle_loop;
use utils::inspect_seed;

fn main() {
    //let neopets_set = load_neopets("assets/neopets.json");
    //battle_loop(&neopets_set[0], &neopets_set[1]);
    inspect_seed(42, 10);
}
