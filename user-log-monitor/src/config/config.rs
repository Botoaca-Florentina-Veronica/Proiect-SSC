//Definește ce fișiere de log să fie monitorizate și reguli 
//(e.g., numărul maxim de autentificări eșuate înainte de alertă)

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub log_path: String,           // "/var/log/auth.log"
    pub max_failed_logins: u32,    // 5
}