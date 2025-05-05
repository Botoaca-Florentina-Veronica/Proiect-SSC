
# Rust-MITM-Detector

### Descriere
Rust-MITM-Detector este o aplicație CLI/REST scrisă în Rust care monitorizează traficul de rețea în timp real pentru detectarea atacurilor de tip Man-in-the-Middle. Logurile sunt salvate în MongoDB pentru analiză ulterioară.

### Funcționalități
- Capturare și analiză trafic în rețea
- Detectarea atacurilor de tip ARP spoofing / IP duplicat
- Salvare loguri în MongoDB
- Sistem de alertare în terminal / webhook (opțional)
- Interfață REST opțională pentru acces la loguri

### Tehnologii
- Rust
- Tokio async runtime
- MongoDB (cu driver async)
- `pnet` pentru procesarea pachetelor de rețea
- `.env` pentru variabile sensibile

### Precondiții
- Cargo + Rust instalat
- MongoDB local sau Atlas
- Permisiuni sudo pentru acces la rețea

### Rulează proiectul
```bash
cargo run
