# Colosseum CLI Low-Level Design

## Implementation Plan for `colosseum.rs`

### 1. **Dependencies to Add**
Add to `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
dialoguer = "0.11"
chrono = { version = "0.4", features = ["serde"] }
# ... existing deps
```

### 2. **Storage Layer (`src/storage.rs`)**

This is the heart of data persistence. Create a new module:

```rust
// src/storage.rs
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
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
    battles_path: String,
    neopets: Vec<Neopet>,
    battles: Vec<BattleRecord>,
}

impl Storage {
    pub fn new(neopets_path: &str, battles_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let neopets = if Path::new(neopets_path).exists() {
            load_neopets(neopets_path)
        } else {
            Vec::new()
        };
        
        let battles = if Path::new(battles_path).exists() {
            let file = File::open(battles_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        } else {
            Vec::new()
        };

        Ok(Self {
            neopets_path: neopets_path.to_string(),
            battles_path: battles_path.to_string(),
            neopets,
            battles,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Save neopets
        let neopets_file = File::create(&self.neopets_path)?;
        let writer = BufWriter::new(neopets_file);
        serde_json::to_writer_pretty(writer, &self.neopets)?;
        
        // Save battles
        let battles_file = File::create(&self.battles_path)?;
        let writer = BufWriter::new(battles_file);
        serde_json::to_writer_pretty(writer, &self.battles)?;
        
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

    // Battle operations
    pub fn add_battle(&mut self, battle: BattleRecord) {
        self.battles.push(battle);
    }

    pub fn list_battles(&self) -> Vec<(String, String, String)> {
        // Returns (id, fighter1 vs fighter2, status)
        self.battles.iter().map(|b| {
            let status = if b.is_completed { "Completed" } else { "Pending" };
            (b.id.clone(), format!("{} vs {}", b.fighter1_name, b.fighter2_name), status.to_string())
        }).collect()
    }

    pub fn get_battle(&mut self, id: &str) -> Option<&mut BattleRecord> {
        self.battles.iter_mut().find(|b| b.id == id)
    }

    pub fn clear_battles(&mut self) {
        self.battles.clear();
    }

    pub fn generate_battle_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("battle_{}", timestamp)
    }
}
```

### 3. **CLI Structure (`src/bin/colosseum.rs`)**

```rust
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select, Confirm};
use rinha_de_neopets::neopets::{Neopet, Behavior, BehaviorDef, Spell};
use rinha_de_neopets::battle::{battle_loop, BattleEvent};
use rinha_de_neopets::display::{BattleDisplay, BattleDisplayConfig};
use rinha_de_neopets::storage::{Storage, BattleRecord};
use std::path::Path;

mod storage; // Or move to src/lib.rs if sharing with casino

#[derive(Parser)]
#[command(name = "colosseum")]
#[command(about = "Neopets battle arena management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage fighters
    Fighter {
        #[command(subcommand)]
        action: FighterAction,
    },
    /// Manage battles
    Battle {
        #[command(subcommand)]
        action: BattleAction,
    },
    /// Clean up battles (remove all saved battles)
    Clean,
}

#[derive(Subcommand)]
enum FighterAction {
    /// Create a new fighter interactively
    Create,
    /// List all fighter names
    List,
    /// Show detailed fighter information
    Show { name: String },
}

#[derive(Subcommand)]
enum BattleAction {
    /// Create a battle between two fighters
    Create {
        fighter1: String,
        fighter2: String,
        /// Watch the battle live immediately
        #[arg(short, long)]
        watch: bool,
        /// Save the battle without watching
        #[arg(short, long)]
        save: bool,
    },
    /// Create N random battles between available fighters
    Random {
        count: usize,
    },
    /// List all saved battles
    List,
    /// Watch a saved battle
    Watch { id: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize storage
    let mut storage = Storage::new("assets/neopets.json", "assets/battles.json")?;

    match cli.command {
        Commands::Fighter { action } => match action {
            FighterAction::Create => create_fighter_interactive(&mut storage)?,
            FighterAction::List => list_fighters(&storage),
            FighterAction::Show { name } => show_fighter(&storage, &name),
        },
        Commands::Battle { action } => match action {
            BattleAction::Create { fighter1, fighter2, watch, save } => {
                create_battle(&mut storage, &fighter1, &fighter2, watch, save)?
            }
            BattleAction::Random { count } => create_random_battles(&mut storage, count)?,
            BattleAction::List => list_battles(&storage),
            BattleAction::Watch { id } => watch_battle(&mut storage, &id)?,
        },
        Commands::Clean => {
            storage.clear_battles();
            storage.save()?;
            println!("All battles cleared!");
        }
    }

    Ok(())
}

// Interactive fighter creation
fn create_fighter_interactive(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Create New Fighter ===\n");

    let name: String = Input::new()
        .with_prompt("Fighter name")
        .interact_text()?;

    let health: u32 = Input::new()
        .with_prompt("Health")
        .default(100)
        .interact_text()?;

    let base_attack: u32 = Input::new()
        .with_prompt("Base attack")
        .default(5)
        .interact_text()?;

    let base_defense: u32 = Input::new()
        .with_prompt("Base defense")
        .default(3)
        .interact_text()?;

    let heal_delta: u32 = Input::new()
        .with_prompt("Heal delta")
        .default(10)
        .interact_text()?;

    // Spells
    let mut spells = Vec::new();
    loop {
        let spell_name: String = Input::new()
            .with_prompt("Spell name (or leave empty to finish)")
            .allow_empty(true)
            .interact_text()?;

        if spell_name.is_empty() {
            break;
        }

        spells.push(Spell {
            name: spell_name,
            effect: serde_json::json!({}),
        });
    }

    // Behavior
    println!("\n=== Behavior Configuration ===");
    println!("Probabilities must sum to 1.0");
    
    let attack_chance: f64 = Input::new()
        .with_prompt("Attack chance (0.0-1.0)")
        .default(0.5)
        .interact_text()?;

    let heal_chance: f64 = Input::new()
        .with_prompt("Heal chance (0.0-1.0)")
        .default(0.25)
        .interact_text()?;

    let mut spell_chances = Vec::new();
    for (i, spell) in spells.iter().enumerate() {
        let chance: f64 = Input::new()
            .with_prompt(format!("Chance for spell '{}' (0.0-1.0)", spell.name))
            .default(0.125)
            .interact_text()?;
        spell_chances.push(chance);
    }

    let behavior_def = BehaviorDef {
        attack_chance,
        spell_chances,
        heal_chance,
    };

    // Construct and validate
    let neopet_def = NeopetDef {
        name: name.clone(),
        health,
        heal_delta,
        base_attack,
        base_defense,
        spells,
        behavior: behavior_def,
    };

    match Neopet::try_from(neopet_def) {
        Ok(neopet) => {
            storage.add_neopet(neopet)?;
            storage.save()?;
            println!("\n✅ Fighter '{}' created successfully!", name);
        }
        Err(e) => {
            println!("\n❌ Validation failed: {}", e);
            println!("Please try again with valid values.");
        }
    }

    Ok(())
}

fn list_fighters(storage: &Storage) {
    let names = storage.list_fighters();
    if names.is_empty() {
        println!("No fighters registered yet.");
    } else {
        println!("=== Registered Fighters ===");
        for name in names {
            println!("  • {}", name);
        }
    }
}

fn show_fighter(storage: &Storage, name: &str) {
    match storage.get_fighter(name) {
        Some(neopet) => {
            println!("=== Fighter Details ===\n");
            println!("{}", neopet);
        }
        None => println!("Fighter '{}' not found.", name),
    }
}

fn create_battle(
    storage: &mut Storage,
    fighter1_name: &str,
    fighter2_name: &str,
    watch: bool,
    save: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate fighters exist
    let fighter1 = storage.get_fighter(fighter1_name)
        .ok_or_else(|| format!("Fighter '{}' not found", fighter1_name))?;
    let fighter2 = storage.get_fighter(fighter2_name)
        .ok_or_else(|| format!("Fighter '{}' not found", fighter2_name))?;

    let battle_id = storage.generate_battle_id();

    if watch {
        // Run battle live
        println!("⚔️  Starting battle: {} vs {}\n", fighter1_name, fighter2_name);
        
        let events = battle_loop(fighter1, fighter2, &mut rand::rng());
        
        let config = BattleDisplayConfig::default();
        let mut display = BattleDisplay::with_config(fighter1, fighter2, config);
        display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
        display.display_battle_summary(&events);

        if save {
            // Save the completed battle
            let winner = events.iter().find_map(|e| {
                if let BattleEvent::BattleComplete { winner, .. } = e {
                    Some(winner.clone())
                } else {
                    None
                }
            });

            let battle_record = BattleRecord {
                id: battle_id,
                fighter1_name: fighter1_name.to_string(),
                fighter2_name: fighter2_name.to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                events,
                winner,
                is_completed: true,
            };

            storage.add_battle(battle_record);
            storage.save()?;
            println!("\n✅ Battle saved with ID: {}", battle_id);
        }
    } else {
        // Save as pending battle
        let battle_record = BattleRecord {
            id: battle_id.clone(),
            fighter1_name: fighter1_name.to_string(),
            fighter2_name: fighter2_name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            events: Vec::new(), // Empty until watched
            winner: None,
            is_completed: false,
        };

        storage.add_battle(battle_record);
        storage.save()?;
        println!("Battle scheduled with ID: {}\nRun 'colosseum battle watch {}' to see it!", battle_id, battle_id);
    }

    Ok(())
}

fn list_battles(storage: &Storage) {
    let battles = storage.list_battles();
    
    if battles.is_empty() {
        println!("No battles found.");
        return;
    }

    println!("=== Saved Battles ===");
    println!("{:<20} {:<30} {:<10}", "ID", "Matchup", "Status");
    println!("{}", "─".repeat(70));
    
    for (id, matchup, status) in battles {
        println!("{:<20} {:<30} {:<10}", id, matchup, status);
    }
}

fn watch_battle(storage: &mut Storage, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let battle = storage.get_battle(id)
        .ok_or_else(|| format!("Battle '{}' not found", id))?;

    if battle.is_completed && !battle.events.is_empty() {
        // Replay from stored events
        let fighter1 = storage.get_fighter(&battle.fighter1_name)
            .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter1_name))?;
        let fighter2 = storage.get_fighter(&battle.fighter2_name)
            .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter2_name))?;

        let config = BattleDisplayConfig::default();
        let mut display = BattleDisplay::with_config(fighter1, fighter2, config);
        display.display_battle_events(&battle.events, Some((fighter1.health, fighter2.health)));
        display.display_battle_summary(&battle.events);
    } else {
        // Run the battle now
        let fighter1 = storage.get_fighter(&battle.fighter1_name)
            .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter1_name))?;
        let fighter2 = storage.get_fighter(&battle.fighter2_name)
            .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter2_name))?;

        println!("⚔️  Starting battle: {} vs {}\n", battle.fighter1_name, battle.fighter2_name);
        
        let events = battle_loop(fighter1, fighter2, &mut rand::rng());
        
        let config = BattleDisplayConfig::default();
        let mut display = BattleDisplay::with_config(fighter1, fighter2, config);
        display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
        display.display_battle_summary(&events);

        // Update battle record
        battle.events = events;
        battle.is_completed = true;
        battle.winner = battle.events.iter().find_map(|e| {
            if let BattleEvent::BattleComplete { winner, .. } = e {
                Some(winner.clone())
            } else {
                None
            }
        });
        
        storage.save()?;
        println!("\n✅ Battle completed and saved.");
    }

    Ok(())
}
```

### 4. **Library Exports (`src/lib.rs`)**

Create `src/lib.rs` so `bin/colosseum.rs` can import:

```rust
// src/lib.rs
pub mod battle;
pub mod display;
pub mod neopets;
pub mod storage; // Add this module
```

### 5. **Usage Examples**

```bash
# Build and run
cargo run --bin colosseum

# Create a fighter
cargo run --bin colosseum fighter create

# List fighters
cargo run --bin colosseum fighter list

# Show fighter details
cargo run --bin colosseum fighter show Xweetok

# Create a battle (save for later)
cargo run --bin colosseum battle create Xweetok Acara --save

# Create and watch a battle immediately
cargo run --bin colosseum battle create Xweetok Acara --watch --save

# List battles
cargo run --bin colosseum battle list

# Watch a saved battle
cargo run --bin colosseum battle watch battle_1234567890

# Generate 5 random battles
cargo run --bin colosseum battle random 5

# Clean up all battles
cargo run --bin colosseum clean
```

## Key Design Decisions:

1. **✅ Interactive Fighter Creation**: Uses `dialoguer` for a guided wizard experience
2. **✅ Battle Storage**: Full `BattleEvent` history is stored in `battles.json`
3. **✅ Clean Command**: Simple `colosseum clean` to clear battles
4. **✅ Flexible Battle Execution**: Run immediately with `--watch` or save for later
5. **✅ Uses Existing Display**: Leverages your already-awesome `BattleDisplay`

## Next Steps:

1. Add dependencies: `cargo add clap --features derive` and `cargo add dialoguer chrono`
2. Create `src/lib.rs` with module exports
3. Create `src/storage.rs` with the storage layer
4. Populate `src/bin/colosseum.rs` with the CLI
5. Touch `assets/battles.json` (empty array `[]`) or let the code create it

The beauty is that the battle display and core logic remain unchanged - we're just adding a management layer on top!
