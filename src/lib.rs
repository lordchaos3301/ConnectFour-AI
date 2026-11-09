//! Vier gewinnt – Einstiegspunkt der Bibliothek.
//!
//! Hier werden die Module eingebunden und die Funktionen definiert,
//! die per `#[wasm_bindgen]` für JavaScript sichtbar sind.

pub mod ai;
pub mod board;
pub mod eval;

// TODO: wasm-Schnittstelle
// - eine Struktur `Game` mit #[wasm_bindgen], die ein Board hält
// - Methoden: neues Spiel, Zug in Spalte, Gewinner abfragen,
//   Feldinhalt für die UI auslesen, KI-Zug berechnen
