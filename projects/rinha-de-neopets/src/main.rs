mod neopets;
mod battle;

use neopets::load_neopets;
use battle::battle_loop;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    battle_loop(&neopets_set[0], &neopets_set[1]);
}
