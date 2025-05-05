//Reprezintă o intrare parsată din fișierul de log
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub source_ip: Option<String>,  // Extras din mesaj (ex: "Failed login from 192.168.1.1")
}