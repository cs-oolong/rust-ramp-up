use crate::neopets::Neopet;
use rand::Rng;

fn roll_d20() -> u8 {
    rand::rng().random_range(1..=20)
}

#[derive(Debug, PartialEq)]
enum Action {
    Attack,
    CastSpell(usize),
    Heal,
}

fn choose_action<R: Rng>(neopet: &Neopet, rng: &mut R) -> Action {
    let roll: f64 = rng.random();
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
    use crate::neopets::Spell;
    use crate::neopets::Behavior;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_roll_d20_always_within_range() {
        for _unused in 0..100 {
            let result = roll_d20();
            assert!(result >= 1 && result <= 20);
        }
    }

    #[test]
    fn test_choose_action_respects_neopet_probabilities() {
        // StdRng with seed 42 outputs this, as verified with `inspect_seed`.
        // Outputs
        // [0] = 0.526557 -> heal
        // [1] = 0.542725 -> heal
        // [2] = 0.636465 -> spell 1
        // [3] = 0.405902 -> heal
        // [4] = 0.034343 -> attack
        // [5] = 0.414957 -> heal
        // [6] = 0.737424 -> spell 1
        // [7] = 0.849252 -> spell 2
        // [8] = 0.131279 -> attack
        // [9] = 0.003252 -> attack
        // [10] = 0.932145 -> spell 3
        let mut rng = StdRng::seed_from_u64(42);
        let neopet = Neopet {
            name: "TestPet".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![
                Spell {
                    name: "Spell1".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                },
                Spell {
                    name: "Spell2".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                },
                Spell {
                    name: "Spell3".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                }
            ],
            behavior: Behavior {
                attack_chance: 0.40, // 0 to 0.40 -> attack
                spell_chances: vec![ // 0.60 to 1.0 -> spell
                    0.15, // 0.60 to 0.75 -> spell 1
                    0.15, // 0.75 to 0.90 -> spell 2
                    0.10, // 0.90 to 1.0 -> spell 3
                ],
                heal_chance: 0.20, // 0.40 to 0.60 -> heal
            }
        };

        let expected_action_sequence = vec![
            Action::Heal,
            Action::Heal,
            Action::CastSpell(0),
            Action::Heal,
            Action::Attack,
            Action::Heal,
            Action::CastSpell(0),
            Action::CastSpell(1),
            Action::Attack,
            Action::Attack,
            Action::CastSpell(2),
        ];

        for i in 0..11 {
            assert_eq!(choose_action(&neopet, &mut rng), expected_action_sequence[i]);
        }
    }
}
