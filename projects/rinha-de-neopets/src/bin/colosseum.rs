use clap::{Parser, Subcommand};
use dialoguer::{Input, Select, Confirm};

#[derive(Parser)]
#[command(name = "colosseum")]
#[command(about = "Battle Arena Management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage fighters
    Fighter {
        #[command(subcommand)]
        action: FighterAction,
    },
    /// Manage battles
    Battle {
        #[command(subcommand)]
        action: BattleAction,
    },
    /// Clean up battles (remove all saved battles)
    Clean,
}

#[derive(Subcommand)]
enum FighterAction {
    /// Create a new fighter interactively
    Create,
    /// List all fighter names
    List,
    /// Show detailed fighter information
    Show { name: String },
}

#[derive(Subcommand)]
enum BattleAction {
    /// Create a battle between two fighters
    Create {
        fighter1: String,
        fighter2: String,
        /// Watch the battle live immediately
        #[arg(short, long)]
        watch: bool,
        /// Save the battle without watching
        #[arg(short, long)]
        save: bool,
    },
    /// Create N random battles between available fighters
    Random {
        count: usize,
    },
    /// List all saved battles
    List,
    /// Watch a saved battle
    Watch { id: String },
}

fn placeholder() {
    println!("to be implemented");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fighter { action } => match action {
            FighterAction::Create => placeholder(),
            FighterAction::List => placeholder(),
            FighterAction::Show { name } => placeholder(),
        },
        Commands::Battle { action } => match action {
            BattleAction::Create { fighter1, fighter2, watch, save } => placeholder(),
            BattleAction::Random { count } => placeholder(),
            BattleAction::List => placeholder(),
            BattleAction::Watch { id } => placeholder(),
        }
        Commands::Clean => placeholder(),
    }
}