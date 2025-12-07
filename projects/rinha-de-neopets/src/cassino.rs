use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CassinoEvent {
    pub description: String,
    pub odd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletedEvent {
    pub event_id: String,
    pub description: String,
    pub odd: f64,
    pub result: bool, // true if event occurred, false otherwise
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpiredBet {
    pub event_id: String,
    pub amount: f64,
    pub potential_win: f64,
    pub result: bool, // true if bet won, false if lost
    pub actual_payout: f64, // 0 if lost, potential_win if won
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpiredAccumulatedBet {
    pub event_ids: Vec<String>,
    pub amount: f64,
    pub combined_odds: f64,
    pub potential_win: f64,
    pub all_events_occurred: bool, // true if all events occurred, false otherwise
    pub actual_payout: f64, // 0 if lost, potential_win if won
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DoneEvents {
    pub completed_events: Vec<CompletedEvent>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExpiredBets {
    pub expired_bets: Vec<ExpiredBet>,
    pub expired_accumulated_bets: Vec<ExpiredAccumulatedBet>,
}