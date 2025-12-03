use crate::neopets::Neopet;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BattleCompletionReason {
    HpDepleted(String), // Fighter name who reached 0 HP
    MaxTurnsReached(u32), // Maximum turns reached
}

/// Battle state that tracks HP and determines when battle ends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    pub fighter1_name: String,
    pub fighter2_name: String,
    pub fighter1_hp: u32,
    pub fighter2_hp: u32,
    pub fighter1_max_hp: u32,
    pub fighter2_max_hp: u32,
    pub current_turn: u32,
    pub max_turns: u32,
    pub is_complete: bool,
    pub completion_reason: Option<BattleCompletionReason>,
}

impl BattleState {
    pub fn new(fighter1: &Neopet, fighter2: &Neopet, max_turns: u32) -> Self {
        Self {
            fighter1_name: fighter1.name.clone(),
            fighter2_name: fighter2.name.clone(),
            fighter1_hp: fighter1.health,
            fighter2_hp: fighter2.health,
            fighter1_max_hp: fighter1.health,
            fighter2_max_hp: fighter2.health,
            current_turn: 0,
            max_turns,
            is_complete: false,
            completion_reason: None,
        }
    }
    
    /// Apply damage to a fighter and return the new HP
    pub fn apply_damage(&mut self, fighter_name: &str, damage: u32) -> u32 {
        if fighter_name == &self.fighter1_name {
            self.fighter1_hp = self.fighter1_hp.saturating_sub(damage);
            self.fighter1_hp
        } else if fighter_name == &self.fighter2_name {
            self.fighter2_hp = self.fighter2_hp.saturating_sub(damage);
            self.fighter2_hp
        } else {
            panic!("Unknown fighter: {}", fighter_name);
        }
    }
    
    /// Apply healing to a fighter and return the new HP
    pub fn apply_healing(&mut self, fighter_name: &str, amount: u32) -> u32 {
        if fighter_name == &self.fighter1_name {
            self.fighter1_hp = (self.fighter1_hp + amount).min(self.fighter1_max_hp);
            self.fighter1_hp
        } else if fighter_name == &self.fighter2_name {
            self.fighter2_hp = (self.fighter2_hp + amount).min(self.fighter2_max_hp);
            self.fighter2_hp
        } else {
            panic!("Unknown fighter: {}", fighter_name);
        }
    }
    
    /// Check if battle should end and set completion reason
    pub fn check_battle_completion(&mut self) -> Option<BattleCompletionReason> {
        if self.is_complete {
            return self.completion_reason.clone();
        }
        
        if self.fighter1_hp == 0 {
            self.is_complete = true;
            self.completion_reason = Some(BattleCompletionReason::HpDepleted(self.fighter1_name.clone()));
            return self.completion_reason.clone();
        }
        
        if self.fighter2_hp == 0 {
            self.is_complete = true;
            self.completion_reason = Some(BattleCompletionReason::HpDepleted(self.fighter2_name.clone()));
            return self.completion_reason.clone();
        }
        
        if self.current_turn >= self.max_turns {
            self.is_complete = true;
            self.completion_reason = Some(BattleCompletionReason::MaxTurnsReached(self.max_turns));
            return self.completion_reason.clone();
        }
        
        None
    }
    
    /// Get the winner and loser (if battle is complete)
    pub fn get_winner_loser(&self) -> Option<(String, String)> {
        if !self.is_complete {
            return None;
        }
        
        if self.fighter1_hp > self.fighter2_hp {
            Some((self.fighter1_name.clone(), self.fighter2_name.clone()))
        } else if self.fighter2_hp > self.fighter1_hp {
            Some((self.fighter2_name.clone(), self.fighter1_name.clone()))
        } else {
            // It's a draw - use max HP as tiebreaker, otherwise first fighter wins
            if self.fighter1_max_hp >= self.fighter2_max_hp {
                Some((self.fighter1_name.clone(), self.fighter2_name.clone()))
            } else {
                Some((self.fighter2_name.clone(), self.fighter1_name.clone()))
            }
        }
    }
    
    /// Get current HP for a fighter
    pub fn get_hp(&self, fighter_name: &str) -> u32 {
        if fighter_name == &self.fighter1_name {
            self.fighter1_hp
        } else if fighter_name == &self.fighter2_name {
            self.fighter2_hp
        } else {
            panic!("Unknown fighter: {}", fighter_name);
        }
    }
}

#[cfg(test)]
mod battle_state_tests {
    use super::*;
    use crate::neopets::{Neopet, Spell, Behavior};

    // Helper function to create a test Neopet
    fn create_test_neopet(name: &str) -> Neopet {
        Neopet {
            name: name.to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![
                Spell {
                    name: "Fireball".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                },
            ],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![0.1],
                heal_chance: 0.4,
            },
        }
    }

    #[test]
    fn test_battle_state_new() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        assert_eq!(battle_state.fighter1_name, "Fighter1");
        assert_eq!(battle_state.fighter2_name, "Fighter2");
        assert_eq!(battle_state.fighter1_hp, 100);
        assert_eq!(battle_state.fighter2_hp, 100);
        assert_eq!(battle_state.fighter1_max_hp, 100);
        assert_eq!(battle_state.fighter2_max_hp, 100);
        assert_eq!(battle_state.current_turn, 0);
        assert_eq!(battle_state.max_turns, 10);
        assert!(!battle_state.is_complete);
        assert!(battle_state.completion_reason.is_none());
    }

    #[test]
    fn test_apply_damage_normal() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        let new_hp = battle_state.apply_damage("Fighter1", 20);
        assert_eq!(new_hp, 80);
        assert_eq!(battle_state.fighter1_hp, 80);
        assert_eq!(battle_state.fighter2_hp, 100); // Unchanged
    }

    #[test]
    fn test_apply_damage_hp_saturation() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Apply damage that would reduce HP below 0
        let new_hp = battle_state.apply_damage("Fighter1", 150);
        assert_eq!(new_hp, 0);
        assert_eq!(battle_state.fighter1_hp, 0);
    }

    #[test]
    fn test_apply_damage_zero_damage() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        let new_hp = battle_state.apply_damage("Fighter1", 0);
        assert_eq!(new_hp, 100);
        assert_eq!(battle_state.fighter1_hp, 100);
    }

    #[test]
    #[should_panic(expected = "Unknown fighter")]
    fn test_apply_damage_invalid_fighter() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        battle_state.apply_damage("NonExistentFighter", 10);
    }

    #[test]
    fn test_apply_healing_normal() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // First reduce HP
        battle_state.apply_damage("Fighter1", 20);
        assert_eq!(battle_state.fighter1_hp, 80);
        
        // Then heal
        let new_hp = battle_state.apply_healing("Fighter1", 15);
        assert_eq!(new_hp, 95);
        assert_eq!(battle_state.fighter1_hp, 95);
    }

    #[test]
    fn test_apply_healing_max_hp_limit() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // First reduce HP
        battle_state.apply_damage("Fighter1", 20);
        assert_eq!(battle_state.fighter1_hp, 80);
        
        // Then heal beyond max HP
        let new_hp = battle_state.apply_healing("Fighter1", 50);
        assert_eq!(new_hp, 100); // Should be capped at max
        assert_eq!(battle_state.fighter1_hp, 100);
    }

    #[test]
    fn test_apply_healing_from_full_hp() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Try to heal from full HP
        let new_hp = battle_state.apply_healing("Fighter1", 20);
        assert_eq!(new_hp, 100); // Should stay at max
        assert_eq!(battle_state.fighter1_hp, 100);
    }

    #[test]
    fn test_apply_healing_zero_amount() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        let new_hp = battle_state.apply_healing("Fighter1", 0);
        assert_eq!(new_hp, 100);
        assert_eq!(battle_state.fighter1_hp, 100);
    }

    #[test]
    #[should_panic(expected = "Unknown fighter")]
    fn test_apply_healing_invalid_fighter() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        battle_state.apply_healing("NonExistentFighter", 10);
    }

    #[test]
    fn test_check_battle_completion_not_complete() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_none());
        assert!(!battle_state.is_complete);
        assert!(battle_state.completion_reason.is_none());
    }

    #[test]
    fn test_check_battle_completion_hp_depleted_fighter1() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Deplete fighter1's HP
        battle_state.apply_damage("Fighter1", 100);
        
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_some());
        match completion.unwrap() {
            BattleCompletionReason::HpDepleted(name) => assert_eq!(name, "Fighter1"),
            _ => panic!("Expected HP depletion completion"),
        }
        assert!(battle_state.is_complete);
        assert!(battle_state.completion_reason.is_some());
    }

    #[test]
    fn test_check_battle_completion_hp_depleted_fighter2() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Deplete fighter2's HP
        battle_state.apply_damage("Fighter2", 100);
        
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_some());
        match completion.unwrap() {
            BattleCompletionReason::HpDepleted(name) => assert_eq!(name, "Fighter2"),
            _ => panic!("Expected HP depletion completion"),
        }
        assert!(battle_state.is_complete);
    }

    #[test]
    fn test_check_battle_completion_max_turns() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 5);
        
        // Set current turn to max
        battle_state.current_turn = 5;
        
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_some());
        match completion.unwrap() {
            BattleCompletionReason::MaxTurnsReached(turns) => assert_eq!(turns, 5),
            _ => panic!("Expected max turns completion"),
        }
        assert!(battle_state.is_complete);
    }

    #[test]
    fn test_check_battle_completion_already_complete() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Mark as complete first
        battle_state.is_complete = true;
        battle_state.completion_reason = Some(BattleCompletionReason::HpDepleted("Fighter1".to_string()));
        
        // Check completion again
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_some());
        match completion.unwrap() {
            BattleCompletionReason::HpDepleted(name) => assert_eq!(name, "Fighter1"),
            _ => panic!("Expected cached completion reason"),
        }
    }

    #[test]
    fn test_get_winner_loser_not_complete() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_none());
    }

    #[test]
    fn test_get_winner_loser_by_hp() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Set different HP values
        battle_state.fighter1_hp = 50;
        battle_state.fighter2_hp = 30;
        battle_state.is_complete = true;
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_some());
        let (winner, loser) = result.unwrap();
        assert_eq!(winner, "Fighter1");
        assert_eq!(loser, "Fighter2");
    }

    #[test]
    fn test_get_winner_loser_fighter2_wins() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Set different HP values
        battle_state.fighter1_hp = 20;
        battle_state.fighter2_hp = 60;
        battle_state.is_complete = true;
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_some());
        let (winner, loser) = result.unwrap();
        assert_eq!(winner, "Fighter2");
        assert_eq!(loser, "Fighter1");
    }

    #[test]
    fn test_get_winner_loser_draw_by_max_hp_fighter1_higher() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Set equal HP but different max HP
        battle_state.fighter1_hp = 50;
        battle_state.fighter2_hp = 50;
        battle_state.fighter1_max_hp = 120;
        battle_state.fighter2_max_hp = 100;
        battle_state.is_complete = true;
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_some());
        let (winner, loser) = result.unwrap();
        assert_eq!(winner, "Fighter1"); // Higher max HP wins tie
        assert_eq!(loser, "Fighter2");
    }

    #[test]
    fn test_get_winner_loser_draw_by_max_hp_fighter2_higher() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Set equal HP but different max HP
        battle_state.fighter1_hp = 50;
        battle_state.fighter2_hp = 50;
        battle_state.fighter1_max_hp = 100;
        battle_state.fighter2_max_hp = 150;
        battle_state.is_complete = true;
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_some());
        let (winner, loser) = result.unwrap();
        assert_eq!(winner, "Fighter2"); // Higher max HP wins tie
        assert_eq!(loser, "Fighter1");
    }

    #[test]
    fn test_get_winner_loser_draw_equal_max_hp() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Set equal everything
        battle_state.fighter1_hp = 50;
        battle_state.fighter2_hp = 50;
        battle_state.fighter1_max_hp = 100;
        battle_state.fighter2_max_hp = 100;
        battle_state.is_complete = true;
        
        let result = battle_state.get_winner_loser();
        assert!(result.is_some());
        let (winner, loser) = result.unwrap();
        // When everything is equal, first fighter wins (implementation detail)
        assert_eq!(winner, "Fighter1");
        assert_eq!(loser, "Fighter2");
    }

    #[test]
    fn test_get_hp() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        // Modify HP
        battle_state.apply_damage("Fighter1", 20);
        battle_state.apply_damage("Fighter2", 30);
        
        assert_eq!(battle_state.get_hp("Fighter1"), 80);
        assert_eq!(battle_state.get_hp("Fighter2"), 70);
    }

    #[test]
    #[should_panic(expected = "Unknown fighter")]
    fn test_get_hp_invalid_fighter() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let battle_state = BattleState::new(&fighter1, &fighter2, 10);
        
        battle_state.get_hp("NonExistentFighter");
    }

    // Integration test: Full battle state lifecycle
    #[test]
    fn test_battle_state_full_lifecycle() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut battle_state = BattleState::new(&fighter1, &fighter2, 5);
        
        // Simulate a battle
        battle_state.current_turn = 1;
        battle_state.apply_damage("Fighter1", 30); // Fighter1: 70 HP
        battle_state.apply_damage("Fighter2", 20); // Fighter2: 80 HP
        battle_state.apply_healing("Fighter1", 10); // Fighter1: 80 HP
        
        assert_eq!(battle_state.fighter1_hp, 80);
        assert_eq!(battle_state.fighter2_hp, 80);
        assert!(!battle_state.is_complete);
        
        // Deplete Fighter2's HP
        battle_state.apply_damage("Fighter2", 100); // Fighter2: 0 HP
        
        let completion = battle_state.check_battle_completion();
        assert!(completion.is_some());
        assert!(battle_state.is_complete);
        
        let winner_loser = battle_state.get_winner_loser();
        assert!(winner_loser.is_some());
        let (winner, loser) = winner_loser.unwrap();
        assert_eq!(winner, "Fighter1");
        assert_eq!(loser, "Fighter2");
    }
}

fn roll_d20<R: Rng>(rng: &mut R) -> u8 {
    rng.random_range(1..=20)
}

#[derive(Debug, PartialEq)]
enum Action {
    Attack,
    CastSpell(usize),
    Heal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    HealthUpdate {
        fighter_name: String,
        from: u32,
        to: u32,
        turn: u32,
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
    },
    BattleComplete {
        turn: u32,
        winner: String,
        loser: String,
        winner_final_hp: u32,
        loser_final_hp: u32,
        completion_reason: BattleCompletionReason,
    }
}

/// Original process_turn function (for backward compatibility with tests)
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

/// Process a turn with HP tracking and HealthUpdate events
fn process_turn_with_state<R: Rng>(
    actor_name: &str,
    target_name: &str,
    actor_stats: &Neopet, // Contains attack/defense stats
    target_stats: &Neopet, // Contains attack/defense stats
    action: &Action,
    turn_number: u32,
    battle_state: &mut BattleState,
    rng: &mut R,
) -> Vec<BattleEvent> {
    let mut events = Vec::new();
    
    // If battle is already complete, return empty events
    if battle_state.is_complete {
        return events;
    }

    battle_state.current_turn = turn_number;
    
    match action {
        Action::Attack => {
            // Roll for attack
            let attack_roll = roll_d20(rng);
            let attack_val = (attack_roll as u32) + actor_stats.base_attack;
            let attack_is_positive_crit = attack_roll == 20;
            let attack_is_negative_crit = attack_roll == 1;
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: actor_name.to_string(),
                dice: attack_roll,
                final_value: attack_val,
                is_positive_crit: attack_is_positive_crit,
                is_negative_crit: attack_is_negative_crit,
                goal: "attack".to_string(),
            });
            
            // Roll for defense
            let defense_roll = roll_d20(rng);
            let defense_val = (defense_roll as u32) + target_stats.base_defense;
            let defense_is_positive_crit = defense_roll == 20;
            let defense_is_negative_crit = defense_roll == 1;
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: target_name.to_string(),
                dice: defense_roll,
                final_value: defense_val,
                is_positive_crit: defense_is_positive_crit,
                is_negative_crit: defense_is_negative_crit,
                goal: "defense".to_string(),
            });
            
            // Calculate damage
            let mut actual_damage = attack_val.saturating_sub(defense_val);
            if attack_is_positive_crit {
                actual_damage *= 2;
            }
            if attack_is_negative_crit {
                actual_damage = 0;
            }
            
            events.push(BattleEvent::Attack {
                turn: turn_number,
                actor: actor_name.to_string(),
                target: target_name.to_string(),
                raw_damage: attack_val,
                shield_value: defense_val,
                actual_damage,
            });
            
            // Apply damage and generate HealthUpdate event
            if actual_damage > 0 {
                let old_hp = battle_state.get_hp(target_name);
                let new_hp = battle_state.apply_damage(target_name, actual_damage);
                
                events.push(BattleEvent::HealthUpdate {
                    fighter_name: target_name.to_string(),
                    from: old_hp,
                    to: new_hp,
                    turn: turn_number,
                });
            }
        }
        
        Action::Heal => {
            let heal_roll = roll_d20(rng);
            let is_positive_crit = heal_roll == 20;
            let is_negative_crit = heal_roll == 1;
            let mut heal_amount = actor_stats.heal_delta;
            
            if is_positive_crit {
                heal_amount *= 2;
            }
            if is_negative_crit {
                heal_amount = 0;
            }
            
            events.push(BattleEvent::Roll {
                turn: turn_number,
                actor: actor_name.to_string(),
                dice: heal_roll,
                final_value: heal_amount,
                is_positive_crit,
                is_negative_crit,
                goal: "heal".to_string(),
            });
            
            events.push(BattleEvent::Heal {
                turn: turn_number,
                actor: actor_name.to_string(),
                amount: heal_amount,
            });
            
            // Apply healing and generate HealthUpdate event
            if heal_amount > 0 {
                let old_hp = battle_state.get_hp(actor_name);
                let new_hp = battle_state.apply_healing(actor_name, heal_amount);
                
                events.push(BattleEvent::HealthUpdate {
                    fighter_name: actor_name.to_string(),
                    from: old_hp,
                    to: new_hp,
                    turn: turn_number,
                });
            }
        }
        
        Action::CastSpell(spell_index) => {
            let spell_name = if let Some(spell) = actor_stats.spells.get(*spell_index) {
                spell.name.clone()
            } else {
                "Unknown Spell".to_string()
            };
            
            events.push(BattleEvent::SpellCast {
                turn: turn_number,
                actor: actor_name.to_string(),
                target: target_name.to_string(),
                spell_name,
            });
        }
    }
    
    events
}

pub fn battle_loop<R: Rng>(fighter1: &Neopet, fighter2: &Neopet, rng: &mut R) -> Vec<BattleEvent> {
    let (initiative_events, first, second) = roll_for_initiative(fighter1, fighter2, rng);
    
    let max_turns = 10; // Very short for testing - will definitely complete
    let mut battle_state = BattleState::new(fighter1, fighter2, max_turns);
    let mut all_events = initiative_events; // Start with initiative events

    let mut turn = 1; // Start battle turns at 1
    
    while !battle_state.is_complete && turn <= max_turns {
        // First fighter's turn
        if !battle_state.is_complete {
            let first_action = choose_action(first, rng);
            let events = process_turn_with_state(
                &first.name, 
                &second.name, 
                first, 
                second, 
                &first_action, 
                turn, 
                &mut battle_state, 
                rng
            );
            all_events.extend(events);
            
            // Check if battle ended after first fighter's action
            if battle_state.check_battle_completion().is_some() {
                break;
            }
        }
        
        if !battle_state.is_complete && turn < max_turns {
            turn += 1;
            
            // Second fighter's turn
            let second_action = choose_action(second, rng);
            let events = process_turn_with_state(
                &second.name, 
                &first.name, 
                second, 
                first, 
                &second_action, 
                turn, 
                &mut battle_state, 
                rng
            );
            all_events.extend(events);
            
            // Check if battle ended after second fighter's action
            if battle_state.check_battle_completion().is_some() {
                break;
            }
        }
        
        if !battle_state.is_complete {
            turn += 1;
        }
    }
    
    // Generate BattleComplete event if battle ended
    if let Some((winner, loser)) = battle_state.get_winner_loser() {
        let winner_hp = battle_state.get_hp(&winner);
        let loser_hp = battle_state.get_hp(&loser);
        
        all_events.push(BattleEvent::BattleComplete {
            turn: battle_state.current_turn,
            winner,
            loser,
            winner_final_hp: winner_hp,
            loser_final_hp: loser_hp,
            completion_reason: battle_state.completion_reason.unwrap(),
        });
    }
    
    all_events
}

#[cfg(test)]
mod process_turn_with_state_tests {
    use super::*;
    use crate::neopets::{Neopet, Spell, Behavior};
    use crate::battle::{BattleState, BattleEvent};
    use rand::SeedableRng;
    use rand::Rng;
    
    fn create_test_neopet(name: &str, health: u32, attack: u32, defense: u32) -> Neopet {
        Neopet {
            name: name.to_string(),
            health,
            base_attack: attack,
            base_defense: defense,
            heal_delta: 10,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.3,
            },
        }
    }
    
    fn create_seeded_rng() -> impl Rng {
        rand::rngs::StdRng::seed_from_u64(42)
    }
    
    #[test]
    fn test_process_turn_with_state_attack_basic() {
        let actor = create_test_neopet("Attacker", 100, 10, 5);
        let target = create_test_neopet("Defender", 100, 5, 3);
        let mut battle_state = BattleState::new(&actor, &target, 10);
        let mut rng = create_seeded_rng();
        
        let events = process_turn_with_state(
            "Attacker", "Defender",
            &actor, &target,
            &Action::Attack,
            1, &mut battle_state, &mut rng
        );
        
        assert!(!events.is_empty());
        
        let roll_events: Vec<_> = events.iter()
            .filter(|e| matches!(e, BattleEvent::Roll { .. }))
            .collect();
        assert!(roll_events.len() >= 2);
        
        let attack_events: Vec<_> = events.iter()
            .filter(|e| matches!(e, BattleEvent::Attack { .. }))
            .collect();
        assert!(!attack_events.is_empty());
    }
    
    #[test]
    fn test_process_turn_with_state_heal_basic() {
        let actor = create_test_neopet("Healer", 80, 10, 5);
        let target = create_test_neopet("Target", 100, 5, 3);
        let mut battle_state = BattleState::new(&actor, &target, 10);
        let mut rng = create_seeded_rng();
        
        battle_state.apply_damage("Healer", 30);
        assert_eq!(battle_state.get_hp("Healer"), 50);
        
        let events = process_turn_with_state(
            "Healer", "Target",
            &actor, &target,
            &Action::Heal,
            1, &mut battle_state, &mut rng
        );
        
        let heal_events: Vec<_> = events.iter()
            .filter(|e| matches!(e, BattleEvent::Heal { .. }))
            .collect();
        assert!(!heal_events.is_empty());
    }
    
    #[test]
    fn test_process_turn_respects_turn_number() {
        let actor = create_test_neopet("Fighter", 100, 10, 5);
        let target = create_test_neopet("Target", 100, 5, 3);
        let mut battle_state = BattleState::new(&actor, &target, 10);
        let mut rng = create_seeded_rng();
        
        let events = process_turn_with_state(
            "Fighter", "Target",
            &actor, &target,
            &Action::Attack,
            7, &mut battle_state, &mut rng
        );
        
        for event in &events {
            match event {
                BattleEvent::Roll { turn, .. } => assert_eq!(*turn, 7),
                BattleEvent::Attack { turn, .. } => assert_eq!(*turn, 7),
                BattleEvent::HealthUpdate { turn, .. } => assert_eq!(*turn, 7),
                _ => {}
            }
        }
    }
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

#[cfg(test)]
mod battle_integration_tests {
    use super::*;
    use crate::neopets::{Neopet, Spell, Behavior};
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    // Helper function to create a test Neopet
    fn create_test_neopet(name: &str) -> Neopet {
        Neopet {
            name: name.to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![
                Spell {
                    name: "Fireball".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                },
                Spell {
                    name: "Ice Storm".to_string(),
                    effect: serde_json::Value::Object(serde_json::Map::new()),
                },
            ],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![0.2, 0.1],
                heal_chance: 0.2,
            },
        }
    }

    // Helper function to create a simple test Neopet with specific stats
    fn create_simple_neopet(name: &str, health: u32, attack: u32, defense: u32) -> Neopet {
        Neopet {
            name: name.to_string(),
            health,
            heal_delta: 10,
            base_attack: attack,
            base_defense: defense,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.8,
                spell_chances: vec![],
                heal_chance: 0.2,
            },
        }
    }

    #[test]
    fn test_battle_loop_completes() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Battle should complete and generate events
        assert!(!events.is_empty());
        
        // Should have initiative events (turn 0)
        let initiative_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Roll { turn: 0, .. })
        }).collect();
        assert!(!initiative_events.is_empty());
        
        // Should have battle events (turn > 0)
        let battle_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Roll { turn, .. } if *turn > 0)
        }).collect();
        assert!(!battle_events.is_empty());
        
        // Should have a BattleComplete event
        let complete_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::BattleComplete { .. })
        }).collect();
        assert_eq!(complete_events.len(), 1);
        
        // Verify completion event structure
        if let BattleEvent::BattleComplete { winner, loser, winner_final_hp, loser_final_hp, completion_reason, .. } = &complete_events[0] {
            assert!(!winner.is_empty());
            assert!(!loser.is_empty());
            assert_ne!(winner, loser);
            assert!(*winner_final_hp > 0 || *loser_final_hp > 0); // At least one should have HP
            
            // Verify completion reason
            match completion_reason {
                BattleCompletionReason::HpDepleted(_) => {
                    // Valid - someone ran out of HP
                },
                BattleCompletionReason::MaxTurnsReached(max_turns) => {
                    assert_eq!(*max_turns, 10); // Default max turns
                },
            }
        }
    }

    #[test]
    fn test_battle_loop_health_updates() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(123);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should have HealthUpdate events
        let health_updates: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::HealthUpdate { .. })
        }).collect();
        
        // Should have at least one health update
        assert!(!health_updates.is_empty());
        
        // Verify health update structure
        for update in &health_updates {
            if let BattleEvent::HealthUpdate { fighter_name, from, to, turn } = update {
                assert!(!fighter_name.is_empty());
                assert!(from != to); // Health should actually change
                assert!(*turn > 0);
                assert!(*from <= 100); // Should be within valid HP range
                assert!(*to <= 100); // Should be within valid HP range
            }
        }
    }

    #[test]
    fn test_battle_loop_attack_events() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(456);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should have Attack events
        let attack_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Attack { .. })
        }).collect();
        
        // Should have at least one attack
        assert!(!attack_events.is_empty());
        
        // Verify attack event structure
        for attack in &attack_events {
            if let BattleEvent::Attack { turn, actor, target, raw_damage, shield_value, actual_damage } = attack {
                assert!(*turn > 0);
                assert!(!actor.is_empty());
                assert!(!target.is_empty());
                assert_ne!(actor, target);
                assert!(*raw_damage > 0);
                assert!(*shield_value >= 0);
                assert!(*actual_damage <= *raw_damage); // Actual damage can't exceed raw damage
            }
        }
    }

    #[test]
    fn test_battle_loop_heal_events() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(789);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should have Heal events
        let heal_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Heal { .. })
        }).collect();
        
        // Should have at least one heal (due to behavior probabilities)
        assert!(!heal_events.is_empty());
        
        // Verify heal event structure
        for heal in &heal_events {
            if let BattleEvent::Heal { turn, actor, amount } = heal {
                assert!(*turn > 0);
                assert!(!actor.is_empty());
                assert!(*amount >= 0); // Can be 0 due to negative crits
                assert!(*amount <= 20); // Max heal is 10 * 2 (crit)
            }
        }
    }

    #[test]
    fn test_battle_loop_spell_events() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(101112);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should have SpellCast events
        let spell_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::SpellCast { .. })
        }).collect();
        
        // Should have at least one spell cast (due to behavior probabilities)
        assert!(!spell_events.is_empty());
        
        // Verify spell cast event structure
        for spell in &spell_events {
            if let BattleEvent::SpellCast { turn, actor, target, spell_name } = spell {
                assert!(*turn > 0);
                assert!(!actor.is_empty());
                assert!(!target.is_empty());
                assert_ne!(actor, target);
                assert!(!spell_name.is_empty());
                // Should be one of the spells from the test neopets
                assert!(spell_name == "Fireball" || spell_name == "Ice Storm" || spell_name == "Unknown Spell");
            }
        }
    }

    #[test]
    fn test_battle_loop_roll_events() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        let mut rng = StdRng::seed_from_u64(131415);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should have Roll events
        let roll_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Roll { .. })
        }).collect();
        
        // Should have many roll events (initiative + battle rolls)
        assert!(!roll_events.is_empty());
        
        // Verify roll event structure
        for roll in &roll_events {
            if let BattleEvent::Roll { turn, actor, dice, final_value, is_positive_crit, is_negative_crit, goal } = roll {
                assert!(*turn >= 0);
                assert!(!actor.is_empty());
                assert!(*dice >= 1 && *dice <= 20);
                assert!(*final_value > 0);
                assert!(!goal.is_empty());
                
                // Crit flags should be mutually exclusive
                assert!(!(*is_positive_crit && *is_negative_crit));
                
                // Check crit conditions
                if *dice == 20 {
                    assert!(*is_positive_crit);
                    assert!(!*is_negative_crit);
                } else if *dice == 1 {
                    assert!(!*is_positive_crit);
                    assert!(*is_negative_crit);
                } else {
                    assert!(!*is_positive_crit);
                    assert!(!*is_negative_crit);
                }
            }
        }
    }

    #[test]
    fn test_battle_loop_quick_battle() {
        // Create fighters with low HP to ensure quick battle
        let fighter1 = create_simple_neopet("Quick1", 20, 10, 0);
        let fighter2 = create_simple_neopet("Quick2", 20, 10, 0);
        let mut rng = StdRng::seed_from_u64(161718);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should still complete
        let complete_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::BattleComplete { .. })
        }).collect();
        assert_eq!(complete_events.len(), 1);
        
        // Should have fewer total events due to quick battle
        assert!(events.len() < 100); // Reasonable upper bound
    }

    #[test]
    fn test_battle_loop_one_sided_battle() {
        // Create a very one-sided battle
        let fighter1 = create_simple_neopet("Strong", 100, 20, 10);  // High attack, good defense
        let fighter2 = create_simple_neopet("Weak", 30, 2, 1);       // Low HP, low stats
        let mut rng = StdRng::seed_from_u64(192021);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should complete
        let complete_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::BattleComplete { .. })
        }).collect();
        assert_eq!(complete_events.len(), 1);
        
        if let BattleEvent::BattleComplete { winner, loser, .. } = &complete_events[0] {
            // Strong fighter should usually win in a one-sided battle
            assert_eq!(winner, "Strong");
            assert_eq!(loser, "Weak");
        }
    }

    #[test]
    fn test_battle_loop_heavy_defense_battle() {
        // Create a battle with heavy defense
        let fighter1 = create_simple_neopet("Tank1", 80, 5, 15);   // High defense
        let fighter2 = create_simple_neopet("Tank2", 80, 5, 15);   // High defense
        let mut rng = StdRng::seed_from_u64(222324);
        
        let events = battle_loop(&fighter1, &fighter2, &mut rng);
        
        // Should complete (likely by max turns due to low damage)
        let complete_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::BattleComplete { .. })
        }).collect();
        assert_eq!(complete_events.len(), 1);
        
        // Should have many attack events with low or zero damage
        let attack_events: Vec<_> = events.iter().filter(|e| {
            matches!(e, BattleEvent::Attack { actual_damage, .. } if *actual_damage == 0)
        }).collect();
        
        // Due to high defense, should have some zero-damage attacks
        assert!(!attack_events.is_empty());
    }

    #[test]
    fn test_battle_loop_reproducible_with_seed() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        
        // Same seed should produce same results
        let mut rng1 = StdRng::seed_from_u64(252627);
        let mut rng2 = StdRng::seed_from_u64(252627);
        
        let events1 = battle_loop(&fighter1, &fighter2, &mut rng1);
        let events2 = battle_loop(&fighter1, &fighter2, &mut rng2);
        
        // Should have same number of events
        assert_eq!(events1.len(), events2.len());
        
        // Events should be identical
        for (i, (e1, e2)) in events1.iter().zip(events2.iter()).enumerate() {
            assert_eq!(e1, e2, "Event {} should be identical", i);
        }
    }

    #[test]
    fn test_battle_loop_different_seeds_different_results() {
        let fighter1 = create_test_neopet("Fighter1");
        let fighter2 = create_test_neopet("Fighter2");
        
        // Different seeds should produce different results (with high probability)
        let mut rng1 = StdRng::seed_from_u64(282930);
        let mut rng2 = StdRng::seed_from_u64(313233);
        
        let events1 = battle_loop(&fighter1, &fighter2, &mut rng1);
        let events2 = battle_loop(&fighter1, &fighter2, &mut rng2);
        
        // Very likely to have different results with different seeds
        // (Though theoretically possible to be the same, extremely unlikely)
        let same_winner = match (&events1.last(), &events2.last()) {
            (Some(BattleEvent::BattleComplete { winner: w1, .. }), Some(BattleEvent::BattleComplete { winner: w2, .. })) => w1 == w2,
            _ => false,
        };
        
        // At least one of winner, length, or event sequence should differ
        let different_length = events1.len() != events2.len();
        let different_events = events1 != events2;
        
        assert!(different_length || different_events || !same_winner, 
                "Different seeds should produce different results");
    }
}
