mod neopets;

use neopets::load_neopets;

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    for n in neopets_set {
        println!("{:#?}", n);
    }
}
