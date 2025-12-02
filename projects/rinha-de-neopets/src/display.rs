use crate::battle::BattleEvent;
use crate::neopets::Neopet;
use colored::*;
use std::collections::HashMap;

/// Enhanced battle display with health tracking and summaries
pub struct BattleDisplay {
    fighter1_name: String,
    fighter2_name: String,
    fighter1_health: u32,
    fighter2_health: u32,
    fighter1_max_health: u32,
    fighter2_max_health: u32,
    total_damage_dealt: HashMap<String, u32>,
    total_healing_done: HashMap<String, u32>,
    spells_cast: HashMap<String, Vec<String>>,
}

impl BattleDisplay {
    pub fn new(fighter1: &Neopet, fighter2: &Neopet) -> Self {
        let mut total_damage_dealt = HashMap::new();
        let mut total_healing_done = HashMap::new();
        let mut spells_cast = HashMap::new();
        
        total_damage_dealt.insert(fighter1.name.clone(), 0);
        total_damage_dealt.insert(fighter2.name.clone(), 0);
        total_healing_done.insert(fighter1.name.clone(), 0);
        total_healing_done.insert(fighter2.name.clone(), 0);
        spells_cast.insert(fighter1.name.clone(), Vec::new());
        spells_cast.insert(fighter2.name.clone(), Vec::new());
        
        Self {
            fighter1_name: fighter1.name.clone(),
            fighter2_name: fighter2.name.clone(),
            fighter1_health: fighter1.health,
            fighter2_health: fighter2.health,
            fighter1_max_health: fighter1.health,
            fighter2_max_health: fighter2.health,
            total_damage_dealt,
            total_healing_done,
            spells_cast,
        }
    }
    
    /// Update health based on battle events
    fn update_health(&mut self, events: &[BattleEvent]) {
        for event in events {
            match event {
                BattleEvent::Attack { actor, target, actual_damage, .. } => {
                    if target == &self.fighter1_name {
                        self.fighter1_health = self.fighter1_health.saturating_sub(*actual_damage);
                    } else if target == &self.fighter2_name {
                        self.fighter2_health = self.fighter2_health.saturating_sub(*actual_damage);
                    }
                    
                    // Track damage dealt
                    *self.total_damage_dealt.entry(actor.clone()).or_insert(0) += actual_damage;
                }
                BattleEvent::Heal { actor, amount, .. } => {
                    if actor == &self.fighter1_name {
                        self.fighter1_health = (self.fighter1_health + amount).min(self.fighter1_max_health);
                    } else if actor == &self.fighter2_name {
                        self.fighter2_health = (self.fighter2_health + amount).min(self.fighter2_max_health);
                    }
                    
                    // Track healing done
                    *self.total_healing_done.entry(actor.clone()).or_insert(0) += amount;
                }
                BattleEvent::SpellCast { actor, spell_name, .. } => {
                    self.spells_cast.entry(actor.clone()).or_insert_with(Vec::new).push(spell_name.clone());
                }
                _ => {}
            }
        }
    }
    
    /// Display battle events with health tracking
    pub fn display_battle_events(&mut self, events: &[BattleEvent]) {
        if events.is_empty() {
            println!("{}", "No battle events to display.".dimmed());
            return;
        }

        // Update health based on events
        self.update_health(events);

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
        println!("{}", "═".repeat(70).bright_black());
        let battle_header = "⚔️  BATTLE BEGINS ⚔️".bright_yellow().bold();
        let centered_header = center_text(&battle_header.to_string(), 70);
        println!("{}", centered_header);
        
        // Display initial health bars
        println!("\n{}", "Initial Status:".bright_white().bold());
        self.display_health_bars();
        
        println!("{}", "═".repeat(70).bright_black());

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
                self.display_single_event(event);
            }

            // Show health bars after each turn (except initiative)
            if turn != 0 && turn % 10 == 0 {
                println!();
                self.display_health_bars();
            }

            // Add spacing between turns (except after initiative)
            if turn != 0 {
                println!();
            }
        }

        // Display footer and summary
        println!("{}", "═".repeat(70).bright_black());
        println!("{}", "🏁 BATTLE COMPLETE 🏁".bright_green().bold().center(70));
        println!("{}", "═".repeat(70).bright_black());
        
        self.display_battle_summary();
    }
    
    /// Display health bars for both fighters
    fn display_health_bars(&self) {
        println!();
        self.display_single_health_bar(&self.fighter1_name, self.fighter1_health, self.fighter1_max_health);
        self.display_single_health_bar(&self.fighter2_name, self.fighter2_health, self.fighter2_max_health);
        println!();
    }
    
    /// Display a single health bar using indicatif progress bar styling
    fn display_single_health_bar(&self, name: &str, current: u32, max: u32) {
        let percentage = if max > 0 { (current as f64 / max as f64 * 100.0) as u32 } else { 0 };
        let bar_width = 30;
        let filled_width = (bar_width as f64 * percentage as f64 / 100.0) as usize;
        let empty_width = bar_width - filled_width;
        
        let health_color = if percentage > 50 {
            "🟢".green()
        } else if percentage > 25 {
            "🟡".yellow()
        } else {
            "🔴".red()
        };
        
        let filled_bar = "█".repeat(filled_width).bright_red();
        let empty_bar = "░".repeat(empty_width).bright_black();
        
        let name_colored = name.bright_cyan().bold();
        let hp_text = format!("{}/{}", current, max).bright_white();
        let percentage_text = format!("{:3}%", percentage).bright_yellow();
        
        println!("  {} {}❤️  [{}{}] {} {}", 
            name_colored,
            health_color,
            filled_bar,
            empty_bar,
            hp_text,
            percentage_text
        );
    }
    
    /// Display a single battle event with appropriate styling
    fn display_single_event(&mut self, event: &BattleEvent) {
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
    
    /// Display battle summary with statistics
    fn display_battle_summary(&self) {
        println!("\n{}", "📊 BATTLE SUMMARY".bright_white().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        // Final health status
        println!("\n{}", "Final Health Status:".bright_white().underline());
        self.display_health_bars();
        
        // Damage statistics
        println!("\n{}", "Damage Dealt:".bright_red().underline());
        for (fighter, damage) in &self.total_damage_dealt {
            let fighter_colored = fighter.bright_cyan().bold();
            let damage_colored = damage.to_string().bright_red().bold();
            println!("  {}: {} total damage", fighter_colored, damage_colored);
        }
        
        // Healing statistics
        println!("\n{}", "Healing Done:".bright_green().underline());
        for (fighter, healing) in &self.total_healing_done {
            let fighter_colored = fighter.bright_cyan().bold();
            let healing_colored = healing.to_string().bright_green().bold();
            println!("  {}: {} total healing", fighter_colored, healing_colored);
        }
        
        // Spell statistics
        println!("\n{}", "Spells Cast:".bright_magenta().underline());
        for (fighter, spells) in &self.spells_cast {
            let fighter_colored = fighter.bright_cyan().bold();
            if spells.is_empty() {
                println!("  {}: No spells cast", fighter_colored);
            } else {
                let spell_count = spells.len();
                let unique_spells: std::collections::HashSet<_> = spells.iter().collect();
                let unique_count = unique_spells.len();
                
                println!("  {}: {} spells cast ({} unique)", 
                    fighter_colored, 
                    spell_count.to_string().bright_yellow(),
                    unique_count.to_string().bright_yellow()
                );
                
                // Show spell frequency
                let mut spell_freq: HashMap<String, usize> = HashMap::new();
                for spell in spells {
                    *spell_freq.entry(spell.clone()).or_insert(0) += 1;
                }
                
                for (spell, count) in spell_freq {
                    let spell_colored = spell.bright_magenta().italic();
                    let count_colored = count.to_string().bright_yellow();
                    println!("    • {} × {}", spell_colored, count_colored);
                }
            }
        }
        
        // Winner determination (based on current health)
        println!("\n{}", "🏆 BATTLE RESULT:".bright_yellow().bold());
        if self.fighter1_health > self.fighter2_health {
            println!("  {} wins with {} HP remaining!", 
                self.fighter1_name.bright_cyan().bold(),
                self.fighter1_health.to_string().bright_green().bold()
            );
            println!("  {} has {} HP remaining.", 
                self.fighter2_name.bright_red().bold(),
                self.fighter2_health.to_string().bright_red()
            );
        } else if self.fighter2_health > self.fighter1_health {
            println!("  {} wins with {} HP remaining!", 
                self.fighter2_name.bright_cyan().bold(),
                self.fighter2_health.to_string().bright_green().bold()
            );
            println!("  {} has {} HP remaining.", 
                self.fighter1_name.bright_red().bold(),
                self.fighter1_health.to_string().bright_red()
            );
        } else {
            println!("  {}It's a draw!{} Both fighters have {} HP remaining.", 
                "⚖️".bright_yellow(),
                "",
                self.fighter1_health.to_string().bright_yellow()
            );
        }
        
        println!("\n{}", "═".repeat(70).bright_black());
    }
}

/// Display a turn header with nice formatting
fn display_turn_header(turn: u32) {
    let header = format!(" TURN {} ", turn);
    let padding = "─".repeat((70 - header.len()) / 2);
    let line = format!("{}{}{}", padding, header.bright_white().bold(), padding);
    
    // If odd length, adjust
    let line = if line.len() < 70 {
        format!("{}{}", line, "─".repeat(70 - line.len()))
    } else {
        line
    };
    
    println!("\n{}", line.bright_blue());
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

/// Extension trait for centering colored strings
trait CenterColoredText {
    fn center(&self, width: usize) -> String;
}

impl CenterColoredText for colored::ColoredString {
    fn center(&self, width: usize) -> String {
        let text = self.to_string();
        center_text(&text, width)
    }
}

/// Simple function to display battle events without health tracking
pub fn display_battle_events(events: &[BattleEvent]) {
    let mut display = BattleDisplay {
        fighter1_name: "Fighter1".to_string(),
        fighter2_name: "Fighter2".to_string(),
        fighter1_health: 100,
        fighter2_health: 100,
        fighter1_max_health: 100,
        fighter2_max_health: 100,
        total_damage_dealt: HashMap::new(),
        total_healing_done: HashMap::new(),
        spells_cast: HashMap::new(),
    };
    
    // Simple display without health tracking
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
            display_turn_header_simple(turn);
        }

        for event in turn_events {
            display.display_single_event(event);
        }

        // Add spacing between turns (except after initiative)
        if turn != 0 {
            println!();
        }
    }

    // Display footer
    println!("{}", "═".repeat(60).bright_black());
}

/// Display a turn header with nice formatting (simple version)
fn display_turn_header_simple(turn: u32) {
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