use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Neopet {
    name: String,
    health: u32,
    heal_delta: u32,
    base_attack: u32,
    base_defense: u32,
    spells: Vec<Spell>,
    behavior: Behavior,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spell {
    name: String,
    effect: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Behavior {
    attack_chance: f64,
    spell_chances: Vec<f64>,
    heal_chance: f64,
}

fn load_neopets(path: &str) -> Vec<Neopet> {
    let file = File::open(path).expect("Failed to open file");
    let neopets: Vec<Neopet> = serde_json::from_reader(file).expect("Failed to deserialize");
    neopets
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
    use tempfile::NamedTempFile;
    use std::{fs, io::Write};

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
