//Detectează autentificări eșuate repetate de la aceeași adresă IP
use std::collections::HashMap;
use crate::models::{log_entry::LogEntry, alert::Alert};

pub struct FailedLoginDetector {
    failed_attempts: HashMap<String, u32>,  // IP -> număr de încercări
}

impl FailedLoginDetector {
    pub fn new() -> Self {
        Self { failed_attempts: HashMap::new() }
    }

    pub fn detect(&mut self, log: &LogEntry) -> Option<Alert> {
        if log.message.contains("Failed login") {
            let ip = log.source_ip.clone()?;
            let count = self.failed_attempts.entry(ip.clone()).or_insert(0);
            *count += 1;

            if *count >= 5 {  // Threshold din config
                Some(Alert::new(format!("Brute-force attempt from {}", ip)))
            } else {
                None
            }
        } else {
            None
        }
    }
}