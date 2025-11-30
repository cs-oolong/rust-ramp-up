use rand::Rng;
use crate::neopets::Neopet;

fn roll_d20() -> u8 {
    rand::rng().random_range(1..=20)
}

#[derive(Debug)]
enum Action {
    Attack,
    CastSpell(usize),
    Heal,
}

fn choose_action<R: Rng>(neopet: &Neopet, rng: &mut R) -> Action {
    let roll : f64 = rng.random();
    if roll < neopet.behavior.attack_chance {
        Action::Attack
    } else if roll < neopet.behavior.attack_chance + neopet.behavior.heal_chance {
        Action::Heal
    } else {
        let spell_roll = roll - (neopet.behavior.attack_chance + neopet.behavior.heal_chance);
        let mut cumulative = 0.0;
        for (index, &chance) in neopet.behavior.spell_chances.iter().enumerate() {
            cumulative += chance;
            if spell_roll < cumulative {
                return Action::CastSpell(index);
            }
        }
        // Fallback (shouldn't happen, just in case)
        Action::Attack
    }
}

pub fn battle_loop(fighter1: &Neopet, fighter2: &Neopet) {
    let mut fighter1_initiative = 0;
    let mut fighter2_initiative = 0;

    while fighter1_initiative == fighter2_initiative {
        fighter1_initiative = roll_d20();
        fighter2_initiative = roll_d20();
    }

    let mut first: &Neopet = fighter1;
    let mut second: &Neopet = fighter2;

    if fighter2_initiative > fighter1_initiative {
        first = fighter2;
        second = fighter1;
    }

    println!("fighter1 = {fighter1}, initiative = {fighter1_initiative}");
    println!("fighter2 = {fighter2}, initiative = {fighter2_initiative}");
    println!("first = {first}, second = {second}");

    let mut battle_in_progress = true;
    let max_turns = 10;

    let mut turn = 0;
    while battle_in_progress && turn < max_turns {
        let first_action = choose_action(first, &mut rand::rng());
        println!("{}: {:?}", first.name, first_action);
        turn += 1;
        println!("turn {turn}");
        let second_action = choose_action(second, &mut rand::rng());
        println!("{}: {:?}", second.name, second_action);
        turn += 2;
        println!("turn {turn}");
    }
    // Turns start, respecting the initiative result.
}

mod tests {
    use super::*;

    #[test]
    fn test_roll_d20_always_within_range() {
        for _unused in 0..100 {
            let result = roll_d20();
            assert!(result >= 1 && result <= 20);
        }
    }
}