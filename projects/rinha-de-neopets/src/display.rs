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
    pub health_bar_updates: bool,
}

impl Default for BattleDisplayConfig {
    fn default() -> Self {
        Self {
            enable_delays: true,
            base_delay_ms: 300,
            critical_delay_ms: 800,
            spell_delay_ms: 500,
            use_spinners: true,
            streaming_effect: true,
            health_bar_updates: true,
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
    pub fn new(fighter1: &Neopet, fighter2: &Neopet) -> Self {
        Self {
            fighter1_name: fighter1.name.clone(),
            fighter2_name: fighter2.name.clone(),
            fighter1_max_health: fighter1.health,
            fighter2_max_health: fighter2.health,
            config: BattleDisplayConfig::default(),
            multi_progress: None,
        }
    }
    
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
            
            let steps = (duration_ms / 100) as u32;
            for i in 0..steps {
                pb.set_position(i as u64);
                thread::sleep(Duration::from_millis(100));
            }
            
            pb.finish_and_clear();
        } else {
            // Simple delay without spinner
            thread::sleep(Duration::from_millis(duration_ms));
        }
    }
    
    /// Create a dramatic entrance effect
    fn dramatic_entrance(&self) {
        if !self.config.enable_delays {
            return;
        }
        
        println!();
        let entrance_text = "⚔️  BATTLE PREPARING ⚔️";
        
        // Typewriter effect
        for (_i, ch) in entrance_text.chars().enumerate() {
            print!("{}", ch.to_string().bright_yellow().bold());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(80));
        }
        println!();
        
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

            // Stream events with delays and effects
            for (i, event) in turn_events.iter().enumerate() {
                self.stream_event(event, i == 0);
                
                // Small delay between events in the same turn
                if i < turn_events.len() - 1 {
                    thread::sleep(Duration::from_millis(150));
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
    
    /// Animate the battle header
    fn animate_header(&self) {
        println!("{}", "═".repeat(70).bright_black());
        
        if self.config.streaming_effect {
            let battle_header = "⚔️  BATTLE BEGINS ⚔️";
            print!("  ");
            for ch in battle_header.chars() {
                print!("{}", ch.to_string().bright_yellow().bold());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                thread::sleep(Duration::from_millis(60));
            }
            println!();
        } else {
            let battle_header = "⚔️  BATTLE BEGINS ⚔️".bright_yellow().bold();
            let centered_header = center_text(&battle_header.to_string(), 70);
            println!("{}", centered_header);
        }
        
        println!("{}", "═".repeat(70).bright_black());
    }
    
    /// Animate initiative phase
    fn animate_initiative_phase(&self) {
        println!("\n{}", "🏁 INITIATIVE PHASE".bright_cyan().bold());
        
        if self.config.streaming_effect {
            self.suspenseful_delay(300, "Rolling for initiative...", true);
        }
    }
    
    /// Animate turn header with suspense
    fn animate_turn_header(&self, turn: u32) {
        let header = format!(" TURN {} ", turn);
        let padding = "─".repeat((70 - header.len()) / 2);
        
        if self.config.streaming_effect {
            print!("\n  ");
            for ch in padding.chars() {
                print!("{}", ch.to_string().bright_blue());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                thread::sleep(Duration::from_millis(20));
            }
            
            print!("{}", header.bright_white().bold());
            for ch in padding.chars() {
                print!("{}", ch.to_string().bright_blue());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                thread::sleep(Duration::from_millis(20));
            }
            println!();
        } else {
            let line = format!("{}{}{}", padding, header.bright_white().bold(), padding);
            let line = if line.len() < 70 {
                format!("{}{}", line, "─".repeat(70 - line.len()))
            } else {
                line
            };
            println!("\n{}", line.bright_blue());
        }
    }
    
    /// Stream a single event with dramatic effect
    fn stream_event(&self, event: &BattleEvent, is_first: bool) {
        match event {
            BattleEvent::Roll { actor, dice, final_value, is_positive_crit, is_negative_crit, goal, .. } => {
                self.stream_roll_event(actor, *dice, *final_value, *is_positive_crit, *is_negative_crit, goal, is_first);
            }
            BattleEvent::Attack { actor, target, actual_damage, .. } => {
                self.stream_attack_event(actor, target, *actual_damage);
            }
            BattleEvent::Heal { actor, amount, .. } => {
                self.stream_heal_event(actor, *amount);
            }
            BattleEvent::SpellCast { actor, target, spell_name, .. } => {
                self.stream_spell_event(actor, target, spell_name);
            }
        }
    }
    
    /// Stream dice roll event with suspense
    fn stream_roll_event(&self, actor: &str, dice: u8, final_value: u32, is_positive_crit: bool, is_negative_crit: bool, goal: &str, is_first: bool) {
        if !is_first {
            thread::sleep(Duration::from_millis(200));
        }
        
        let goal_icon = match goal {
            "attack" => "⚔️",
            "defense" => "🛡️",
            "heal" => "💚",
            "initiative" => "🎲",
            _ => "🎲",
        };

        print!("  {} ", goal_icon);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
        
        print!("{}", actor.bright_cyan());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
        
        print!(" rolls {} for ", goal.bright_white());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
        
        // Dramatic dice reveal
        let dice_display = if is_positive_crit {
            format!("{}", dice).on_bright_yellow().red().bold()
        } else if is_negative_crit {
            format!("{}", dice).on_red().white().bold()
        } else {
            dice.to_string().normal()
        };
        
        print!("{}", dice_display);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        // Extra suspense for critical hits
        if is_positive_crit || is_negative_crit {
            thread::sleep(Duration::from_millis(400));
            print!(" {} ", "🎯".bright_yellow());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(200));
            println!("= {}", final_value.to_string().bright_yellow().bold());
            
            // Critical hit announcement
            if is_positive_crit {
                thread::sleep(Duration::from_millis(300));
                println!("     {}", "⭐ NATURAL 20! Critical Success! ⭐".bright_yellow().bold());
            } else if is_negative_crit {
                thread::sleep(Duration::from_millis(300));
                println!("     {}", "💥 NATURAL 1! Critical Failure! 💥".bright_red().bold());
            }
        } else {
            thread::sleep(Duration::from_millis(200));
            print!(" {} ", "🎯".bright_yellow());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(150));
            println!("= {}", final_value.to_string().normal());
        }
    }
    
    /// Stream attack event with impact
    fn stream_attack_event(&self, actor: &str, target: &str, actual_damage: u32) {
        thread::sleep(Duration::from_millis(300));
        
        print!("  ⚔️  ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
        
        print!("{}", actor.bright_blue().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        if actual_damage == 0 {
            thread::sleep(Duration::from_millis(200));
            print!(" attacks {} but the attack is ", target.bright_red().bold());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(300));
            println!("{}", "BLOCKED!".bright_white().on_red());
        } else {
            thread::sleep(Duration::from_millis(200));
            print!(" hits {} for ", target.bright_red().bold());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(200));
            println!("{} damage", actual_damage.to_string().bright_red().bold());
        }
    }
    
    /// Stream healing event with warmth
    fn stream_heal_event(&self, actor: &str, amount: u32) {
        thread::sleep(Duration::from_millis(300));
        
        print!("  💚 ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        print!("{}", actor.bright_green().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        print!(" heals for ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        println!("{} HP", amount.to_string().bright_green().bold());
    }
    
    /// Stream spell casting event with magic
    fn stream_spell_event(&self, actor: &str, target: &str, spell_name: &str) {
        if self.config.streaming_effect {
            self.suspenseful_delay(self.config.spell_delay_ms, "Casting spell...", true);
        } else {
            thread::sleep(Duration::from_millis(300));
        }
        
        print!("  ✨ ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(200));
        
        print!("{}", actor.bright_magenta().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        print!(" casts ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        print!("{}", spell_name.bright_yellow().italic());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(150));
        
        println!(" on {}", target.bright_red().bold());
    }
    
    /// Display health bars with animation
    fn display_health_bars_with_effect(&self, fighter1_hp: u32, fighter2_hp: u32) {
        println!();
        
        if self.config.streaming_effect {
            // Animate health bars appearing
            print!("  ");
            for i in 0..10 {
                print!("{}", "█".repeat(i).bright_red());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                thread::sleep(Duration::from_millis(30));
                if i < 9 {
                    print!("\r  ");
                }
            }
            print!("\r");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
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
    
    /// Display a single battle event with appropriate styling
    fn display_single_event(&self, event: &BattleEvent) {
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
}

/// Simple function to display battle events without animations (for testing/comparison)
pub fn display_battle_events_simple(events: &[BattleEvent]) {
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
            display_single_event_simple(event);
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

/// Display a single battle event (simple version)
fn display_single_event_simple(event: &BattleEvent) {
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

/// Battle state that can be passed to display for showing current status
#[derive(Debug, Clone)]
pub struct BattleState {
    pub fighter1_name: String,
    pub fighter2_name: String,
    pub fighter1_current_hp: u32,
    pub fighter2_current_hp: u32,
    pub fighter1_max_hp: u32,
    pub fighter2_max_hp: u32,
    pub current_turn: u32,
}

impl BattleState {
    pub fn new(fighter1: &Neopet, fighter2: &Neopet) -> Self {
        Self {
            fighter1_name: fighter1.name.clone(),
            fighter2_name: fighter2.name.clone(),
            fighter1_current_hp: fighter1.health,
            fighter2_current_hp: fighter2.health,
            fighter1_max_hp: fighter1.health,
            fighter2_max_hp: fighter2.health,
            current_turn: 0,
        }
    }
    
    pub fn display_current_status(&self) {
        println!("\n{}", format!(" Turn {} Status ", self.current_turn).bright_white().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        let display = BattleDisplay {
            fighter1_name: self.fighter1_name.clone(),
            fighter2_name: self.fighter2_name.clone(),
            fighter1_max_health: self.fighter1_max_hp,
            fighter2_max_health: self.fighter2_max_hp,
            config: BattleDisplayConfig::default(),
            multi_progress: None,
        };
        
        display.display_health_bars_internal(self.fighter1_current_hp, self.fighter2_current_hp);
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
        
        let mut state = BattleState::new(&fighter1, &fighter2);
        state.current_turn = 5;
        state.fighter1_current_hp = 75;
        state.fighter2_current_hp = 60;
        state.display_current_status();
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