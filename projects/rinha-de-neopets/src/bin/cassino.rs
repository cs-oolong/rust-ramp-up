use clap::{Parser, Subcommand};
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use rinha_de_neopets::cassino_display::CassinoDisplay;
use rinha_de_neopets::cassino::CassinoEvent;
use colored::Colorize;

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
	AccumulatedBet {
	    #[arg(short, long, num_args = 1..)]
	    event_ids: Vec<String>,
	    #[arg(short, long)]
	    amount: f64,
	},
}



#[derive(Serialize, Deserialize, Debug, Clone)]
struct Bet {
    event_id: String,
    amount: f64,
    potential_win: f64,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccumulatedBet {
    event_ids: Vec<String>,
    amount: f64,
    combined_odds: f64,
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

#[derive(Serialize, Deserialize, Debug, Default)]
struct AccumulatedBets {
    accumulated_bets: Vec<AccumulatedBet>,
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

fn load_accumulated_bets() -> AccumulatedBets {
    let path = "assets/accumulated_bets.json";
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(accumulated_bets) => accumulated_bets,
                Err(_) => AccumulatedBets::default(),
            },
            Err(_) => AccumulatedBets::default(),
        }
    } else {
        AccumulatedBets::default()
    }
}

fn save_accumulated_bets(accumulated_bets: &AccumulatedBets) {
    let path = "assets/accumulated_bets.json";
    let json = serde_json::to_string_pretty(accumulated_bets)
        .expect("Failed to serialize accumulated bets");
    fs::write(path, json).expect("Failed to write accumulated bets to file");
}

fn place_bet_with_display(event_id: String, amount: f64, display: &CassinoDisplay) {
    // Validate that event_id is not empty
    if event_id.trim().is_empty() {
        display.show_error("No event ID provided for bet!");
        return;
    }
    
    // Validate that amount is positive
    if amount <= 0.0 {
        display.show_error("Bet amount must be greater than 0!");
        return;
    }
    
    // Show loading animation
    display.show_loading_animation("🔍 Verifying event...");
    
    // Load events to verify the event exists and get the odd
    let events_and_odds = load_events_and_odds();
    
    if let Some(event) = events_and_odds.events.get(&event_id) {
        // Calculate potential win (amount * odd)
        let potential_win = amount * event.odd;
        
        // Show processing animation
        display.show_loading_animation("💰 Processing bet...");
        
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
        
        // Display beautiful bet confirmation
        display.show_bet_placement(&event_id, amount, potential_win, event.odd, false);
    } else {
        display.show_error(&format!("Event '{}' not found! Use 'cassino list-events' to see available events.", event_id));
    }
}

fn place_accumulated_bet_with_display(event_ids: Vec<String>, amount: f64, display: &CassinoDisplay) {
    // Validate that event_ids is not empty
    if event_ids.is_empty() {
        display.show_error("No event IDs provided for accumulated bet!");
        display.show_info("Usage: cassino accumulated-bet --event-ids <EVENT_ID1> <EVENT_ID2> ... --amount <AMOUNT>");
        return;
    }
    
    // Validate that amount is positive
    if amount <= 0.0 {
        display.show_error("Bet amount must be greater than 0!");
        return;
    }
    
    display.show_loading_animation("🔍 Verifying events...");
    
    // Load events to verify all events exist and calculate combined odds
    let events_and_odds = load_events_and_odds();
    let mut combined_odds = 1.0;
    let mut valid_events = Vec::new();
    
    for event_id in &event_ids {
        if let Some(event) = events_and_odds.events.get(event_id) {
            combined_odds *= event.odd;
            valid_events.push((event_id.clone(), event.description.clone()));
        } else {
            display.show_error(&format!("Event '{}' not found! Use 'cassino list-events' to see available events.", event_id));
            return;
        }
    }
    
    // Calculate potential win (amount * combined_odds)
    let potential_win = amount * combined_odds;
    
    display.show_loading_animation("🎯 Processing accumulated bet...");
    
    // Create the accumulated bet
    let accumulated_bet = AccumulatedBet {
        event_ids: event_ids.clone(),
        amount,
        combined_odds,
        potential_win,
        timestamp: chrono::Local::now().to_rfc3339(),
    };
    
    // Load existing accumulated bets and add the new one
    let mut accumulated_bets = load_accumulated_bets();
    accumulated_bets.accumulated_bets.push(accumulated_bet);
    
    // Save accumulated bets
    save_accumulated_bets(&accumulated_bets);
    
    // Display beautiful accumulated bet confirmation
    display.show_bet_placement(&format!("{:?}", event_ids), amount, potential_win, combined_odds, true);
    
    // Show individual events in the bet
    println!("\n{}", "📋 Events in this accumulated bet:".yellow().bold());
    for (i, (event_id, description)) in valid_events.iter().enumerate() {
        let event = events_and_odds.events.get(event_id).unwrap();
        println!("  {}. {} ({}): Odd {:.2}", 
            (i + 1).to_string().cyan(),
            event_id.yellow(),
            description.white(),
            event.odd
        );
    }
}

fn create_event_interactively_with_display(display: &CassinoDisplay) {
    display.show_event_creation();
    
    // Get event description with styled prompt
    let description: String = Input::new()
        .with_prompt("📝 Enter event description")
        .interact_text()
        .expect("Failed to read description");
    
    // Get event odd with validation
    let odd: f64 = Input::new()
        .with_prompt("🎲 Enter event odd (must be > 0)")
        .validate_with(|input: &f64| {
            if *input > 0.0 {
                Ok(())
            } else {
                Err("Odd must be greater than 0")
            }
        })
        .interact_text()
        .expect("Failed to read odd");
    
    // Create the event
    let event = CassinoEvent {
        description: description.clone(),
        odd,
    };
    
    // Show loading animation while processing
    display.show_loading_animation("💾 Saving event to database...");
    
    // Load existing events and odds
    let mut events_and_odds = load_events_and_odds();
    
    // Generate a unique ID for the event
    let event_id = format!("event_{}", events_and_odds.events.len() + 1);
    
    // Add the new event
    events_and_odds.events.insert(event_id.clone(), event.clone());
    
    // Save to file
    save_events_and_odds(&events_and_odds);
    
    // Show success animation
    display.show_event_success(&event_id, &event.description, event.odd);
}

fn list_events_with_display(display: &CassinoDisplay) {
    display.show_loading_animation("📋 Loading available events...");
    
    let events_and_odds = load_events_and_odds();
    display.show_events_list(&events_and_odds.events);
}

fn main() {
    let cli = Cli::parse();
    let display = CassinoDisplay::new();
    
    // Show welcome banner
    display.show_welcome_banner();
    
    match cli.command {
    	Commands::Event => {
    		create_event_interactively_with_display(&display);
    	},
    	Commands::Cash => {
    		display.show_info("💰 Cash management feature coming soon!");
    	},
    	Commands::Bet { event_id, amount } => {
    		place_bet_with_display(event_id, amount, &display);
    	},
    	Commands::ListEvents => {
    		list_events_with_display(&display);
    	},
    	Commands::AccumulatedBet { event_ids, amount } => {
    		place_accumulated_bet_with_display(event_ids, amount, &display);
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
