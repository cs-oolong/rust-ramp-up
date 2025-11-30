use crate::neopets::Neopet;
use rand::Rng;

fn roll_d20<R: Rng>(rng: &mut R) -> u8 {
    rng.random_range(1..=20)
}

#[derive(Debug, PartialEq)]
enum Action {
    Attack,
    CastSpell(usize),
    Heal,
}

fn roll_for_initiative<'a, R: Rng>(
    fighter1: &'a Neopet,
    fighter2: &'a Neopet,
    rng: &mut R,
) -> (&'a Neopet, &'a Neopet) {
    let mut fighter1_initiative = 0;
    let mut fighter2_initiative = 0;

    while fighter1_initiative == fighter2_initiative {
        fighter1_initiative = roll_d20(rng);
        fighter2_initiative = roll_d20(rng);
    }

    let mut first: &Neopet = fighter1;
    let mut second: &Neopet = fighter2;

    if fighter2_initiative > fighter1_initiative {
        first = fighter2;
        second = fighter1;
    }
    (first, second)
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

pub fn battle_loop<R: Rng>(fighter1: &Neopet, fighter2: &Neopet, rng: &mut R) {
    let (first, second) = roll_for_initiative(fighter1, fighter2, rng);

    let mut battle_in_progress = true;
    let max_turns = 10;

    let mut turn = 0;
    while battle_in_progress && turn < max_turns {
        let first_action = choose_action(first, rng);
        println!("{}: {:?}", first.name, first_action);
        turn += 1;
        println!("turn {turn}");
        let second_action = choose_action(second, rng);
        println!("{}: {:?}", second.name, second_action);
        turn += 2;
        println!("turn {turn}");
    }
    // Turns start, respecting the initiative result.
}

mod tests {
    use super::*;
    use crate::neopets::Behavior;
    use crate::neopets::Spell;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn get_testing_neopet() -> Neopet {
        Neopet {
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
                },
            ],
            behavior: Behavior {
                attack_chance: 0.40, // 0 to 0.40 -> attack
                spell_chances: vec![
                    // 0.60 to 1.0 -> spell
                    0.15, // 0.60 to 0.75 -> spell 1
                    0.15, // 0.75 to 0.90 -> spell 2
                    0.10, // 0.90 to 1.0 -> spell 3
                ],
                heal_chance: 0.20, // 0.40 to 0.60 -> heal
            },
        }
    }

    #[test]
    fn test_roll_d20_always_within_range() {
        let mut rng = rand::rng();
        for _unused in 0..100 {
            let result = roll_d20(&mut rng);
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
        let neopet = get_testing_neopet();

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
            assert_eq!(
                choose_action(&neopet, &mut rng),
                expected_action_sequence[i]
            );
        }
    }

    #[test]
    fn test_roll_for_initiative_respects_bigger_roll() {
        let fighter1 = get_testing_neopet();
        let fighter2 = get_testing_neopet();

        // 3, 11, 5, 11, 18, 13, 20, 9, 20, 1
        let mut rng = StdRng::seed_from_u64(42);

        let expected = vec![
            (&fighter1, &fighter2),
            (&fighter1, &fighter2),
            (&fighter2, &fighter1),
            (&fighter2, &fighter1),
            (&fighter2, &fighter1),
        ];

        for i in 0..5 {
            assert_eq!(roll_for_initiative(&fighter1, &fighter2, &mut rng), expected[i])
        }
    }
}
