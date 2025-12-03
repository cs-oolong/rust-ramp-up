use clap::{Parser, Subcommand};
use dialoguer::Input;
use rinha_de_neopets::neopets::{Neopet, NeopetDef, BehaviorDef, Spell};
use rinha_de_neopets::storage::{Storage, BattleRecord};
use rinha_de_neopets::display::{BattleDisplay, BattleDisplayConfig};

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
    /// Create a battle between two fighters and save it as pending
    Create {
        fighter1: String,
        fighter2: String,
    },
    /// Create N random battles between available fighters
    Random {
        count: usize,
    },
    /// List all completed battles
    Complete,
    /// List all pending battles
    Pending,
    /// Start/execute a pending battle
    Start {
        id: String,
        /// Display the battle live as it happens
        #[arg(short, long)]
        live: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize storage
    let mut storage = Storage::new("assets/neopets.json", "assets/complete_battles.json")?;

    match cli.command {
        Commands::Fighter { action } => match action {
            FighterAction::Create => create_fighter_interactive(&mut storage)?,
            FighterAction::List => list_fighters(&storage),
            FighterAction::Show { name } => show_fighter(&storage, &name),
        },
        Commands::Battle { action } => match action {
            BattleAction::Create { fighter1, fighter2 } => {
                create_battle(&mut storage, &fighter1, &fighter2)?
            }
            BattleAction::Random { count } => {
                create_random_battles(&mut storage, count)?
            }
            BattleAction::Complete => {
                list_complete_battles(&storage);
            }
            BattleAction::Pending => {
                list_pending_battles(&storage);
            }
            BattleAction::Start { id, live } => {
                start_battle(&mut storage, &id, live)?
            }
        },
        Commands::Clean => {
            clean_all_data(&mut storage)?;
        }
    }

    Ok(())
}

fn start_battle(
    storage: &mut Storage,
    battle_id: &str,
    live_display: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Find the pending battle
    let battle = storage.find_pending_battle(battle_id)
        .ok_or_else(|| format!("Pending battle '{}' not found", battle_id))?;

    // Get the fighters
    let fighter1 = storage.get_fighter(&battle.fighter1_name)
        .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter1_name))?;
    let fighter2 = storage.get_fighter(&battle.fighter2_name)
        .ok_or_else(|| format!("Fighter '{}' not found", battle.fighter2_name))?;

    println!("⚔️  Starting battle: {} vs {}\n", battle.fighter1_name, battle.fighter2_name);

    // Run the battle
    let events = rinha_de_neopets::battle::battle_loop(fighter1, fighter2, &mut rand::rng());

    // Determine winner from events
    let winner = events.iter().find_map(|e| {
        if let rinha_de_neopets::battle::BattleEvent::BattleComplete { winner, .. } = e {
            Some(winner.clone())
        } else {
            None
        }
    });

    if live_display {
        // Display the battle live
        let config = rinha_de_neopets::display::BattleDisplayConfig::default();
        let mut display = rinha_de_neopets::display::BattleDisplay::with_config(fighter1, fighter2, config);
        display.display_battle_events(&events, Some((fighter1.health, fighter2.health)));
        display.display_battle_summary(&events);
    } else {
        // Just show summary without live display
        println!("✅ Battle completed!");
        if let Some(ref winner_name) = winner {
            println!("🏆 Winner: {}", winner_name);
        } else {
            println!("🤝 Battle ended in a tie or max turns reached");
        }
        println!("📊 Total events: {}", events.len());
    }

    // Move battle from pending to complete
    storage.remove_pending_battle(battle_id);
    storage.move_battle_to_complete(battle, events, winner.clone());
    storage.save()?;

    println!("\n✅ Battle moved to complete history with ID: {}", battle_id);

    Ok(())
}

fn clean_all_data(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    // For testing purposes, we'll skip the interactive confirmation
    // and just clear the battle data directly
    
    // Clear all battle data
    storage.clear_complete_battles();
    storage.clear_pending_battles();
    
    storage.save()?;
    
    println!("✅ All battle data has been cleaned!");
    println!("Note: Fighter data has been preserved.");
    println!("All battles have been cleared.");

    Ok(())
}

fn create_random_battles(
    storage: &mut Storage,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let fighters = storage.list_fighters();
    
    if fighters.len() < 2 {
        return Err("Need at least 2 fighters to create battles".into());
    }

    let mut created_count = 0;
    
    for _ in 0..count {
        // Randomly select two different fighters
        use rand::Rng;
        
        let mut rng = rand::rng();
        let fighter1_idx = rng.random_range(0..fighters.len());
        let mut fighter2_idx = rng.random_range(0..fighters.len());
        
        // Ensure different fighters
        while fighter2_idx == fighter1_idx {
            fighter2_idx = rng.random_range(0..fighters.len());
        }
        
        let fighter1 = &fighters[fighter1_idx];
        let fighter2 = &fighters[fighter2_idx];
        
        // Create the battle
        let battle_id = storage.generate_battle_id();
        let created_at = chrono::Utc::now().to_rfc3339();

        let battle_record = BattleRecord {
            id: battle_id.clone(),
            fighter1_name: fighter1.clone(),
            fighter2_name: fighter2.clone(),
            created_at: created_at.clone(),
            events: Vec::new(),
            winner: None,
            is_completed: false,
        };

        storage.add_pending_battle(battle_record);
        created_count += 1;
    }

    storage.save()?;

    println!("✅ Created {} random battles successfully!", created_count);
    println!("Use 'colosseum battle pending' to see all pending battles");

    Ok(())
}

fn list_complete_battles(storage: &Storage) {
    let battles = storage.list_complete_battles();
    
    if battles.is_empty() {
        println!("No completed battles found.");
        return;
    }

    println!("=== Completed Battles ===");
    println!("{:<20} {:<30} {:<10}", "ID", "Matchup", "Status");
    println!("{}", "─".repeat(70));
    
    for (id, matchup, status) in battles {
        println!("{:<20} {:<30} {:<10}", id, matchup, status);
    }
}

fn list_pending_battles(storage: &Storage) {
    let battles = storage.list_pending_battles();
    
    if battles.is_empty() {
        println!("No pending battles found.");
        return;
    }

    println!("=== Pending Battles ===");
    println!("{:<20} {:<30} {:<20}", "ID", "Matchup", "Created At");
    println!("{}", "─".repeat(80));
    
    for (id, matchup, created_at) in battles {
        // Format the timestamp to be more readable
        let formatted_time = if created_at.len() > 19 {
            &created_at[..19] // Take first 19 chars (YYYY-MM-DDTHH:MM:SS)
        } else {
            &created_at
        };
        println!("{:<20} {:<30} {:<20}", id, matchup, formatted_time);
    }
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
    for (_i, spell) in spells.iter().enumerate() {
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
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate fighters exist
    let _fighter1 = storage.get_fighter(fighter1_name)
        .ok_or_else(|| format!("Fighter '{}' not found", fighter1_name))?;
    let _fighter2 = storage.get_fighter(fighter2_name)
        .ok_or_else(|| format!("Fighter '{}' not found", fighter2_name))?;

    // Prevent self-battles
    if fighter1_name == fighter2_name {
        return Err("A fighter cannot battle themselves".into());
    }

    let battle_id = storage.generate_battle_id();
    let created_at = chrono::Utc::now().to_rfc3339();

    // Create pending battle record
    let battle_record = BattleRecord {
        id: battle_id.clone(),
        fighter1_name: fighter1_name.to_string(),
        fighter2_name: fighter2_name.to_string(),
        created_at: created_at.clone(),
        events: Vec::new(), // Empty until battle is run
        winner: None,
        is_completed: false,
    };

    storage.add_pending_battle(battle_record);
    storage.save()?;

    println!("✅ Battle created successfully!");
    println!("ID: {}", battle_id);
    println!("Matchup: {} vs {}", fighter1_name, fighter2_name);
    println!("Created: {}", created_at);
    println!("\nUse 'colosseum battle pending' to see all pending battles");

    Ok(())
}