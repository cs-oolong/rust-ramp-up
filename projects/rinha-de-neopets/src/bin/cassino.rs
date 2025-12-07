use clap::{Parser, Subcommand};
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use rinha_de_neopets::cassino_display::CassinoDisplay;
use rinha_de_neopets::cassino::{CassinoEvent, CompletedEvent, ExpiredBet, ExpiredAccumulatedBet, DoneEvents, ExpiredBets};
use rand;
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
	RunEvent {
	    #[arg(short, long)]
	    event_id: String,
	},
	RunAllEvents,
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

fn load_done_events() -> DoneEvents {
    let path = "assets/done.json";
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(done_events) => done_events,
                Err(_) => DoneEvents::default(),
            },
            Err(_) => DoneEvents::default(),
        }
    } else {
        DoneEvents::default()
    }
}

fn save_done_events(done_events: &DoneEvents) {
    let path = "assets/done.json";
    let json = serde_json::to_string_pretty(done_events)
        .expect("Failed to serialize done events");
    fs::write(path, json).expect("Failed to write done events to file");
}

fn load_expired_bets() -> ExpiredBets {
    let path = "assets/expired_bets.json";
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(expired_bets) => expired_bets,
                Err(_) => ExpiredBets::default(),
            },
            Err(_) => ExpiredBets::default(),
        }
    } else {
        ExpiredBets::default()
    }
}

fn save_expired_bets(expired_bets: &ExpiredBets) {
    let path = "assets/expired_bets.json";
    let json = serde_json::to_string_pretty(expired_bets)
        .expect("Failed to serialize expired bets");
    fs::write(path, json).expect("Failed to write expired bets to file");
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

fn run_event_with_display(event_id: String, display: &CassinoDisplay) {
    display.show_loading_animation(&format!("🎲 Running event {}...", event_id));
    
    // Load events and odds
    let mut events_and_odds = load_events_and_odds();
    
    // Check if event exists
    if let Some(event) = events_and_odds.events.get(&event_id).cloned() {
        // Randomly determine if event occurred (50% chance)
        let event_occurred = rand::random::<bool>();
        
        // Create completed event
        let completed_event = CompletedEvent {
            event_id: event_id.clone(),
            description: event.description.clone(),
            odd: event.odd,
            result: event_occurred,
            timestamp: chrono::Local::now().to_rfc3339(),
        };
        
        // Remove event from active events
        events_and_odds.events.remove(&event_id);
        
        // Load existing done events and add the new one
        let mut done_events = load_done_events();
        done_events.completed_events.push(completed_event.clone());
        
        // Process bets for this event
        let mut bets = load_bets();
        let mut expired_bets = load_expired_bets();
        let mut total_spent = 0.0;
        let mut total_earned = 0.0;
        
        // Process individual bets
        let mut remaining_bets = Vec::new();
        for bet in bets.bets {
            if bet.event_id == event_id {
                // This bet is for the event we're running
                total_spent += bet.amount;
                
                let actual_payout = if event_occurred {
                    bet.potential_win
                } else {
                    0.0
                };
                
                total_earned += actual_payout;
                
                let expired_bet = ExpiredBet {
                    event_id: bet.event_id,
                    amount: bet.amount,
                    potential_win: bet.potential_win,
                    result: event_occurred,
                    actual_payout,
                    timestamp: bet.timestamp,
                };
                
                expired_bets.expired_bets.push(expired_bet);
            } else {
                // Keep bets for other events
                remaining_bets.push(bet);
            }
        }
        bets.bets = remaining_bets;
        
        // Process accumulated bets
        let mut accumulated_bets = load_accumulated_bets();
        let mut remaining_accumulated_bets = Vec::new();
        
        for acc_bet in accumulated_bets.accumulated_bets {
            if acc_bet.event_ids.contains(&event_id) {
                // This accumulated bet contains the event we're running
                total_spent += acc_bet.amount;
                
                // For accumulated bets, all events must occur for the bet to win
                // Since we're only running one event at a time, we'll consider it a loss
                // In a real system, you'd wait for all events to be run
                let expired_acc_bet = ExpiredAccumulatedBet {
                    event_ids: acc_bet.event_ids,
                    amount: acc_bet.amount,
                    combined_odds: acc_bet.combined_odds,
                    potential_win: acc_bet.potential_win,
                    all_events_occurred: false, // Simplified: assume loss when any event is run
                    actual_payout: 0.0,
                    timestamp: acc_bet.timestamp,
                };
                
                expired_bets.expired_accumulated_bets.push(expired_acc_bet);
                // Don't add to remaining since this bet is now expired
            } else {
                // Keep accumulated bets that don't contain this event
                remaining_accumulated_bets.push(acc_bet);
            }
        }
        accumulated_bets.accumulated_bets = remaining_accumulated_bets;
        
        // Save all changes
        save_events_and_odds(&events_and_odds);
        save_done_events(&done_events);
        save_bets(&bets);
        save_accumulated_bets(&accumulated_bets);
        save_expired_bets(&expired_bets);
        
        // Display results
        display.show_event_result(&event_id, &event.description, event_occurred, event.odd, total_spent, total_earned);
        
    } else {
        display.show_error(&format!("Event '{}' not found!", event_id));
    }
}

fn run_all_events_with_display(display: &CassinoDisplay) {
    display.show_loading_animation("🎲 Running all events...");
    
    // Load all events
    let events_and_odds = load_events_and_odds();
    let event_ids: Vec<String> = events_and_odds.events.keys().cloned().collect();
    
    if event_ids.is_empty() {
        display.show_info("No events to run!");
        return;
    }
    
    let mut total_spent = 0.0;
    let mut total_earned = 0.0;
    let mut results = Vec::new();
    
    // Run each event
    for event_id in event_ids {
        // Load fresh data for each event since previous events may have modified the state
        let mut current_events = load_events_and_odds();
        
        if let Some(event) = current_events.events.get(&event_id).cloned() {
            // Randomly determine if event occurred
            let event_occurred = rand::random::<bool>();
            
            // Create completed event
            let completed_event = CompletedEvent {
                event_id: event_id.clone(),
                description: event.description.clone(),
                odd: event.odd,
                result: event_occurred,
                timestamp: chrono::Local::now().to_rfc3339(),
            };
            
            // Remove event from active events
            current_events.events.remove(&event_id);
            
            // Load existing done events and add the new one
            let mut done_events = load_done_events();
            done_events.completed_events.push(completed_event.clone());
            
            // Process bets for this event
            let mut bets = load_bets();
            let mut expired_bets = load_expired_bets();
            
            // Process individual bets
            let mut remaining_bets = Vec::new();
            for bet in bets.bets {
                if bet.event_id == event_id {
                    total_spent += bet.amount;
                    
                    let actual_payout = if event_occurred {
                        bet.potential_win
                    } else {
                        0.0
                    };
                    
                    total_earned += actual_payout;
                    
                    let expired_bet = ExpiredBet {
                        event_id: bet.event_id,
                        amount: bet.amount,
                        potential_win: bet.potential_win,
                        result: event_occurred,
                        actual_payout,
                        timestamp: bet.timestamp,
                    };
                    
                    expired_bets.expired_bets.push(expired_bet);
                } else {
                    remaining_bets.push(bet);
                }
            }
            bets.bets = remaining_bets;
            
            // For accumulated bets, we need to track which events have been processed
            // and only mark them as expired when all their events have been run
            // For now, let's just collect the results and process accumulated bets at the end
            
            results.push((event_id.clone(), event.description.clone(), event_occurred, event.odd));
            
            // Save changes for this event
            save_events_and_odds(&current_events);
            save_done_events(&done_events);
            save_bets(&bets);
            save_expired_bets(&expired_bets);
        }
    }
    
    // Now process accumulated bets
    process_accumulated_bets_after_all_events(&mut total_spent, &mut total_earned);
    
    // Display summary
    display.show_all_events_result(results, total_spent, total_earned);
}

fn process_accumulated_bets_after_all_events(total_spent: &mut f64, total_earned: &mut f64) {
    let mut accumulated_bets = load_accumulated_bets();
    let mut expired_bets = load_expired_bets();
    let done_events = load_done_events();
    
    // Create a map of event results for quick lookup
    let event_results: HashMap<String, bool> = done_events.completed_events
        .iter()
        .map(|e| (e.event_id.clone(), e.result))
        .collect();
    
    let mut remaining_accumulated_bets = Vec::new();
    
    for acc_bet in accumulated_bets.accumulated_bets {
        // Check if all events in this accumulated bet have been processed
        let all_events_processed = acc_bet.event_ids.iter()
            .all(|event_id| event_results.contains_key(event_id));
        
        if all_events_processed {
            // All events have been processed, determine if bet won
            *total_spent += acc_bet.amount;
            
            let all_events_occurred = acc_bet.event_ids.iter()
                .all(|event_id| *event_results.get(event_id).unwrap_or(&false));
            
            let actual_payout = if all_events_occurred {
                acc_bet.potential_win
            } else {
                0.0
            };
            
            *total_earned += actual_payout;
            
            let expired_acc_bet = ExpiredAccumulatedBet {
                event_ids: acc_bet.event_ids,
                amount: acc_bet.amount,
                combined_odds: acc_bet.combined_odds,
                potential_win: acc_bet.potential_win,
                all_events_occurred,
                actual_payout,
                timestamp: acc_bet.timestamp,
            };
            
            expired_bets.expired_accumulated_bets.push(expired_acc_bet);
        } else {
            // Keep accumulated bet for later
            remaining_accumulated_bets.push(acc_bet);
        }
    }
    
    accumulated_bets.accumulated_bets = remaining_accumulated_bets;
    save_accumulated_bets(&accumulated_bets);
    save_expired_bets(&expired_bets);
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
    	},
    	Commands::RunEvent { event_id } => {
    		run_event_with_display(event_id, &display);
    	},
    	Commands::RunAllEvents => {
    		run_all_events_with_display(&display);
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
