use crate::battle::BattleEvent;
use crate::neopets::Neopet;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Configuration for battle display animations and timing
#[derive(Debug, Clone)]
pub struct BattleDisplayConfig {
    pub enable_delays: bool,
    pub base_delay_ms: u64,
    pub critical_delay_ms: u64,
    pub spell_delay_ms: u64,
    pub use_spinners: bool,
    pub streaming_effect: bool,
}

impl Default for BattleDisplayConfig {
    fn default() -> Self {
        Self {
            enable_delays: true,
            base_delay_ms: 600,      // Increased from 300ms
            critical_delay_ms: 1200, // Increased from 800ms
            spell_delay_ms: 800,     // Increased from 500ms
            use_spinners: true,
            streaming_effect: true,
        }
    }
}

/// Purely presentational battle display with suspenseful animations
pub struct BattleDisplay {
    fighter1_name: String,
    fighter2_name: String,
    fighter1_max_health: u32,
    fighter2_max_health: u32,
    config: BattleDisplayConfig,
    multi_progress: Option<MultiProgress>,
}

impl BattleDisplay {
    pub fn with_config(fighter1: &Neopet, fighter2: &Neopet, config: BattleDisplayConfig) -> Self {
        Self {
            fighter1_name: fighter1.name.clone(),
            fighter2_name: fighter2.name.clone(),
            fighter1_max_health: fighter1.health,
            fighter2_max_health: fighter2.health,
            config: config.clone(),
            multi_progress: if config.use_spinners || config.streaming_effect {
                Some(MultiProgress::new())
            } else {
                None
            },
        }
    }
    
    /// Add suspenseful delay with optional spinner
    fn suspenseful_delay(&self, duration_ms: u64, message: &str, use_spinner: bool) {
        if !self.config.enable_delays {
            return;
        }
        
        // Use the configured base delay if it's shorter than requested duration
        let actual_duration = if duration_ms > self.config.base_delay_ms {
            duration_ms
        } else {
            self.config.base_delay_ms
        };
        
        if use_spinner && self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap()
                    )
                    .with_message(message.to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = (actual_duration / 100) as u32;
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            // Simple delay without spinner
            thread::sleep(Duration::from_millis(actual_duration));
        }
    }
    
    /// Create a dramatic entrance effect with spinner
    fn dramatic_entrance(&self) {
        if !self.config.enable_delays {
            return;
        }
        
        println!();
        
        if self.config.use_spinners {
            // Spinner approach instead of typewriter
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.yellow} {msg}")
                            .unwrap()
                    )
                    .with_message("⚔️  BATTLE PREPARING ⚔️".bright_yellow().bold().to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = 25; // Show spinner for ~2.5 seconds
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            // Simple display without spinner
            println!("{}", "⚔️  BATTLE PREPARING ⚔️".bright_yellow().bold());
        }
        
        // Dramatic pause
        self.suspenseful_delay(500, "Fighters taking positions...", true);
    }
    
    /// Display battle events with suspenseful animations and streaming effects
    pub fn display_battle_events(&self, events: &[BattleEvent], health_state: Option<(u32, u32)>) {
        if events.is_empty() {
            println!("{}", "No battle events to display.".dimmed());
            return;
        }

        // Dramatic entrance
        self.dramatic_entrance();

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

        // Display header with animation
        self.animate_header();
        
        // Display initial health bars if health state is provided
        if let Some((hp1, hp2)) = health_state {
            println!("\n{}", "Initial Status:".bright_white().bold());
            self.display_health_bars_with_effect(hp1, hp2);
        }
        
        println!("{}", "═".repeat(70).bright_black());

        // Display events grouped by turn with streaming effects
        for turn in turns {
            let turn_events = &events_by_turn[&turn];
            
            if turn == 0 {
                // Initiative phase
                self.animate_initiative_phase();
            } else {
                self.animate_turn_header(turn);
            }

            // Display events with spinner suspense (no streaming text)
            for (i, event) in turn_events.iter().enumerate() {
                self.display_event_with_spinner(event, i == 0);
                
                // Small delay between events in the same turn
                if i < turn_events.len() - 1 {
                    thread::sleep(Duration::from_millis(500)); // Increased from 300ms // Increased from 150ms
                }
            }

            // Add spacing between turns (except after initiative)
            if turn != 0 {
                println!();
                
                // Dramatic pause between turns
                if self.config.streaming_effect {
                    self.suspenseful_delay(400, "Preparing next turn...", false);
                }
            }
        }

        // Display footer with animation
        self.animate_footer();
    }
    
    /// Animate the battle header with spinner (no streaming text)
    fn animate_header(&self) {
        println!("{}", "═".repeat(70).bright_black());
        
        if self.config.use_spinners {
            // Spinner approach for battle header
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.yellow} {msg}")
                            .unwrap()
                    )
                    .with_message("⚔️  BATTLE BEGINS ⚔️".bright_yellow().bold().to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = 20; // Show spinner for ~2.0 seconds
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            // Simple display without spinner
            let battle_header = "⚔️  BATTLE BEGINS ⚔️".bright_yellow().bold();
            let centered_header = center_text(&battle_header.to_string(), 70);
            println!("{}", centered_header);
        }
        
        println!("{}", "═".repeat(70).bright_black());
    }
    
    /// Animate initiative phase with spinner
    fn animate_initiative_phase(&self) {
        println!("\n{}", "🏁 INITIATIVE PHASE".bright_cyan().bold());
        
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap()
                    )
                    .with_message("Rolling for initiative...".to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = 12; // Show spinner for ~1.2 seconds
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            self.suspenseful_delay(300, "Rolling for initiative...", true);
        }
    }
    
    /// Animate turn header with spinner (no streaming text)
    fn animate_turn_header(&self, turn: u32) {
        let header = format!(" TURN {} ", turn);
        let padding = "─".repeat((70 - header.len()) / 2);
        
        if self.config.use_spinners {
            // Spinner approach for turn header
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.blue} {msg}")
                            .unwrap()
                    )
                    .with_message(format!("Preparing Turn {}...", turn))
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = 8; // Show spinner for ~0.8 seconds
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        }

        // Print the complete header instantly
        let line = format!("{}{}{}", padding, header.bright_white().bold(), padding);
        let line = if line.len() < 70 {
            format!("{}{}", line, "─".repeat(70 - line.len()))
        } else {
            line
        };
        println!("\n{}", line.bright_blue());
    }
    
    /// Display a single event with spinner suspense (no streaming text)
    fn display_event_with_spinner(&self, event: &BattleEvent, is_first: bool) {
        match event {
            BattleEvent::Roll { actor, dice, final_value, is_positive_crit, is_negative_crit, goal, .. } => {
                self.display_roll_with_spinner(actor, *dice, *final_value, *is_positive_crit, *is_negative_crit, goal, is_first);
            }
            BattleEvent::Attack { actor, target, actual_damage, .. } => {
                self.display_attack_with_spinner(actor, target, *actual_damage);
            }
            BattleEvent::Heal { actor, amount, .. } => {
                self.display_heal_with_spinner(actor, *amount);
            }
            BattleEvent::SpellCast { actor, target, spell_name, .. } => {
                self.display_spell_with_spinner(actor, target, spell_name);
            }
        }
    }
    
    /// Display dice roll event with spinner suspense (no streaming text)
    fn display_roll_with_spinner(&self, actor: &str, dice: u8, final_value: u32, is_positive_crit: bool, is_negative_crit: bool, goal: &str, is_first: bool) {
        if !is_first {
            thread::sleep(Duration::from_millis(400)); // Increased from 200ms
        }
        
        let goal_icon = match goal {
            "attack" => "⚔️",
            "defense" => "🛡️",
            "heal" => "💚",
            "initiative" => "🎲",
            _ => "🎲",
        };

        // Determine spinner message based on goal
        let spinner_msg = match goal {
            "attack" => "Rolling attack dice...",
            "defense" => "Rolling defense dice...",
            "heal" => "Rolling heal dice...",
            "initiative" => "Rolling initiative...",
            _ => "Rolling dice...",
        };

        // Show spinner for suspense
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap()
                    )
                    .with_message(spinner_msg.to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            // Show spinner for different durations based on critical hits
            let spin_duration = if is_positive_crit || is_negative_crit {
                self.config.critical_delay_ms / 2
            } else {
                self.config.base_delay_ms / 2
            };
            
            let steps = (spin_duration / 100) as u32;
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            // Simple delay without spinner
            let delay = if is_positive_crit || is_negative_crit {
                self.config.critical_delay_ms / 2
            } else {
                self.config.base_delay_ms / 2
            };
            thread::sleep(Duration::from_millis(delay));
        }

        // Now print the complete event instantly
        let dice_display = if is_positive_crit {
            format!("{}", dice).on_bright_yellow().red().bold()
        } else if is_negative_crit {
            format!("{}", dice).on_red().white().bold()
        } else {
            dice.to_string().normal()
        };

        println!("  {} {} rolls {} for {}: {} = {}", 
            goal_icon,
            actor.bright_cyan(),
            goal.bright_white(),
            dice_display,
            "🎯".bright_yellow(),
            if is_positive_crit || is_negative_crit {
                final_value.to_string().bright_yellow().bold()
            } else {
                final_value.to_string().normal()
            }
        );

        // Critical hit announcement
        if is_positive_crit {
            println!("     {}", "⭐ NATURAL 20! Critical Success! ⭐".bright_yellow().bold());
        } else if is_negative_crit {
            println!("     {}", "💥 NATURAL 1! Critical Failure! 💥".bright_red().bold());
        }
    }
    
    /// Display attack event with spinner suspense (no streaming text)
    fn display_attack_with_spinner(&self, actor: &str, target: &str, actual_damage: u32) {
        thread::sleep(Duration::from_millis(500)); // Increased from 300ms
        
        // Show spinner for suspense
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.red} {msg}")
                            .unwrap()
                    )
                    .with_message("Preparing attack...".to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = ((self.config.base_delay_ms * 3/4) / 100) as u32; // 75% of base delay
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            thread::sleep(Duration::from_millis(self.config.base_delay_ms));
        }

        // Now print the complete event instantly
        let actor_colored = actor.bright_blue().bold();
        let target_colored = target.bright_red().bold();

        if actual_damage == 0 {
            println!("  ⚔️  {} attacks {} but the attack is {}", 
                actor_colored,
                target_colored,
                "BLOCKED!".bright_white().on_red()
            );
        } else {
            println!("  ⚔️  {} hits {} for {} damage", 
                actor_colored,
                target_colored,
                actual_damage.to_string().bright_red().bold()
            );
        }
    }
    
    /// Display healing event with spinner suspense (no streaming text)
    fn display_heal_with_spinner(&self, actor: &str, amount: u32) {
        thread::sleep(Duration::from_millis(500)); // Increased from 300ms
        
        // Show spinner for suspense
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    )
                    .with_message("Channeling healing energy...".to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = ((self.config.base_delay_ms * 3/4) / 100) as u32; // 75% of base delay
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            thread::sleep(Duration::from_millis(self.config.base_delay_ms));
        }

        // Now print the complete event instantly
        println!("  💚 {} heals for {} HP", 
            actor.bright_green().bold(),
            amount.to_string().bright_green().bold()
        );
    }
    
    /// Display spell casting event with spinner suspense (no streaming text)
    fn display_spell_with_spinner(&self, actor: &str, target: &str, spell_name: &str) {
        // Show spinner for suspense
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.magenta} {msg}")
                            .unwrap()
                    )
                    .with_message("Casting spell...".to_string())
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            
            let steps = ((self.config.spell_delay_ms * 3/4) / 100) as u32; // 75% of spell delay
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            thread::sleep(Duration::from_millis(self.config.spell_delay_ms));
        }

        // Now print the complete event instantly
        println!("  ✨ {} casts {} on {}", 
            actor.bright_magenta().bold(),
            spell_name.bright_yellow().italic(),
            target.bright_red().bold()
        );
    }
    
    /// Display health bars (no streaming animation)
    fn display_health_bars_with_effect(&self, fighter1_hp: u32, fighter2_hp: u32) {
        println!();
        
        // Simple delay for suspense, then show health bars instantly
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(200));
        }
        
        self.display_health_bars(fighter1_hp, fighter2_hp);
        println!();
    }
    
    /// Display battle summary with dramatic effect
    pub fn display_battle_summary(&self, events: &[BattleEvent]) {
        if self.config.streaming_effect {
            self.suspenseful_delay(500, "Calculating battle results...", true);
        }
        
        println!("\n{}", "🏁 BATTLE COMPLETE 🏁".bright_green().bold().center(70));
        println!("{}", "═".repeat(70).bright_black());
        
        if self.config.streaming_effect {
            self.suspenseful_delay(300, "Analyzing statistics...", true);
        }
        
        // Calculate statistics from events
        let mut total_damage_dealt: HashMap<String, u32> = HashMap::new();
        let mut total_healing_done: HashMap<String, u32> = HashMap::new();
        let mut spells_cast: HashMap<String, Vec<String>> = HashMap::new();
        
        for event in events {
            match event {
                BattleEvent::Attack { actor, actual_damage, .. } => {
                    *total_damage_dealt.entry(actor.clone()).or_insert(0) += actual_damage;
                }
                BattleEvent::Heal { actor, amount, .. } => {
                    *total_healing_done.entry(actor.clone()).or_insert(0) += amount;
                }
                BattleEvent::SpellCast { actor, spell_name, .. } => {
                    spells_cast.entry(actor.clone()).or_insert_with(Vec::new).push(spell_name.clone());
                }
                _ => {}
            }
        }
        
        println!("\n{}", "📊 BATTLE SUMMARY".bright_white().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        // Damage statistics
        if !total_damage_dealt.is_empty() {
            println!("\n{}", "Damage Dealt:".bright_red().underline());
            for (fighter, damage) in &total_damage_dealt {
                let fighter_colored = fighter.bright_cyan().bold();
                let damage_colored = damage.to_string().bright_red().bold();
                println!("  {}: {} total damage", fighter_colored, damage_colored);
            }
        }
        
        // Healing statistics
        if !total_healing_done.is_empty() {
            println!("\n{}", "Healing Done:".bright_green().underline());
            for (fighter, healing) in &total_healing_done {
                let fighter_colored = fighter.bright_cyan().bold();
                let healing_colored = healing.to_string().bright_green().bold();
                println!("  {}: {} total healing", fighter_colored, healing_colored);
            }
        }
        
        // Spell statistics
        if !spells_cast.is_empty() {
            println!("\n{}", "Spells Cast:".bright_magenta().underline());
            for (fighter, spells) in &spells_cast {
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
        }
        
        println!("\n{}", "═".repeat(70).bright_black());
    }
    
    /// Animate footer
    fn animate_footer(&self) {
        if self.config.streaming_effect {
            self.suspenseful_delay(400, "Finalizing results...", true);
        }
        
        println!("{}", "═".repeat(70).bright_black());
    }
    
    /// Display health bars
    pub fn display_health_bars(&self, fighter1_hp: u32, fighter2_hp: u32) {
        let display = BattleDisplay {
            fighter1_name: self.fighter1_name.clone(),
            fighter2_name: self.fighter2_name.clone(),
            fighter1_max_health: self.fighter1_max_health,
            fighter2_max_health: self.fighter2_max_health,
            config: BattleDisplayConfig::default(),
            multi_progress: None,
        };
        display.display_health_bars_internal(fighter1_hp, fighter2_hp);
    }
    
    fn display_health_bars_internal(&self, fighter1_hp: u32, fighter2_hp: u32) {
        println!();
        self.display_single_health_bar(&self.fighter1_name, fighter1_hp, self.fighter1_max_health);
        self.display_single_health_bar(&self.fighter2_name, fighter2_hp, self.fighter2_max_health);
        println!();
    }
    
    /// Display a single health bar
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neopets::{Neopet, Behavior};

    #[test]
    fn test_display_empty_events() {
        let display = BattleDisplay {
            fighter1_name: "Fighter1".to_string(),
            fighter2_name: "Fighter2".to_string(),
            fighter1_max_health: 100,
            fighter2_max_health: 100,
            config: BattleDisplayConfig::default(),
            multi_progress: None,
        };
        display.display_battle_events(&[], None);
    }

    #[test]
    fn test_display_with_health_state() {
        let mut config = BattleDisplayConfig::default();
        config.enable_delays = false; // Disable delays for testing
        config.use_spinners = false;
        config.streaming_effect = false;
        
        let display = BattleDisplay::with_config(
            &Neopet {
                name: "Pikachu".to_string(),
                health: 100,
                heal_delta: 10,
                base_attack: 5,
                base_defense: 3,
                spells: vec![],
                behavior: Behavior {
                    attack_chance: 0.5,
                    spell_chances: vec![],
                    heal_chance: 0.5,
                },
            },
            &Neopet {
                name: "Charizard".to_string(),
                health: 120,
                heal_delta: 15,
                base_attack: 8,
                base_defense: 5,
                spells: vec![],
                behavior: Behavior {
                    attack_chance: 0.4,
                    spell_chances: vec![],
                    heal_chance: 0.6,
                },
            },
            config
        );
        
        let events = vec![BattleEvent::Roll {
            turn: 1,
            actor: "Pikachu".to_string(),
            dice: 15,
            final_value: 25,
            is_positive_crit: false,
            is_negative_crit: false,
            goal: "attack".to_string(),
        }];
        
        // Display with health state (current HP)
        display.display_battle_events(&events, Some((85, 120)));
    }

    #[test]
    fn test_battle_state() {
        
        let fighter1 = Neopet {
            name: "TestFighter1".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.5,
            },
        };
        
        let fighter2 = Neopet {
            name: "TestFighter2".to_string(),
            health: 80,
            heal_delta: 15,
            base_attack: 8,
            base_defense: 5,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.4,
                spell_chances: vec![],
                heal_chance: 0.6,
            },
        };
        
        // Test health bar display directly
        let display = BattleDisplay::with_config(&fighter1, &fighter2, BattleDisplayConfig::default());
        display.display_health_bars(75, 60);
    }
    
    #[test]
    fn test_config_options() {
        let mut config = BattleDisplayConfig::default();
        config.enable_delays = false; // Disable delays for testing
        config.use_spinners = false;
        config.streaming_effect = false;
        
        let fighter1 = Neopet {
            name: "Fighter1".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.5,
            },
        };
        
        let fighter2 = Neopet {
            name: "Fighter2".to_string(),
            health: 100,
            heal_delta: 10,
            base_attack: 5,
            base_defense: 3,
            spells: vec![],
            behavior: Behavior {
                attack_chance: 0.5,
                spell_chances: vec![],
                heal_chance: 0.5,
            },
        };
        
        let display = BattleDisplay::with_config(&fighter1, &fighter2, config);
        let events = vec![BattleEvent::Heal {
            turn: 1,
            actor: "Fighter1".to_string(),
            amount: 10,
        }];
        
        display.display_battle_events(&events, None);
    }
}