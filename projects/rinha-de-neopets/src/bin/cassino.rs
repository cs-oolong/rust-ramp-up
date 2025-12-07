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
	Bet {
	    #[arg(short, long)]
	    event_id: String,
	    #[arg(short, long)]
	    amount: f64,
	},
	ListEvents,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CassinoEvent {
    description: String,
    odd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Bet {
    event_id: String,
    amount: f64,
    potential_win: f64,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EventsAndOdds {
    events: HashMap<String, CassinoEvent>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Bets {
    bets: Vec<Bet>,
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

fn load_bets() -> Bets {
    let path = "assets/bets.json";
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(bets) => bets,
                Err(_) => Bets::default(),
            },
            Err(_) => Bets::default(),
        }
    } else {
        Bets::default()
    }
}

fn save_bets(bets: &Bets) {
    let path = "assets/bets.json";
    let json = serde_json::to_string_pretty(bets)
        .expect("Failed to serialize bets");
    fs::write(path, json).expect("Failed to write bets to file");
}

fn place_bet(event_id: String, amount: f64) {
    // Load events to verify the event exists and get the odd
    let events_and_odds = load_events_and_odds();
    
    if let Some(event) = events_and_odds.events.get(&event_id) {
        // Calculate potential win (amount * odd)
        let potential_win = amount * event.odd;
        
        // Create the bet
        let bet = Bet {
            event_id: event_id.clone(),
            amount,
            potential_win,
            timestamp: chrono::Local::now().to_rfc3339(),
        };
        
        // Load existing bets and add the new one
        let mut bets = load_bets();
        bets.bets.push(bet);
        
        // Save bets
        save_bets(&bets);
        
        println!("Bet placed successfully!");
        println!("Event ID: {}", event_id);
        println!("Event: {}", event.description);
        println!("Bet amount: {:.2}", amount);
        println!("Potential win: {:.2}", potential_win);
        println!("Odd: {:.2}", event.odd);
    } else {
        println!("Error: Event '{}' not found!", event_id);
        println!("Use 'cassino list-events' to see available events.");
    }
}

fn list_events() {
    let events_and_odds = load_events_and_odds();
    
    if events_and_odds.events.is_empty() {
        println!("No events available. Create some events first with 'cassino event'");
        return;
    }
    
    println!("Available Events:");
    println!("==================");
    
    for (event_id, event) in &events_and_odds.events {
        println!("Event ID: {}", event_id);
        println!("Description: {}", event.description);
        println!("Odd: {:.2}", event.odd);
        println!("------------------");
    }
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
    	Commands::Bet { event_id, amount } => {
    		place_bet(event_id, amount);
    	},
    	Commands::ListEvents => {
    		list_events();
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
