//Parsează fișierele de log în structuri organizate
use crate::models::log_entry::LogEntry;

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }

    let timestamp = format!("{} {}", parts[0], parts[1]);
    let message = parts[2].to_string();
    let source_ip = extract_ip(&message);  // Funcție helper (poate folosi regex)

    Some(LogEntry { timestamp, message, source_ip })
}