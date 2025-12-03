use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Spell {
    pub name: String,
    pub effect: serde_json::Value,
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize)]
pub struct BehaviorDef {
    pub attack_chance: f64,
    pub spell_chances: Vec<f64>,
    pub heal_chance: f64,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(try_from = "BehaviorDef")]
pub struct Behavior {
    pub attack_chance: f64,
    pub spell_chances: Vec<f64>,
    pub heal_chance: f64,
}

impl TryFrom<BehaviorDef> for Behavior {
    type Error = String;

    fn try_from(def: BehaviorDef) -> Result<Self, Self::Error> {
        let total = def.attack_chance + def.heal_chance + def.spell_chances.iter().sum::<f64>();

        if (total - 1.0).abs() > f64::EPSILON {
            Err(format!(
                "Behavior probabilities sum to {} but must equal 1.0 (attack: {}, heal: {}, spells: {:?})",
                total, def.attack_chance, def.heal_chance, def.spell_chances
            ))
        } else {
            Ok(Behavior {
                attack_chance: def.attack_chance,
                spell_chances: def.spell_chances,
                heal_chance: def.heal_chance,
            })
        }
    }
}

impl fmt::Display for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "⚔️ {:.0}% | 🪄 {:?} | 💚 {:.0}%",
            self.attack_chance * 100.0,
            self.spell_chances
                .iter()
                .map(|c| format!("{:.0}%", c * 100.0))
                .collect::<Vec<_>>(),
            self.heal_chance * 100.0
        )
    }
}

#[derive(Deserialize)]
pub struct NeopetDef {
    pub name: String,
    pub health: u32,
    pub heal_delta: u32,
    pub base_attack: u32,
    pub base_defense: u32,
    pub spells: Vec<Spell>,
    pub behavior: BehaviorDef,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(try_from = "NeopetDef")]
pub struct Neopet {
    pub name: String,
    pub health: u32,
    pub heal_delta: u32,
    pub base_attack: u32,
    pub base_defense: u32,
    pub spells: Vec<Spell>,
    pub behavior: Behavior,
}

impl TryFrom<NeopetDef> for Neopet {
    type Error = String;

    fn try_from(def: NeopetDef) -> Result<Self, Self::Error> {
        if def.behavior.spell_chances.len() != def.spells.len() {
            return Err(format!(
                "Neopet {}: {} spell chances but {} spells",
                def.name,
                def.behavior.spell_chances.len(),
                def.spells.len()
            ));
        }

        let behavior = Behavior::try_from(def.behavior)?;

        Ok(Neopet {
            name: def.name,
            health: def.health,
            heal_delta: def.heal_delta,
            base_attack: def.base_attack,
            base_defense: def.base_defense,
            spells: def.spells,
            behavior,
        })
    }
}

impl fmt::Display for Neopet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let spell_list = self
            .spells
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "{}\nHP: {} | ATK: {} | DEF: {} | Heal: +{}\nSpells: {}\nBehavior: {}",
            self.name,
            self.health,
            self.base_attack,
            self.base_defense,
            self.heal_delta,
            spell_list,
            self.behavior
        )
    }
}

pub fn load_neopets(path: &str) -> Vec<Neopet> {
    let file = File::open(path).expect("Failed to open file");
    let neopets_def: Vec<NeopetDef> = serde_json::from_reader(file).expect("Failed to deserialize");
    neopets_def
        .into_iter()
        .map(|def| Neopet::try_from(def).expect("Failed to validate neopet"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_roundtrip_with_file_comparison() {
        let original_json =
            fs::read_to_string("assets/neopets.json").expect("Failed to read original file");

        let neopets = load_neopets("assets/neopets.json");

        let serialized_json = serde_json::to_string_pretty(&neopets).expect("Failed to serialize");

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        write!(temp_file, "{}", serialized_json).expect("Failed to write to temp file");

        let roundtrip_json =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");

        let original_value: serde_json::Value =
            serde_json::from_str(&original_json).expect("Failed to parse original JSON");
        let roundtrip_value: serde_json::Value =
            serde_json::from_str(&roundtrip_json).expect("Failed to parse roundtrip JSON");

        assert_eq!(original_value, roundtrip_value);
    }

    #[test]
    fn test_behavior_valid_exactly_one() {
        let def = BehaviorDef {
            attack_chance: 0.5,
            spell_chances: vec![0.1, 0.15],
            heal_chance: 0.25,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavior_valid_close_to_one() {
        let def = BehaviorDef {
            attack_chance: 0.5 + 1e-17,
            spell_chances: vec![0.1, 0.15],
            heal_chance: 0.25,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavior_valid_zero_spells() {
        let def = BehaviorDef {
            attack_chance: 0.5,
            spell_chances: vec![],
            heal_chance: 0.5,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavior_invalid_sum_too_low() {
        let def = BehaviorDef {
            attack_chance: 0.5,
            spell_chances: vec![0.1, 0.15],
            heal_chance: 0.1,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_err());
    }

    #[test]
    fn test_behavior_invalid_sum_too_high() {
        let def = BehaviorDef {
            attack_chance: 0.5,
            spell_chances: vec![0.1, 0.15],
            heal_chance: 0.4,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_err());
    }

    #[test]
    fn test_behavior_invalid_sum_way_off() {
        let def = BehaviorDef {
            attack_chance: 1.5,
            spell_chances: vec![0.5, 0.5],
            heal_chance: 0.5,
        };
        let result = Behavior::try_from(def);
        assert!(result.is_err());
    }

    #[test]
    fn test_behavior_error_message_content() {
        let def = BehaviorDef {
            attack_chance: 0.5,
            spell_chances: vec![0.1, 0.15],
            heal_chance: 0.1,
        };
        let result = Behavior::try_from(def);
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("sum to"));
        assert!(error_msg.contains("attack"));
        assert!(error_msg.contains("heal"));
        assert!(error_msg.contains("spells"));
    }

    #[test]
    fn test_neopet_valid_two_spells_two_chances() {
        let def = NeopetDef {
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
            ],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![0.1, 0.15],
                heal_chance: 0.25,
            },
        };
        let result = Neopet::try_from(def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neopet_valid_zero_spells_zero_chances() {
        let def = NeopetDef {
            name: "TestPet".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.5,
            },
        };
        let result = Neopet::try_from(def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neopet_invalid_more_spells_than_chances() {
        let def = NeopetDef {
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
            ],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![0.1],
                heal_chance: 0.25,
            },
        };
        let result = Neopet::try_from(def);
        assert!(result.is_err());
    }

    #[test]
    fn test_neopet_invalid_more_chances_than_spells() {
        let def = NeopetDef {
            name: "TestPet".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![Spell {
                name: "Spell1".to_string(),
                effect: serde_json::Value::Object(serde_json::Map::new()),
            }],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![0.1, 0.15],
                heal_chance: 0.25,
            },
        };
        let result = Neopet::try_from(def);
        assert!(result.is_err());
    }

    #[test]
    fn test_neopet_invalid_spell_count_mismatch_error_message() {
        let def = NeopetDef {
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
            ],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![0.1],
                heal_chance: 0.25,
            },
        };
        let result = Neopet::try_from(def);
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("TestPet"));
        assert!(error_msg.contains("spell chances"));
        assert!(error_msg.contains("spells"));
    }

    #[test]
    fn test_neopet_invalid_behavior_sum_propagates() {
        let def = NeopetDef {
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
            ],
            behavior: BehaviorDef {
                attack_chance: 0.5,
                spell_chances: vec![0.1, 0.15],
                heal_chance: 0.1,
            },
        };
        let result = Neopet::try_from(def);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("sum"));
    }

    #[test]
    #[should_panic(expected = "Failed to validate neopet")]
    fn test_load_neopets_with_invalid_behavior_sum() {
        let json = r#"
        [
            {
                "name": "InvalidPet",
                "health": 100,
                "heal_delta": 10,
                "base_attack": 5,
                "base_defense": 3,
                "spells": [
                    {"name": "Spell1", "effect": {}},
                    {"name": "Spell2", "effect": {}}
                ],
                "behavior": {
                    "attack_chance": 0.5,
                    "spell_chances": [0.1, 0.15],
                    "heal_chance": 0.1
                }
            }
        ]
        "#;
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(temp_file.path(), json).expect("Failed to write to temp file");
        let _neopets = load_neopets(temp_file.path().to_str().unwrap());
    }

    #[test]
    #[should_panic(expected = "spell chances but")]
    fn test_load_neopets_with_spell_count_mismatch() {
        let json = r#"
        [
            {
                "name": "InvalidPet",
                "health": 100,
                "heal_delta": 10,
                "base_attack": 5,
                "base_defense": 3,
                "spells": [
                    {"name": "Spell1", "effect": {}},
                    {"name": "Spell2", "effect": {}}
                ],
                "behavior": {
                    "attack_chance": 0.5,
                    "spell_chances": [0.1],
                    "heal_chance": 0.4
                }
            }
        ]
        "#;
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(temp_file.path(), json).expect("Failed to write to temp file");
        let _neopets = load_neopets(temp_file.path().to_str().unwrap());
    }

    #[test]
    fn test_load_neopets_all_validation_passes() {
        let neopets = load_neopets("assets/neopets.json");
        assert_eq!(neopets.len(), 3);
        for neopet in neopets {
            assert_eq!(neopet.behavior.spell_chances.len(), neopet.spells.len());
            let total = neopet.behavior.attack_chance
                + neopet.behavior.heal_chance
                + neopet.behavior.spell_chances.iter().sum::<f64>();
            assert!((total - 1.0).abs() <= f64::EPSILON);
        }
    }
}
