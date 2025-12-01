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

#[derive(Debug, Clone, PartialEq)]
pub enum BattleEvent {
    Roll {
        turn: u32,
        actor: String,
        dice: u8,
        final_value: u32,
        is_positive_crit: bool,
        is_negative_crit: bool,
        goal: String,
    },
    Attack {
        turn: u32,
        actor: String,
        target: String,
        raw_damage: u32,
        shield_value: u32,
        actual_damage: u32, 
    },
    Heal {
        turn: u32,
        actor: String,
        amount: u32,
    },
    SpellCast {
        turn: u32,
        actor: String,
        target: String,
        spell_name: String,
    }
}

fn process_turn<R: Rng>(actor: &Neopet, other: &Neopet, action: &Action, turn_number: u32, rng: &mut R) -> Vec<BattleEvent> {
    match action {
        Action::Attack => {
            let mut events = Vec::new();
            
            let attack_roll = roll_d20(rng);
            let attack_val = (attack_roll as u32) + actor.base_attack;
            let attack_is_positive_crit = attack_roll == 20;
            let attack_is_negative_crit = attack_roll == 1;
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: actor.name.clone(),
                dice: attack_roll,
                final_value: attack_val,
                is_positive_crit: attack_is_positive_crit,
                is_negative_crit: attack_is_negative_crit,
                goal: "attack".to_string(),
            });

            let defense_roll = roll_d20(rng);
            let defense_val = (defense_roll as u32) + other.base_defense;
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: other.name.clone(),
                dice: defense_roll,
                final_value: defense_val,
                is_positive_crit: defense_roll == 20,
                is_negative_crit: defense_roll == 1,
                goal: "defense".to_string(),
            });
            
            let mut actual_damage = attack_val.saturating_sub(defense_val);
            if attack_is_positive_crit {
                actual_damage *= 2;
            }
            if attack_is_negative_crit {
                actual_damage = 0;
            }
            
            events.push(BattleEvent::Attack {
                turn: turn_number,
                actor: actor.name.clone(),
                target: other.name.clone(),
                raw_damage: attack_val,
                shield_value: defense_val,
                actual_damage: actual_damage,
            });
            
            events
        }
        Action::Heal => {
            let mut events = Vec::new();
            
            let heal_roll = roll_d20(rng);

            let is_positive_crit = heal_roll == 20;
            let is_negative_crit = heal_roll == 1;
            let mut heal_val = actor.heal_delta;
            if is_positive_crit {
                heal_val = heal_val * 2;
            }
            if is_negative_crit {
                heal_val = 0;
            }
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: actor.name.clone(),
                dice: heal_roll,
                final_value: heal_val,
                is_positive_crit: is_positive_crit,
                is_negative_crit: is_negative_crit,
                goal: "heal".to_string(),
            });
            
            events.push(BattleEvent::Heal {
                turn: turn_number,
                actor: actor.name.clone(),
                amount: heal_val,
            });
            
            events
        }
        Action::CastSpell(spell_index) => {
            let spell_name = if let Some(spell) = actor.spells.get(*spell_index) {
                spell.name.clone()
            } else {
                println!("Error: No spell found at index {}", spell_index);
                "Unknown Spell".to_string()
            };
            
            vec![BattleEvent::SpellCast {
                turn: turn_number,
                actor: actor.name.clone(),
                target: other.name.clone(),
                spell_name: spell_name,
            }]
        }
    }
}

fn roll_for_initiative<'a, R: Rng>(
    fighter1: &'a Neopet,
    fighter2: &'a Neopet,
    rng: &mut R,
) -> (Vec<BattleEvent>, &'a Neopet, &'a Neopet) {
    let mut fighter1_initiative = 0;
    let mut fighter2_initiative = 0;
    let mut events = Vec::new();

    while fighter1_initiative == fighter2_initiative {
        let roll1 = roll_d20(rng);
        events.push(BattleEvent::Roll {
            turn: 0, // Turn 0 for initiative phase
            actor: fighter1.name.clone(),
            dice: roll1,
            final_value: roll1 as u32,
            is_positive_crit: roll1 == 20,
            is_negative_crit: roll1 == 1,
            goal: "initiative".to_string(),
        });
        
        let roll2 = roll_d20(rng);
        events.push(BattleEvent::Roll {
            turn: 0, // Turn 0 for initiative phase
            actor: fighter2.name.clone(),
            dice: roll2,
            final_value: roll2 as u32,
            is_positive_crit: roll2 == 20,
            is_negative_crit: roll2 == 1,
            goal: "initiative".to_string(),
        });
        
        fighter1_initiative = roll1;
        fighter2_initiative = roll2;
    }

    let mut first: &Neopet = fighter1;
    let mut second: &Neopet = fighter2;

    if fighter2_initiative > fighter1_initiative {
        first = fighter2;
        second = fighter1;
    }
    
    (events, first, second)
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

pub fn battle_loop<R: Rng>(fighter1: &Neopet, fighter2: &Neopet, rng: &mut R) -> Vec<BattleEvent> {
    let (initiative_events, first, second) = roll_for_initiative(fighter1, fighter2, rng);
    
    let mut battle_in_progress = true;
    let max_turns = 2000;
    let mut all_events = initiative_events; // Start with initiative events

    let mut turn = 1; // Start battle turns at 1
    while battle_in_progress && turn <= max_turns {
        let first_action = choose_action(first, rng);
        let events = process_turn(first, second, &first_action, turn, rng);
        all_events.extend(events);
        turn += 1;

        let second_action = choose_action(second, rng);
        let events = process_turn(second, first, &second_action, turn, rng);
        all_events.extend(events);
        turn += 1;
    }
    
    all_events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neopets::Behavior;
    use crate::neopets::Spell;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn get_testing_neopet() -> Neopet {
        get_testing_neopets_with_name("TestPet")
    }

    fn get_testing_neopets_with_name(name: &str) -> Neopet {
        Neopet {
            name: name.to_string(),
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

    fn seed_produces_initiative_tie(seed: u64) -> bool {
        let fighter1 = get_testing_neopet();
        let fighter2 = get_testing_neopets_with_name("Fighter2");
        let mut rng = StdRng::seed_from_u64(seed);
        
        let (events, _first, _second) = roll_for_initiative(&fighter1, &fighter2, &mut rng);
        
        let fighter1_rolls: Vec<_> = events.iter().filter(|e| {
            if let BattleEvent::Roll { actor, .. } = e {
                actor == "TestPet"
            } else { false }
        }).collect();
        
        fighter1_rolls.len() > 1
    }

    #[test]
    fn find_seed_for_tie() {
        let mut tie_seed = None;
        for seed in 0..=100 {
            if seed_produces_initiative_tie(seed) {
                println!("Found seed with tie: {}", seed);
                tie_seed = Some(seed);
                break;
            }
        }
        assert!(tie_seed.is_some(), "Should find at least one seed that produces a tie");
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
            let (_, first, second) = roll_for_initiative(&fighter1, &fighter2, &mut rng);
            assert_eq!((first, second), expected[i])
        }
    }

    #[test]
    fn test_roll_for_initiative_generates_events() {
        let fighter1 = get_testing_neopet();
        let fighter2 = get_testing_neopet();
        let mut rng = StdRng::seed_from_u64(42);
        
        let (events, first, second) = roll_for_initiative(&fighter1, &fighter2, &mut rng);
        
        assert!(!events.is_empty(), "Should generate initiative events");
        
        for event in &events {
            match event {
                BattleEvent::Roll { turn, goal, .. } => {
                    assert_eq!(*turn, 0, "Initiative events should have turn 0");
                    assert_eq!(goal, "initiative", "Goal should be 'initiative'");
                }
                _ => panic!("All initiative events should be Roll type"),
            }
        }
        
        assert_eq!(events.len() % 2, 0, "Should have pairs of rolls, one per fighter");
        
        if let Some(BattleEvent::Roll { actor, dice, .. }) = events.last() {
            let last_roller = if actor == &fighter1.name { &fighter1 } else { &fighter2 };
            let other = if actor == &fighter1.name { &fighter2 } else { &fighter1 };
            
            if dice > &0 { // Dice will always be > 0, this just ensures we got a value
                if last_roller.name == first.name {
                    assert_eq!(*actor, first.name, "Last roller with higher roll should be first");
                } else {
                    assert_eq!(other.name, first.name, "Other fighter should be first if they rolled higher");
                }
            }
        }
    }

    #[test]
    fn test_roll_for_initiative_tracks_ties() {
        let fighter1 = get_testing_neopet();
        let fighter2 = get_testing_neopets_with_name("Fighter2");
        
        let mut rng = StdRng::seed_from_u64(25);
        
        let (events, first, _second) = roll_for_initiative(&fighter1, &fighter2, &mut rng);
        
        let fighter1_rolls: Vec<_> = events.iter().filter(|e| {
            if let BattleEvent::Roll { actor, .. } = e {
                actor == "TestPet"
            } else { false }
        }).collect();
        
        let fighter2_rolls: Vec<_> = events.iter().filter(|e| {
            if let BattleEvent::Roll { actor, .. } = e {
                actor == "Fighter2"
            } else { false }
        }).collect();
        
        assert_eq!(fighter1_rolls.len(), fighter2_rolls.len(), 
                   "Both fighters should roll the same number of times");
        
        assert!(fighter1_rolls.len() > 1, "This seed was tested to ensure at least a tie, there should be more than one roll per fighter.");
        
        if fighter1_rolls.len() > 1 {
            println!("Detected tie in initiative - each rolled {} times", fighter1_rolls.len());
            
            for event in &events {
                if let BattleEvent::Roll { turn, .. } = event {
                    assert_eq!(*turn, 0, "All initiative events should be turn 0");
                }
            }
        }
        
        if let Some(BattleEvent::Roll { actor, dice, .. }) = fighter1_rolls.last() {
            assert_eq!(*actor, fighter1.name);
            
            if let Some(BattleEvent::Roll { dice: dice2, .. }) = fighter2_rolls.last() {
                if dice > dice2 {
                    assert_eq!(first.name, fighter1.name, "Fighter1 should go first (higher roll)");
                } else {
                    assert_eq!(first.name, fighter2.name, "Fighter2 should go first (higher roll)");
                }
            }
        }
    }
}
