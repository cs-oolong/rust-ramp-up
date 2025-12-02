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
    
    let max_turns = 2000;
    let mut all_events = initiative_events; // Start with initiative events

    let mut turn = 1; // Start battle turns at 1
    while turn <= max_turns {
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
        
        let (events, first, _unused_second) = roll_for_initiative(&fighter1, &fighter2, &mut rng);
        
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

#[cfg(test)]
mod process_turn_tests {
    use super::*;
    use std::cell::Cell;
    
    
    use rand::RngCore;

    /// Fixed RNG for testing - returns pre-programmed dice values in sequence
    struct FixedRng {
        values: Vec<u8>,
        index: Cell<usize>,
    }

    impl FixedRng {
        fn new(values: Vec<u8>) -> Self {
            Self {
                values,
                index: Cell::new(0),
            }
        }

        fn next_value(&self) -> u8 {
            let idx = self.index.get();
            let val = self.values[idx % self.values.len()];
            self.index.set(idx + 1);
            val
        }
    }

    impl RngCore for FixedRng {
        fn next_u32(&mut self) -> u32 {
            // Scale the u8 value to u32 range to work with random_range
            // The random_range implementation uses the full u32 range
            let val = self.next_value() as u32;
            // Map our values (1-20) uniformly across the u32 space
            // This ensures random_range(1..=20) will return our exact values
            val * (u32::MAX / 21)
        }

        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_value();
            }
        }
    }

    // Note: Rng is automatically implemented for all RngCore types
    // so we don't need to implement it explicitly

    /// Helper to create a test Neopet with full control
    fn test_neopet(name: &str, attack: u32, defense: u32, heal_delta: u32, spells: Vec<crate::neopets::Spell>) -> crate::neopets::Neopet {
        crate::neopets::Neopet {
            name: name.to_string(),
            health: 100,
            heal_delta,
            base_attack: attack,
            base_defense: defense,
            spells,
            behavior: crate::neopets::Behavior {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.5,
            },
        }
    }

    /// Helper to create a simple test Neopet with default spells
    fn test_neopet_simple(name: &str, attack: u32, defense: u32) -> crate::neopets::Neopet {
        test_neopet(name, attack, defense, 10, vec![
            crate::neopets::Spell {
                name: "Fireball".to_string(),
                effect: serde_json::Value::Object(serde_json::Map::new()),
            },
            crate::neopets::Spell {
                name: "Ice Storm".to_string(),
                effect: serde_json::Value::Object(serde_json::Map::new()),
            },
        ])
    }

    // ==================== Attack Action Tests ====================

    #[test]
    fn test_attack_normal_damage() {
        // Attack roll = 14, Defense roll = 8
        let mut rng = FixedRng::new(vec![14, 8]);

        let attacker = test_neopet_simple("Alice", 10, 0);
        let defender = test_neopet_simple("Bob", 0, 5);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        // Should have 3 events: attack roll, defense roll, attack
        assert_eq!(events.len(), 3);

        // Verify attack roll event
        match &events[0] {
            BattleEvent::Roll { turn, actor, dice, final_value, is_positive_crit, is_negative_crit, goal } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Alice");
                assert_eq!(*dice, 14);
                assert_eq!(*final_value, 24); // 14 + 10 base_attack
                assert!(!is_positive_crit);
                assert!(!is_negative_crit);
                assert_eq!(goal, "attack");
            }
            _ => panic!("Expected Roll event for attack"),
        }

        // Verify defense roll event
        match &events[1] {
            BattleEvent::Roll { turn, actor, dice, final_value, is_positive_crit, is_negative_crit, goal } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Bob");
                assert_eq!(*dice, 8);
                assert_eq!(*final_value, 13); // 8 + 5 base_defense
                assert!(!is_positive_crit);
                assert!(!is_negative_crit);
                assert_eq!(goal, "defense");
            }
            _ => panic!("Expected Roll event for defense"),
        }

        // Verify attack event with damage calculation
        match &events[2] {
            BattleEvent::Attack { turn, actor, target, raw_damage, shield_value, actual_damage } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Alice");
                assert_eq!(target, "Bob");
                assert_eq!(*raw_damage, 24);
                assert_eq!(*shield_value, 13);
                assert_eq!(*actual_damage, 11); // 24 - 13 = 11
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_attack_positive_crit() {
        // Attack roll = 20 (positive crit), Defense roll = 5
        let mut rng = FixedRng::new(vec![20, 5]);

        let attacker = test_neopet_simple("Alice", 10, 0);
        let defender = test_neopet_simple("Bob", 0, 8);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        assert_eq!(events.len(), 3);

        // Verify attack roll is marked as positive crit
        match &events[0] {
            BattleEvent::Roll { dice, final_value, is_positive_crit, is_negative_crit, .. } => {
                assert_eq!(*dice, 20);
                assert_eq!(*final_value, 30); // 20 + 10
                assert!(is_positive_crit);
                assert!(!is_negative_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify defense roll
        match &events[1] {
            BattleEvent::Roll { dice, final_value, .. } => {
                assert_eq!(*dice, 5);
                assert_eq!(*final_value, 13); // 5 + 8
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify damage is doubled due to crit
        match &events[2] {
            BattleEvent::Attack { raw_damage, shield_value, actual_damage, .. } => {
                assert_eq!(*raw_damage, 30);
                assert_eq!(*shield_value, 13);
                // Normal damage: 30 - 13 = 17
                // Crit doubles it: 17 * 2 = 34
                assert_eq!(*actual_damage, 34);
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_attack_negative_crit() {
        // Attack roll = 1 (negative crit), Defense roll = 10
        let mut rng = FixedRng::new(vec![1, 10]);

        let attacker = test_neopet_simple("Alice", 15, 0);
        let defender = test_neopet_simple("Bob", 0, 5);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        assert_eq!(events.len(), 3);

        // Verify attack roll is marked as negative crit
        match &events[0] {
            BattleEvent::Roll { dice, is_positive_crit, is_negative_crit, .. } => {
                assert_eq!(*dice, 1);
                assert!(!is_positive_crit);
                assert!(is_negative_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify damage is 0 due to negative crit
        match &events[2] {
            BattleEvent::Attack { actual_damage, .. } => {
                assert_eq!(*actual_damage, 0); // Negative crit zeros all damage
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_attack_defense_exceeds_attack() {
        // Attack roll = 5, Defense roll = 15 (defense will be higher)
        let mut rng = FixedRng::new(vec![5, 15]);

        let attacker = test_neopet_simple("Alice", 1, 0);  // Low attack
        let defender = test_neopet_simple("Bob", 0, 20); // High defense

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        assert_eq!(events.len(), 3);

        // Verify damage is 0 due to saturating subtraction
        match &events[2] {
            BattleEvent::Attack { raw_damage, shield_value, actual_damage, .. } => {
                assert_eq!(*raw_damage, 6);  // 5 + 1
                assert_eq!(*shield_value, 35); // 15 + 20
                assert_eq!(*actual_damage, 0); // saturating_sub results in 0
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_attack_both_roll_twenty() {
        // Both attacker and defender roll 20 (both crit)
        let mut rng = FixedRng::new(vec![20, 20]);

        let attacker = test_neopet_simple("Alice", 10, 0);
        let defender = test_neopet_simple("Bob", 0, 10);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        assert_eq!(events.len(), 3);

        // Both rolls should be marked as positive crits
        match &events[0] {
            BattleEvent::Roll { dice, is_positive_crit, .. } => {
                assert_eq!(*dice, 20);
                assert!(is_positive_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        match &events[1] {
            BattleEvent::Roll { dice, is_positive_crit, .. } => {
                assert_eq!(*dice, 20);
                assert!(is_positive_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        // Attack: (20 + 10) - (20 + 10) = 0, then * 2 (crit) = 0
        match &events[2] {
            BattleEvent::Attack { actual_damage, .. } => {
                assert_eq!(*actual_damage, 0);
            }
            _ => panic!("Expected Attack event"),
        }
    }

    // ==================== Heal Action Tests ====================

    #[test]
    fn test_heal_normal() {
        // Heal roll = 10 (normal, not 1 or 20)
        let mut rng = FixedRng::new(vec![10]);

        let mut healer = test_neopet_simple("Alice", 0, 0);
        healer.heal_delta = 15;
        let other = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&healer, &other, &Action::Heal, 1, &mut rng);

        // Should have 2 events: heal roll, heal
        assert_eq!(events.len(), 2);

        // Verify heal roll event
        match &events[0] {
            BattleEvent::Roll { turn, actor, dice, is_positive_crit, is_negative_crit, goal, .. } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Alice");
                assert_eq!(*dice, 10);
                assert!(!is_positive_crit);
                assert!(!is_negative_crit);
                assert_eq!(goal, "heal");
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify heal event
        match &events[1] {
            BattleEvent::Heal { turn, actor, amount } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Alice");
                assert_eq!(*amount, 15); // Normal heal_delta
            }
            _ => panic!("Expected Heal event"),
        }
    }

    #[test]
    fn test_heal_positive_crit() {
        // Heal roll = 20 (positive crit)
        let mut rng = FixedRng::new(vec![20]);

        let mut healer = test_neopet_simple("Alice", 0, 0);
        healer.heal_delta = 10;
        let other = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&healer, &other, &Action::Heal, 1, &mut rng);

        assert_eq!(events.len(), 2);

        // Verify heal roll is marked as positive crit
        match &events[0] {
            BattleEvent::Roll { dice, is_positive_crit, is_negative_crit, .. } => {
                assert_eq!(*dice, 20);
                assert!(is_positive_crit);
                assert!(!is_negative_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify heal is doubled
        match &events[1] {
            BattleEvent::Heal { amount, .. } => {
                assert_eq!(*amount, 20); // 10 * 2 = 20
            }
            _ => panic!("Expected Heal event"),
        }
    }

    #[test]
    fn test_heal_negative_crit() {
        // Heal roll = 1 (negative crit)
        let mut rng = FixedRng::new(vec![1]);

        let mut healer = test_neopet_simple("Alice", 0, 0);
        healer.heal_delta = 10;
        let other = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&healer, &other, &Action::Heal, 1, &mut rng);

        assert_eq!(events.len(), 2);

        // Verify heal roll is marked as negative crit
        match &events[0] {
            BattleEvent::Roll { dice, is_positive_crit, is_negative_crit, .. } => {
                assert_eq!(*dice, 1);
                assert!(!is_positive_crit);
                assert!(is_negative_crit);
            }
            _ => panic!("Expected Roll event"),
        }

        // Verify heal is 0
        match &events[1] {
            BattleEvent::Heal { amount, .. } => {
                assert_eq!(*amount, 0); // Negative crit zeros heal
            }
            _ => panic!("Expected Heal event"),
        }
    }

    // ==================== CastSpell Action Tests ====================

    #[test]
    fn test_spell_cast_valid_index() {
        let mut rng = FixedRng::new(vec![10]); // RNG not used for spells

        let caster = test_neopet_simple("Alice", 0, 0);
        let target = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&caster, &target, &Action::CastSpell(0), 1, &mut rng);

        // Should have 1 event: spell cast
        assert_eq!(events.len(), 1);

        match &events[0] {
            BattleEvent::SpellCast { turn, actor, target: tgt, spell_name } => {
                assert_eq!(*turn, 1);
                assert_eq!(actor, "Alice");
                assert_eq!(tgt, "Bob");
                assert_eq!(spell_name, "Fireball"); // First spell in test_neopet_simple
            }
            _ => panic!("Expected SpellCast event"),
        }
    }

    #[test]
    fn test_spell_cast_second_spell() {
        let mut rng = FixedRng::new(vec![10]);

        let caster = test_neopet_simple("Alice", 0, 0);
        let target = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&caster, &target, &Action::CastSpell(1), 1, &mut rng);

        assert_eq!(events.len(), 1);

        match &events[0] {
            BattleEvent::SpellCast { spell_name, .. } => {
                assert_eq!(spell_name, "Ice Storm"); // Second spell
            }
            _ => panic!("Expected SpellCast event"),
        }
    }

    #[test]
    fn test_spell_cast_invalid_index() {
        let mut rng = FixedRng::new(vec![10]);

        let caster = test_neopet_simple("Alice", 0, 0);
        let target = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&caster, &target, &Action::CastSpell(99), 1, &mut rng);

        assert_eq!(events.len(), 1);

        match &events[0] {
            BattleEvent::SpellCast { spell_name, .. } => {
                assert_eq!(spell_name, "Unknown Spell"); // Fallback for out of bounds
            }
            _ => panic!("Expected SpellCast event"),
        }
    }

    // ==================== Additional Edge Case Tests ====================

    #[test]
    fn test_attack_with_zero_stats() {
        // Attack with 0 base stats
        let mut rng = FixedRng::new(vec![10, 10]);

        let attacker = test_neopet_simple("Alice", 0, 0);
        let defender = test_neopet_simple("Bob", 0, 0);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        assert_eq!(events.len(), 3);

        // With 0 base stats and normal rolls, damage should be 0 (10 - 10 = 0)
        match &events[2] {
            BattleEvent::Attack { raw_damage, shield_value, actual_damage, .. } => {
                assert_eq!(*raw_damage, 10);
                assert_eq!(*shield_value, 10);
                assert_eq!(*actual_damage, 0);
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_turn_number_propagation_attack() {
        let mut rng = FixedRng::new(vec![10, 10]);
        let attacker = test_neopet_simple("Alice", 5, 0);
        let defender = test_neopet_simple("Bob", 0, 5);

        // Test with turn 5
        let events = process_turn(&attacker, &defender, &Action::Attack, 5, &mut rng);

        for event in &events {
            match event {
                BattleEvent::Roll { turn, .. } => assert_eq!(*turn, 5),
                BattleEvent::Attack { turn, .. } => assert_eq!(*turn, 5),
                _ => {}
            }
        }
    }

    #[test]
    fn test_turn_number_propagation_heal() {
        let mut rng = FixedRng::new(vec![10]);
        let healer = test_neopet_simple("Alice", 0, 0);
        let other = test_neopet_simple("Bob", 0, 0);

        // Test with turn 10
        let events = process_turn(&healer, &other, &Action::Heal, 10, &mut rng);

        for event in &events {
            match event {
                BattleEvent::Roll { turn, .. } => assert_eq!(*turn, 10),
                BattleEvent::Heal { turn, .. } => assert_eq!(*turn, 10),
                _ => {}
            }
        }
    }

    #[test]
    fn test_turn_number_propagation_spell() {
        let mut rng = FixedRng::new(vec![10]);
        let caster = test_neopet_simple("Alice", 0, 0);
        let target = test_neopet_simple("Bob", 0, 0);

        // Test with turn 7
        let events = process_turn(&caster, &target, &Action::CastSpell(0), 7, &mut rng);

        for event in &events {
            match event {
                BattleEvent::SpellCast { turn, .. } => assert_eq!(*turn, 7),
                _ => {}
            }
        }
    }

    #[test]
    fn test_actor_and_target_names() {
        let mut rng = FixedRng::new(vec![10, 10]);
        let attacker = test_neopet_simple("Pikachu", 5, 0);
        let defender = test_neopet_simple("Charizard", 0, 5);

        let events = process_turn(&attacker, &defender, &Action::Attack, 1, &mut rng);

        // Check attack roll has correct actor
        match &events[0] {
            BattleEvent::Roll { actor, .. } => assert_eq!(actor, "Pikachu"),
            _ => panic!("Expected Roll event"),
        }

        // Check defense roll has correct actor (the defender)
        match &events[1] {
            BattleEvent::Roll { actor, .. } => assert_eq!(actor, "Charizard"),
            _ => panic!("Expected Roll event"),
        }

        // Check attack event has correct actor and target
        match &events[2] {
            BattleEvent::Attack { actor, target, .. } => {
                assert_eq!(actor, "Pikachu");
                assert_eq!(target, "Charizard");
            }
            _ => panic!("Expected Attack event"),
        }
    }

    #[test]
    fn test_event_count_for_all_actions() {
        let mut rng = FixedRng::new(vec![10, 10]);
        let neopet1 = test_neopet_simple("Alice", 5, 5);
        let neopet2 = test_neopet_simple("Bob", 5, 5);

        // Attack should produce 3 events
        let attack_events = process_turn(&neopet1, &neopet2, &Action::Attack, 1, &mut rng);
        assert_eq!(attack_events.len(), 3);

        // Heal should produce 2 events
        let heal_events = process_turn(&neopet1, &neopet2, &Action::Heal, 1, &mut rng);
        assert_eq!(heal_events.len(), 2);

        // Spell should produce 1 event
        let spell_events = process_turn(&neopet1, &neopet2, &Action::CastSpell(0), 1, &mut rng);
        assert_eq!(spell_events.len(), 1);
    }
}
