//! KI-Gegner.
//!
//! Mathe-Schritt 2: Negamax  V(s) = max_a [ -V(s') ]
//! Mathe-Schritt 4: Alpha-Beta-Pruning mit Fenster [alpha, beta]

// TODO: negamax(board, depth, alpha, beta) -> i32
// - Endstellung: Sieg/Niederlage/Remis, Sieg mit Tiefenbonus
// - Tiefe 0: eval::h(s)
// - sonst: alle Züge, Wert = -negamax(...), Abbruch bei alpha >= beta

// TODO: Zugreihenfolge – mittlere Spalten zuerst

// TODO: best_move(board, depth) -> usize

#[cfg(test)]
mod tests {
    // TODO: KI findet Sieg in 1 Zug, KI blockt Sieg des Gegners
}
