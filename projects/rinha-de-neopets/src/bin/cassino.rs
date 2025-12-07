use clap::{Parser, Subcommand};
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "cassino")]
#[command(about = "Neopets battle arena bets management CLI")]

struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Event,
	Cash,
	Bet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CassinoEvent {
    description: String,
    odd: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EventsAndOdds {
    events: HashMap<String, CassinoEvent>,
}

fn load_events_and_odds() -> EventsAndOdds {
    let path = "assets/events_and_odds.json";
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(events_and_odds) => events_and_odds,
                Err(_) => EventsAndOdds::default(),
            },
            Err(_) => EventsAndOdds::default(),
        }
    } else {
        EventsAndOdds::default()
    }
}

fn save_events_and_odds(events_and_odds: &EventsAndOdds) {
    let path = "assets/events_and_odds.json";
    let json = serde_json::to_string_pretty(events_and_odds)
        .expect("Failed to serialize events and odds");
    fs::write(path, json).expect("Failed to write events and odds to file");
}

fn create_event_interactively() {
    println!("Creating a new event...");
    
    // Get event description
    let description: String = Input::new()
        .with_prompt("Enter event description")
        .interact_text()
        .expect("Failed to read description");
    
    // Get event odd
    let odd: f64 = Input::new()
        .with_prompt("Enter event odd")
        .validate_with(|input: &f64| {
            if *input > 0.0 {
                Ok(())
            } else {
                Err("Odd must be greater than 0")
            }
        })
        .interact_text()
        .expect("Failed to read odd");
    
    // Create and display the event
    let event = CassinoEvent {
        description: description.clone(),
        odd,
    };
    
    // Load existing events and odds
    let mut events_and_odds = load_events_and_odds();
    
    // Generate a unique ID for the event
    let event_id = format!("event_{}", events_and_odds.events.len() + 1);
    
    // Add the new event
    events_and_odds.events.insert(event_id.clone(), event.clone());
    
    // Save to file
    save_events_and_odds(&events_and_odds);
    
    println!("\nEvent created successfully!");
    println!("Event ID: {}", event_id);
    println!("Description: {}", event.description);
    println!("Odd: {:.2}", event.odd);
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
    	Commands::Event => {
    		create_event_interactively();
    	},
    	Commands::Cash => {
    		println!("cash command called");
    	},
    	Commands::Bet => {
    		println!("bet command called");
    	}
    }
}

// user can add cash (not real cash though) to their account
// user can list available pending battles
// user can create a simple bet
    // has to provide the battle id
    // has to pick just one among the possible events
// user can create a combined bet regarding a single battle, but multiple events within it
    // provide battle id
        // pick N events
// user can create a combined bet regarding N battles and M events
    // provide battle id
        // provide events
    // provide battle id
        // provide events
    // and so on, until done
// placing a bet deducts from the user's cash
// can't bet if no cash
// events have odds, if they occur, player receives cash
