// === src/models/log.rs ===
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub source_ip: String,
    pub dest_ip: String,
    pub source_mac: String,
    pub dest_mac: String,
    pub protocol: String,
    pub suspicious: bool,
    pub timestamp: DateTime<Utc>,
}
