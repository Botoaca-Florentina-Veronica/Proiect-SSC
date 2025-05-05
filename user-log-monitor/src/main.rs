//Pornește monitorizarea și procesează logurile în timp real
mod config;
mod models;
mod detectors;
mod utils;
mod middleware;

use std::fs::File;
use std::io::{BufRead, BufReader};
use detectors::failed_logins::FailedLoginDetector;
use utils::log_parser::parse_line;

fn main() {
    let config = config::config::load_config();  // Încarcă fișierul de configurare
    let mut detector = FailedLoginDetector::new();
    let file = File::open(config.log_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(log_entry) = parse_line(&line) {
                if let Some(alert) = detector.detect(&log_entry) {
                    println!("ALERT: {}", alert.message);
                }
            }
        }
    }
}