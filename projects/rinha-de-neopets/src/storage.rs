// src/storage.rs
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::neopets::{Neopet, load_neopets};
use crate::battle::BattleEvent;

/// Serializable battle record
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BattleRecord {
    pub id: String,                    // Unique ID (timestamp)
    pub fighter1_name: String,
    pub fighter2_name: String,
    pub created_at: String,            // ISO 8601 timestamp
    pub events: Vec<BattleEvent>,      // Full battle history
    pub winner: Option<String>,        // None if battle hasn't been run
    pub is_completed: bool,
}

pub struct Storage {
    neopets_path: String,
    complete_battles_path: String,
    pending_battles_path: String,
    neopets: Vec<Neopet>,
    complete_battles: Vec<BattleRecord>,
    pending_battles: Vec<BattleRecord>,
}

impl Storage {
    pub fn new(neopets_path: &str, complete_battles_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let neopets = if Path::new(neopets_path).exists() {
            load_neopets(neopets_path)
        } else {
            Vec::new()
        };
        
        let complete_battles = if Path::new(complete_battles_path).exists() {
            let file = File::open(complete_battles_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        } else {
            Vec::new()
        };

        let pending_battles_path = "assets/pending_battles.json";
        let pending_battles = if Path::new(pending_battles_path).exists() {
            let file = File::open(pending_battles_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        } else {
            Vec::new()
        };

        Ok(Self {
            neopets_path: neopets_path.to_string(),
            complete_battles_path: complete_battles_path.to_string(),
            pending_battles_path: pending_battles_path.to_string(),
            neopets,
            complete_battles,
            pending_battles,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Save neopets
        let neopets_file = File::create(&self.neopets_path)?;
        let writer = BufWriter::new(neopets_file);
        serde_json::to_writer_pretty(writer, &self.neopets)?;
        
        // Save complete battles
        let complete_battles_file = File::create(&self.complete_battles_path)?;
        let writer = BufWriter::new(complete_battles_file);
        serde_json::to_writer_pretty(writer, &self.complete_battles)?;
        
        // Save pending battles
        let pending_battles_file = File::create(&self.pending_battles_path)?;
        let writer = BufWriter::new(pending_battles_file);
        serde_json::to_writer_pretty(writer, &self.pending_battles)?;
        
        Ok(())
    }

    // Fighter operations
    pub fn add_neopet(&mut self, neopet: Neopet) -> Result<(), String> {
        // Check for duplicate name
        if self.neopets.iter().any(|n| n.name == neopet.name) {
            return Err(format!("A fighter named '{}' already exists", neopet.name));
        }
        self.neopets.push(neopet);
        Ok(())
    }

    pub fn list_fighters(&self) -> Vec<String> {
        self.neopets.iter().map(|n| n.name.clone()).collect()
    }

    pub fn get_fighter(&self, name: &str) -> Option<&Neopet> {
        self.neopets.iter().find(|n| n.name == name)
    }

    // Complete battle operations
    pub fn add_complete_battle(&mut self, battle: BattleRecord) {
        self.complete_battles.push(battle);
    }

    pub fn list_complete_battles(&self) -> Vec<(String, String, String)> {
        // Returns (id, fighter1 vs fighter2, status)
        self.complete_battles.iter().map(|b| {
            let status = if b.is_completed { "Completed" } else { "Pending" };
            (b.id.clone(), format!("{} vs {}", b.fighter1_name, b.fighter2_name), status.to_string())
        }).collect()
    }

    pub fn get_complete_battle(&mut self, id: &str) -> Option<&mut BattleRecord> {
        self.complete_battles.iter_mut().find(|b| b.id == id)
    }

    pub fn clear_complete_battles(&mut self) {
        self.complete_battles.clear();
    }

    // Pending battle operations
    pub fn add_pending_battle(&mut self, battle: BattleRecord) {
        self.pending_battles.push(battle);
    }

    pub fn list_pending_battles(&self) -> Vec<(String, String, String)> {
        // Returns (id, fighter1 vs fighter2, created_at)
        self.pending_battles.iter().map(|b| {
            (b.id.clone(), format!("{} vs {}", b.fighter1_name, b.fighter2_name), b.created_at.clone())
        }).collect()
    }

    pub fn clear_pending_battles(&mut self) {
        self.pending_battles.clear();
    }

    // Battle execution operations
    pub fn find_pending_battle(&self, id: &str) -> Option<BattleRecord> {
        self.pending_battles.iter().find(|b| b.id == id).cloned()
    }

    pub fn remove_pending_battle(&mut self, id: &str) -> Option<BattleRecord> {
        if let Some(pos) = self.pending_battles.iter().position(|b| b.id == id) {
            Some(self.pending_battles.remove(pos))
        } else {
            None
        }
    }

    pub fn move_battle_to_complete(&mut self, mut battle: BattleRecord, events: Vec<BattleEvent>, winner: Option<String>) -> BattleRecord {
        // Update the battle record with execution results
        battle.events = events;
        battle.winner = winner;
        battle.is_completed = true;
        
        // Add to complete battles
        self.complete_battles.push(battle.clone());
        battle
    }

    pub fn generate_battle_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("battle_{}", timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neopets::{Neopet, Spell, Behavior};
    use tempfile::tempdir;
    use std::fs;

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

    // Helper function to create a test BattleRecord
    fn create_test_battle_record(id: &str, fighter1: &str, fighter2: &str) -> BattleRecord {
        BattleRecord {
            id: id.to_string(),
            fighter1_name: fighter1.to_string(),
            fighter2_name: fighter2.to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            events: vec![],
            winner: None,
            is_completed: false,
        }
    }

    // Helper function to create a clean test storage
    fn create_test_storage() -> Storage {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("test_neopets.json");
        let battles_path = temp_dir.path().join("test_battles.json");
        let pending_path = temp_dir.path().join("test_pending.json");
        
        // Create empty JSON files
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        fs::write(&pending_path, "[]").unwrap();
        
        // Create a storage with custom paths by modifying the implementation
        let neopets = Vec::new();
        let complete_battles = Vec::new();
        let pending_battles = Vec::new();
        
        Storage {
            neopets_path: neopets_path.to_str().unwrap().to_string(),
            complete_battles_path: battles_path.to_str().unwrap().to_string(),
            pending_battles_path: pending_path.to_str().unwrap().to_string(),
            neopets,
            complete_battles,
            pending_battles,
        }
    }

    #[test]
    fn test_storage_new_success() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("test_neopets.json");
        let battles_path = temp_dir.path().join("test_battles.json");
        
        // Write valid test data
        fs::write(&neopets_path, r#"[{"name":"TestPet","health":100,"heal_delta":10,"base_attack":5,"base_defense":3,"spells":[{"name":"Fireball","effect":{}}],"behavior":{"attack_chance":0.5,"spell_chances":[0.1],"heal_chance":0.4}}]"#).unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap());
        assert!(storage.is_ok());
        let storage = storage.unwrap();
        assert_eq!(storage.list_fighters().len(), 1);
        assert_eq!(storage.list_fighters()[0], "TestPet");
    }

    #[test]
    fn test_storage_new_empty_files() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("empty_neopets.json");
        let battles_path = temp_dir.path().join("empty_battles.json");
        let pending_path = temp_dir.path().join("empty_pending.json");
        
        // Create empty files
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        fs::write(&pending_path, "[]").unwrap();
        
        let storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap());
        assert!(storage.is_ok());
        let storage = storage.unwrap();
        assert_eq!(storage.list_fighters().len(), 0);
        assert_eq!(storage.list_complete_battles().len(), 0);
    }

    #[test]
    fn test_storage_new_missing_files() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("missing_neopets.json");
        let battles_path = temp_dir.path().join("missing_battles.json");
        
        // Files don't exist - should create empty storage
        let storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap());
        assert!(storage.is_ok());
        let storage = storage.unwrap();
        assert_eq!(storage.list_fighters().len(), 0);
    }

    #[test]
    fn test_storage_save_persistence() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("save_test_neopets.json");
        let battles_path = temp_dir.path().join("save_test_battles.json");
        
        // Create storage and add data
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let neopet = create_test_neopet("SaveTestPet");
        storage.add_neopet(neopet).unwrap();
        
        // Save
        assert!(storage.save().is_ok());
        
        // Load new storage instance and verify data persisted
        let new_storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        assert!(new_storage.get_fighter("SaveTestPet").is_some());
    }

    #[test]
    fn test_add_neopet_success() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("add_neopet_test.json");
        let battles_path = temp_dir.path().join("add_battle_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let neopet = create_test_neopet("NewPet");
        
        let result = storage.add_neopet(neopet);
        assert!(result.is_ok());
        assert_eq!(storage.list_fighters().len(), 1);
        assert_eq!(storage.list_fighters()[0], "NewPet");
    }

    #[test]
    fn test_add_neopet_duplicate_name() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("duplicate_test.json");
        let battles_path = temp_dir.path().join("duplicate_battle_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let neopet = create_test_neopet("DuplicatePet");
        
        // First addition should succeed
        assert!(storage.add_neopet(neopet.clone()).is_ok());
        
        // Second addition should fail
        let result = storage.add_neopet(neopet);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_list_fighters() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("list_fighters_test.json");
        let battles_path = temp_dir.path().join("list_battles_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        
        // Add multiple fighters
        storage.add_neopet(create_test_neopet("Fighter1")).unwrap();
        storage.add_neopet(create_test_neopet("Fighter2")).unwrap();
        storage.add_neopet(create_test_neopet("Fighter3")).unwrap();
        
        let fighters = storage.list_fighters();
        assert_eq!(fighters.len(), 3);
        assert!(fighters.contains(&"Fighter1".to_string()));
        assert!(fighters.contains(&"Fighter2".to_string()));
        assert!(fighters.contains(&"Fighter3".to_string()));
    }

    #[test]
    fn test_get_fighter() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("get_fighter_test.json");
        let battles_path = temp_dir.path().join("get_battle_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let neopet = create_test_neopet("TestFighter");
        storage.add_neopet(neopet.clone()).unwrap();
        
        // Should find existing fighter
        let found = storage.get_fighter("TestFighter");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "TestFighter");
        
        // Should not find non-existing fighter
        let not_found = storage.get_fighter("NonExistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_add_complete_battle() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_test.json");
        let battles_path = temp_dir.path().join("battles_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let battle = create_test_battle_record("battle_123", "Fighter1", "Fighter2");
        
        storage.add_complete_battle(battle);
        assert_eq!(storage.list_complete_battles().len(), 1);
    }

    #[test]
    fn test_list_complete_battles() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_list_test.json");
        let battles_path = temp_dir.path().join("battles_list_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        
        // Add multiple battles
        storage.add_complete_battle(create_test_battle_record("battle_1", "Fighter1", "Fighter2"));
        storage.add_complete_battle(create_test_battle_record("battle_2", "Fighter3", "Fighter4"));
        
        let battles = storage.list_complete_battles();
        assert_eq!(battles.len(), 2);
        
        // Check format: (id, "Fighter1 vs Fighter2", "Completed")
        assert_eq!(battles[0].0, "battle_1");
        assert_eq!(battles[0].1, "Fighter1 vs Fighter2");
        assert_eq!(battles[0].2, "Pending"); // is_completed is false by default
    }

    #[test]
    fn test_get_complete_battle() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_get_test.json");
        let battles_path = temp_dir.path().join("battles_get_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let battle = create_test_battle_record("battle_get_123", "Fighter1", "Fighter2");
        storage.add_complete_battle(battle);
        
        // Should find existing battle
        let found = storage.get_complete_battle("battle_get_123");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "battle_get_123");
        
        // Should not find non-existing battle
        let not_found = storage.get_complete_battle("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_clear_complete_battles() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_clear_test.json");
        let battles_path = temp_dir.path().join("battles_clear_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        
        // Add battles
        storage.add_complete_battle(create_test_battle_record("battle_1", "Fighter1", "Fighter2"));
        storage.add_complete_battle(create_test_battle_record("battle_2", "Fighter3", "Fighter4"));
        assert_eq!(storage.list_complete_battles().len(), 2);
        
        // Clear battles
        storage.clear_complete_battles();
        assert_eq!(storage.list_complete_battles().len(), 0);
    }

    #[test]
    fn test_add_pending_battle() {
        let mut storage = create_test_storage();
        let battle = create_test_battle_record("pending_123", "Fighter1", "Fighter2");
        
        storage.add_pending_battle(battle);
        let pending = storage.list_pending_battles();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, "pending_123");
    }

    #[test]
    fn test_find_pending_battle() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_find_test.json");
        let battles_path = temp_dir.path().join("battles_find_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let battle = create_test_battle_record("find_123", "Fighter1", "Fighter2");
        storage.add_pending_battle(battle);
        
        // Should find existing battle
        let found = storage.find_pending_battle("find_123");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "find_123");
        
        // Should not find non-existing battle
        let not_found = storage.find_pending_battle("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_remove_pending_battle() {
        let mut storage = create_test_storage();
        let battle = create_test_battle_record("remove_123", "Fighter1", "Fighter2");
        storage.add_pending_battle(battle);
        
        assert_eq!(storage.list_pending_battles().len(), 1);
        
        // Remove existing battle
        let removed = storage.remove_pending_battle("remove_123");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, "remove_123");
        assert_eq!(storage.list_pending_battles().len(), 0);
        
        // Remove non-existing battle
        let not_removed = storage.remove_pending_battle("nonexistent");
        assert!(not_removed.is_none());
    }

    #[test]
    fn test_move_battle_to_complete() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_move_test.json");
        let battles_path = temp_dir.path().join("battles_move_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let mut storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        let battle = create_test_battle_record("move_123", "Fighter1", "Fighter2");
        
        // Create some test events
        let events = vec![
            BattleEvent::Roll {
                turn: 1,
                actor: "Fighter1".to_string(),
                dice: 15,
                final_value: 20,
                is_positive_crit: false,
                is_negative_crit: false,
                goal: "attack".to_string(),
            },
        ];
        
        // Move to complete
        let completed = storage.move_battle_to_complete(battle, events.clone(), Some("Fighter1".to_string()));
        
        assert_eq!(completed.events.len(), 1);
        assert_eq!(completed.winner, Some("Fighter1".to_string()));
        assert!(completed.is_completed);
        
        // Should be in complete battles
        assert_eq!(storage.list_complete_battles().len(), 1);
    }

    #[test]
    fn test_generate_battle_id() {
        let temp_dir = tempdir().unwrap();
        let neopets_path = temp_dir.path().join("neopets_id_test.json");
        let battles_path = temp_dir.path().join("battles_id_test.json");
        
        fs::write(&neopets_path, "[]").unwrap();
        fs::write(&battles_path, "[]").unwrap();
        
        let storage = Storage::new(neopets_path.to_str().unwrap(), battles_path.to_str().unwrap()).unwrap();
        
        // Generate multiple IDs
        let id1 = storage.generate_battle_id();
        let id2 = storage.generate_battle_id();
        
        // Should be different
        assert_ne!(id1, id2);
        
        // Should start with battle_
        assert!(id1.starts_with("battle_"));
        assert!(id2.starts_with("battle_"));
    }

    #[test]
    fn test_clear_pending_battles() {
        let mut storage = create_test_storage();
        
        // Add pending battles
        storage.add_pending_battle(create_test_battle_record("pending_1", "Fighter1", "Fighter2"));
        storage.add_pending_battle(create_test_battle_record("pending_2", "Fighter3", "Fighter4"));
        assert_eq!(storage.list_pending_battles().len(), 2);
        
        // Clear pending battles
        storage.clear_pending_battles();
        assert_eq!(storage.list_pending_battles().len(), 0);
    }
}