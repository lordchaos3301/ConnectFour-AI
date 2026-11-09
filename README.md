# Vier gewinnt

Vier gewinnt mit KI-Gegner (Negamax + Alpha-Beta) in Rust, kompiliert zu WebAssembly, mit Browser-UI.

## Struktur

```
vier-gewinnt/
├── Cargo.toml        Rust-Projekt
├── src/
│   ├── lib.rs        Module + wasm-Schnittstelle
│   ├── board.rs      Spielfeld und Regeln
│   ├── eval.rs       Bewertungsfunktion h(s)
│   └── ai.rs         Negamax mit Alpha-Beta
├── tests/
│   └── game_tests.rs Integrationstests
└── www/
    ├── index.html    Seite
    ├── style.css     Aussehen
    └── main.js       UI-Logik, ruft Rust auf
```

## Einmalige Einrichtung

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Bauen und starten

```
cargo test                                   # Logik testen
wasm-pack build --target web --out-dir www/pkg
cd www && python3 -m http.server 8000
```

Dann im Browser `http://localhost:8000` öffnen.
(Per Doppelklick auf `index.html` geht es nicht, weil Browser WebAssembly-Module nur über einen Server laden.)
