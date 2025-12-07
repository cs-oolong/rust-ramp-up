use colored::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::thread;
use std::time::Duration;

/// Configuration for cassino display animations and styling
#[derive(Debug, Clone)]
pub struct CassinoDisplayConfig {
    pub enable_delays: bool,
    pub base_delay_ms: u64,
    pub use_spinners: bool,
    pub color_theme: ColorTheme,
}

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Yellow,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
        }
    }
}

impl Default for CassinoDisplayConfig {
    fn default() -> Self {
        Self {
            enable_delays: true,
            base_delay_ms: 300,
            use_spinners: true,
            color_theme: ColorTheme::default(),
        }
    }
}

/// Cassino display manager with animations and styling
pub struct CassinoDisplay {
    config: CassinoDisplayConfig,
    multi_progress: Option<MultiProgress>,
}

impl CassinoDisplay {
    pub fn new() -> Self {
        Self::with_config(CassinoDisplayConfig::default())
    }
    
    pub fn with_config(config: CassinoDisplayConfig) -> Self {
        let multi_progress = if config.use_spinners {
            Some(MultiProgress::new())
        } else {
            None
        };
        
        Self {
            config,
            multi_progress,
        }
    }
    
    /// Display welcome banner with casino theme
    pub fn show_welcome_banner(&self) {
        println!("{}", "═".repeat(60).color(self.config.color_theme.primary));
        
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.rainbow} {msg}")
                            .unwrap()
                    )
                    .with_message("🎰 Initializing Neopets Casino...")
            );
            
            for _ in 0..10 {
                pb.tick();
                thread::sleep(Duration::from_millis(100));
            }
            pb.finish_and_clear();
        }
        
        let welcome_text = "🎰 NEOPETS CASINO 🎰"
            .color(self.config.color_theme.primary)
            .bold();
        let centered_welcome = center_text(&welcome_text.to_string(), 60);
        println!("{}", centered_welcome);
        
        let subtitle = "🎲 Place your bets and test your luck! 🎲"
            .color(self.config.color_theme.secondary)
            .italic();
        let centered_subtitle = center_text(&subtitle.to_string(), 60);
        println!("{}", centered_subtitle);
        
        println!("{}", "═".repeat(60).color(self.config.color_theme.primary));
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(500));
        }
    }
    
    /// Display event creation with animation
    pub fn show_event_creation(&self) {
        println!();
        
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.yellow} {msg}")
                            .unwrap()
                    )
                    .with_message("📝 Creating new event...")
            );
            
            for _ in 0..8 {
                pb.tick();
                thread::sleep(Duration::from_millis(150));
            }
            pb.finish_and_clear();
        }
        
        println!("{}", "🎲 CREATING NEW EVENT 🎲".color(self.config.color_theme.secondary).bold());
        println!("{}", "─".repeat(40).color(self.config.color_theme.secondary));
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(300));
        }
    }
    
    /// Display successful event creation
    pub fn show_event_success(&self, event_id: &str, description: &str, odd: f64) {
        println!();
        println!("{}", "✅ EVENT CREATED SUCCESSFULLY!".color(self.config.color_theme.success).bold());
        
        let event_card = format!(
            "┌──────────────────────────────────────┐\n\
             │ Event ID: {:<28} │\n\
             │ Description: {:<25} │\n\
             │ Odds: {:.2}x {:<23} │\n\
             └──────────────────────────────────────┘",
            event_id, description, odd, ""
        );
        
        println!("{}", event_card.color(self.config.color_theme.info));
        
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    )
                    .with_message("Event saved to casino database...")
            );
            
            for _ in 0..5 {
                pb.tick();
                thread::sleep(Duration::from_millis(100));
            }
            pb.finish_and_clear();
        }
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(300));
        }
    }
    
    /// Display bet placement animation
    pub fn show_bet_placement(&self, event_id: &str, amount: f64, potential_win: f64, odd: f64, is_accumulated: bool) {
        println!();
        
        let bet_type = if is_accumulated { "ACCUMULATED BET" } else { "SINGLE BET" };
        let bet_icon = if is_accumulated { "🎯" } else { "💰" };
        
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap()
                    )
                    .with_message(format!("{} Processing bet...", bet_icon))
            );
            
            for _ in 0..10 {
                pb.tick();
                thread::sleep(Duration::from_millis(80));
            }
            pb.finish_and_clear();
        }
        
        println!("{}", format!("{} {} PLACED SUCCESSFULLY! {}", bet_icon, bet_type, bet_icon)
            .color(self.config.color_theme.success).bold());
        
        // Display bet details in a card format
        let bet_card = format!(
            "┌──────────────────────────────────────┐\n\
             │ Event: {:<29} │\n\
             │ Bet Amount: ${:<24.2} │\n\
             │ Potential Win: ${:<21.2} │\n\
             │ Odds: {:.2}x {:<23} │\n\
             └──────────────────────────────────────┘",
            event_id, amount, potential_win, odd, ""
        );
        
        println!("{}", bet_card.color(self.config.color_theme.info));
        
        // Add some celebration animation
        if self.config.use_spinners {
            let celebration_pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.rainbow} {msg}")
                            .unwrap()
                    )
                    .with_message("🎉 Bet registered in casino system...")
            );
            
            for _ in 0..6 {
                celebration_pb.tick();
                thread::sleep(Duration::from_millis(120));
            }
            celebration_pb.finish_and_clear();
        }
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(400));
        }
    }
    
    /// Display available events in a beautiful table format
    pub fn show_events_list(&self, events: &std::collections::HashMap<String, crate::cassino::CassinoEvent>) {
        
        if events.is_empty() {
            println!();
            println!("{}", "⚠️  NO EVENTS AVAILABLE".color(self.config.color_theme.warning).bold());
            println!("{}", "Create some events first with 'cassino event'".color(self.config.color_theme.info));
            return;
        }
        
        println!();
        println!("{}", "🎲 AVAILABLE EVENTS 🎲".color(self.config.color_theme.primary).bold());
        println!("{}", "═".repeat(60).color(self.config.color_theme.primary));
        
        let mut event_count = 0;
        for (event_id, event) in events {
            event_count += 1;
            
            // Color coding based on odds (higher odds = more rare = different color)
            let odds_color = if event.odd >= 5.0 {
                &self.config.color_theme.warning // High odds (rare events)
            } else if event.odd >= 2.0 {
                &self.config.color_theme.secondary // Medium odds
            } else {
                &self.config.color_theme.success // Low odds (likely events)
            };
            
            let event_box = format!(
                "┌────────────────────────────────────────────────┐\n\
                 │ Event ID: {:<36} │\n\
                 │ Description: {:<33} │\n\
                 │ Odds: {:<5.2}x {:<30} │\n\
                 └────────────────────────────────────────────────┘",
                event_id, event.description, event.odd, ""
            );
            
            println!("{}", event_box.color(*odds_color));
            
            if self.config.enable_delays && event_count < events.len() {
                thread::sleep(Duration::from_millis(200));
            }
        }
        
        println!("{}", "═".repeat(60).color(self.config.color_theme.primary));
        println!("{}", format!("📊 Total Events: {}", events.len()).color(self.config.color_theme.info));
    }
    
    /// Display error message with style
    pub fn show_error(&self, message: &str) {
        println!();
        println!("{}", "❌ ERROR".color(self.config.color_theme.error).bold());
        println!("{}", "─".repeat(30).color(self.config.color_theme.error));
        println!("{}", message.color(self.config.color_theme.error));
        println!("{}", "─".repeat(30).color(self.config.color_theme.error));
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(300));
        }
    }
    
    /// Display info message with style
    pub fn show_info(&self, message: &str) {
        println!();
        println!("{}", "ℹ️  INFO".color(self.config.color_theme.info).bold());
        println!("{}", message.color(self.config.color_theme.info));
        
        if self.config.enable_delays {
            thread::sleep(Duration::from_millis(200));
        }
    }
    
    /// Display loading animation for data operations
    pub fn show_loading_animation(&self, message: &str) {
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap()
                    )
                    .with_message(message.to_string())
            );
            
            for _ in 0..8 {
                pb.tick();
                thread::sleep(Duration::from_millis(100));
            }
            pb.finish_and_clear();
        } else {
            println!("{}", message.color(self.config.color_theme.info));
        }
    }
    
    /// Display success animation
    pub fn show_success_animation(&self, message: &str) {
        if self.config.use_spinners {
            let pb = self.multi_progress.as_ref().unwrap().add(
                ProgressBar::new_spinner()
                    .with_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    )
                    .with_message(message.to_string())
            );
            
            for _ in 0..5 {
                pb.tick();
                thread::sleep(Duration::from_millis(100));
            }
            pb.finish_and_clear();
        }
        
        println!("{}", message.color(self.config.color_theme.success));
    }
}

/// Helper function to center text
fn center_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len >= width {
        return text.to_string();
    }
    
    let padding = (width - text_len) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

