use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Spell {
    name: String,
    effect: serde_json::Value,
}

#[derive(Deserialize)]
struct BehaviorDef {
    attack_chance: f64,
    spell_chances: Vec<f64>,
    heal_chance: f64,
}

#[derive(Debug, Serialize)]
#[serde(try_from = "BehaviorDef")]
struct Behavior {
    attack_chance: f64,
    spell_chances: Vec<f64>,
    heal_chance: f64,
}

impl TryFrom<BehaviorDef> for Behavior {
    type Error = String;
    
    fn try_from(def: BehaviorDef) -> Result<Self, Self::Error> {
        let total = def.attack_chance + def.heal_chance + def.spell_chances.iter().sum::<f64>();
        
        if (total - 1.0).abs() > f64::EPSILON {
            Err(format!("Behavior probabilities sum to {} but must equal 1.0 (attack: {}, heal: {}, spells: {:?})", total, def.attack_chance, def.heal_chance, def.spell_chances))
        } else {
            Ok(Behavior {
                attack_chance: def.attack_chance,
                spell_chances: def.spell_chances,
                heal_chance: def.heal_chance,
            })
        }
    }
}

#[derive(Deserialize)]
struct NeopetDef {
    name: String,
    health: u32,
    heal_delta: u32,
    base_attack: u32,
    base_defense: u32,
    spells: Vec<Spell>,
    behavior: BehaviorDef,
}

#[derive(Debug, Serialize)]
#[serde(try_from = "NeopetDef")]
struct Neopet {
    name: String,
    health: u32,
    heal_delta: u32,
    base_attack: u32,
    base_defense: u32,
    spells: Vec<Spell>,
    behavior: Behavior,
}

impl TryFrom<NeopetDef> for Neopet {
    type Error = String;
    
    fn try_from(def: NeopetDef) -> Result<Self, Self::Error> {
        if def.behavior.spell_chances.len() != def.spells.len() {
            return Err(format!("Neopet {}: {} spell chances but {} spells", def.name, def.behavior.spell_chances.len(), def.spells.len()));
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

fn load_neopets(path: &str) -> Vec<Neopet> {
    let file = File::open(path).expect("Failed to open file");
    let neopets_def: Vec<NeopetDef> = serde_json::from_reader(file).expect("Failed to deserialize");
    neopets_def.into_iter()
        .map(|def| Neopet::try_from(def).expect("Failed to validate neopet"))
        .collect()
}

fn main() {
    let neopets_set = load_neopets("assets/neopets.json");
    for n in neopets_set {
        println!("{:#?}", n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_roundtrip_with_file_comparison() {
        let original_json = fs::read_to_string("assets/neopets.json")
            .expect("Failed to read original file");
        
        let neopets = load_neopets("assets/neopets.json");
        
        let serialized_json = serde_json::to_string_pretty(&neopets)
            .expect("Failed to serialize");
        
        let mut temp_file = NamedTempFile::new()
            .expect("Failed to create temp file");
        write!(temp_file, "{}", serialized_json)
            .expect("Failed to write to temp file");
        
        let roundtrip_json = fs::read_to_string(temp_file.path())
            .expect("Failed to read temp file");
        
        let original_value: serde_json::Value = serde_json::from_str(&original_json)
            .expect("Failed to parse original JSON");
        let roundtrip_value: serde_json::Value = serde_json::from_str(&roundtrip_json)
            .expect("Failed to parse roundtrip JSON");
        
        assert_eq!(original_value, roundtrip_value);
    }
}
