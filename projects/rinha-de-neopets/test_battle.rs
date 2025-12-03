use rinha_de_neopets::battle::battle_loop;
use rinha_de_neopets::neopets::load_neopets;

fn main() {
    println!("Testing battle system...");
    
    let neopets_set = load_neopets("assets/neopets.json");
    let fighter1 = &neopets_set[0];
    let fighter2 = &neopets_set[1];
    
    println!("Fighter 1: {} (HP: {})", fighter1.name, fighter1.health);
    println!("Fighter 2: {} (HP: {})", fighter2.name, fighter2.health);
    
    println!("\nStarting battle simulation...");
    let events = battle_loop(fighter1, fighter2, &mut rand::rng());
    
    println!("\nBattle completed! Total events: {}", events.len());
    
    // Look for battle completion events
    let completion_events: Vec<_> = events.iter().filter(|e| {
        matches!(e, rinha_de_neopets::battle::BattleEvent::BattleComplete { .. })
    }).collect();
    
    if !completion_events.is_empty() {
        println!("Found {} BattleComplete events!", completion_events.len());
        for event in &completion_events {
            if let rinha_de_neopets::battle::BattleEvent::BattleComplete { 
                turn, winner, loser, winner_final_hp, loser_final_hp, completion_reason 
            } = event {
                println!("Battle completed on turn {}", turn);
                println!("Winner: {} ({} HP)", winner, winner_final_hp);
                println!("Loser: {} ({} HP)", loser, loser_final_hp);
            }
        }
    } else {
        println!("No BattleComplete events found. Battle may have timed out.");
    }
    
    // Show some sample events
    println!("\nFirst 10 events:");
    for (i, event) in events.iter().take(10).enumerate() {
        println!("Event {}: {:?}", i + 1, event);
    }
}