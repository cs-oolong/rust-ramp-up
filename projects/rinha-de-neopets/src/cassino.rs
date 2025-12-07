use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CassinoEvent {
    pub description: String,
    pub odd: f64,
}