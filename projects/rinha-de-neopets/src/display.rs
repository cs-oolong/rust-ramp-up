use crate::battle::BattleEvent;
use colored::*;
use std::collections::HashMap;

/// Main entry point: Display a list of battle events in a beautiful format
pub fn display_battle_events(events: &[BattleEvent]) {
    if events.is_empty() {
        println!("{}", "No battle events to display.".dimmed());
        return;
    }

    // Group events by turn for better organization
    let mut events_by_turn: HashMap<u32, Vec<&BattleEvent>> = HashMap::new();
    for event in events {
        let turn = match event {
            BattleEvent::Roll { turn, .. } => *turn,
            BattleEvent::Attack { turn, .. } => *turn,
            BattleEvent::Heal { turn, .. } => *turn,
            BattleEvent::SpellCast { turn, .. } => *turn,
        };
        events_by_turn.entry(turn).or_insert_with(Vec::new).push(event);
    }

    // Sort turns (0 is initiative phase)
    let mut turns: Vec<u32> = events_by_turn.keys().cloned().collect();
    turns.sort_unstable();

    // Display header
    println!("{}", "═".repeat(60).bright_black());
    let battle_header = "⚔️  BATTLE BEGINS ⚔️".bright_yellow().bold();
    let centered_header = center_text(&battle_header.to_string(), 60);
    println!("{}", centered_header);
    println!("{}", "═".repeat(60).bright_black());

    // Display events grouped by turn
    for turn in turns {
        let turn_events = &events_by_turn[&turn];
        
        if turn == 0 {
            // Initiative phase
            println!("\n{}", "🏁 INITIATIVE PHASE".bright_cyan().bold());
        } else {
            display_turn_header(turn);
        }

        for event in turn_events {
            display_single_event(event);
        }

        // Add spacing between turns (except after initiative)
        if turn != 0 {
            println!();
        }
    }

    // Display footer
    println!("{}", "═".repeat(60).bright_black());
}

/// Display a turn header with nice formatting
fn display_turn_header(turn: u32) {
    let header = format!(" TURN {} ", turn);
    let padding = "─".repeat((60 - header.len()) / 2);
    let line = format!("{}{}{}", padding, header.bright_white().bold(), padding);
    
    // If odd length, add one more dash
    let line = if line.len() < 60 {
        format!("{}", line)
    } else {
        line
    };
    
    println!("\n{}", line.bright_blue());
}

/// Display a single battle event with appropriate styling
fn display_single_event(event: &BattleEvent) {
    match event {
        BattleEvent::Roll { actor, dice, final_value, is_positive_crit, is_negative_crit, goal, .. } => {
            display_roll_event(actor, *dice, *final_value, *is_positive_crit, *is_negative_crit, goal);
        }
        BattleEvent::Attack { actor, target, actual_damage, .. } => {
            display_attack_event(actor, target, *actual_damage);
        }
        BattleEvent::Heal { actor, amount, .. } => {
            display_heal_event(actor, *amount);
        }
        BattleEvent::SpellCast { actor, target, spell_name, .. } => {
            display_spell_event(actor, target, spell_name);
        }
    }
}

/// Display dice roll events with special styling for critical hits
fn display_roll_event(actor: &str, dice: u8, final_value: u32, is_positive_crit: bool, is_negative_crit: bool, goal: &str) {
    let dice_display = if is_positive_crit {
        format!("{}", dice).on_bright_yellow().red().bold()
    } else if is_negative_crit {
        format!("{}", dice).on_red().white().bold()
    } else {
        dice.to_string().normal()
    };

    let goal_icon = match goal {
        "attack" => "⚔️",
        "defense" => "🛡️",
        "heal" => "💚",
        "initiative" => "🎲",
        _ => "🎲",
    };

    let actor_colored = actor.bright_cyan();
    let goal_colored = goal.bright_white();
    let final_value_colored = if is_positive_crit {
        final_value.to_string().bright_yellow().bold()
    } else if is_negative_crit {
        final_value.to_string().bright_red().bold()
    } else {
        final_value.to_string().normal()
    };

    println!("  {} {} rolls {} for {}: {} = {}", 
        goal_icon,
        actor_colored,
        goal_colored,
        dice_display,
        "🎯".bright_yellow(),
        final_value_colored
    );

    // Add special message for critical hits
    if is_positive_crit {
        println!("     {}", "⭐ NATURAL 20! Critical Success! ⭐".bright_yellow().bold());
    } else if is_negative_crit {
        println!("     {}", "💥 NATURAL 1! Critical Failure! 💥".bright_red().bold());
    }
}

/// Display attack events with damage information
fn display_attack_event(actor: &str, target: &str, actual_damage: u32) {
    let actor_colored = actor.bright_blue().bold();
    let target_colored = target.bright_red().bold();

    if actual_damage == 0 {
        println!("  ⚔️  {} attacks {} but the attack is {}", 
            actor_colored,
            target_colored,
            "BLOCKED!".bright_white().on_red()
        );
    } else {
        let damage_colored = actual_damage.to_string().bright_red().bold();
        println!("  ⚔️  {} hits {} for {} damage", 
            actor_colored,
            target_colored,
            damage_colored
        );
    }
}

/// Display healing events
fn display_heal_event(actor: &str, amount: u32) {
    let actor_colored = actor.bright_green().bold();
    let amount_colored = amount.to_string().bright_green().bold();
    
    println!("  💚 {} heals for {} HP", actor_colored, amount_colored);
}

/// Display spell casting events
fn display_spell_event(actor: &str, target: &str, spell_name: &str) {
    let actor_colored = actor.bright_magenta().bold();
    let target_colored = target.bright_red().bold();
    let spell_colored = spell_name.bright_yellow().italic();
    
    println!("  ✨ {} casts {} on {}", actor_colored, spell_colored, target_colored);
}

/// Center text helper function
fn center_text(text: &str, width: usize) -> String {
    let len = text.len();
    if len >= width {
        text.to_string()
    } else {
        let padding = (width - len) / 2;
        let left_pad = " ".repeat(padding);
        let right_pad = " ".repeat(width - len - padding);
        format!("{}{}{}", left_pad, text, right_pad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_empty_events() {
        display_battle_events(&[]);
    }

    #[test]
    fn test_display_single_roll() {
        let events = vec![BattleEvent::Roll {
            turn: 1,
            actor: "Pikachu".to_string(),
            dice: 15,
            final_value: 25,
            is_positive_crit: false,
            is_negative_crit: false,
            goal: "attack".to_string(),
        }];
        display_battle_events(&events);
    }

    #[test]
    fn test_display_critical_hit() {
        let events = vec![
            BattleEvent::Roll {
                turn: 1,
                actor: "Pikachu".to_string(),
                dice: 20,
                final_value: 30,
                is_positive_crit: true,
                is_negative_crit: false,
                goal: "attack".to_string(),
            },
            BattleEvent::Attack {
                turn: 1,
                actor: "Pikachu".to_string(),
                target: "Charizard".to_string(),
                raw_damage: 30,
                shield_value: 10,
                actual_damage: 40, // Doubled from crit
            },
        ];
        display_battle_events(&events);
    }
}