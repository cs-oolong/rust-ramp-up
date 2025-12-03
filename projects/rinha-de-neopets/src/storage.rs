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

    pub fn generate_battle_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("battle_{}", timestamp)
    }
}