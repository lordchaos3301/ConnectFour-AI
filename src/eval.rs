use crate::board::{Board, COLUMNS, ROWS, DIRECTIONS};

pub fn evaluate(board: &Board, player: i8) -> i32 {
    let mut score = 0;

    for row in 0..ROWS as i32 {
        for col in 0..COLUMNS as i32 {
            for (dr, dc) in DIRECTIONS {
                let end_row = row + 3 * dr;
                let end_col = col + 3 * dc;

                // check if the window is on the board
                if end_row < 0 || end_row >= ROWS as i32 || end_col < 0 || end_col >= COLUMNS as i32 {
                    continue;
                }

                let mut window = [0i8; 4];

                for i in 0..4 {
                    let rr = row + i * dr;
                    let cc = col + i * dc;
                    window[i as usize] = board.get(rr as usize, cc as usize);
                }

                score += score_window(window, player);
            }
        }
    }

    for row in 0..ROWS {
        let cell = board.get(row, COLUMNS / 2);
        if cell == player {
            score += 3;
        }
        if cell == -player {
            score -= 3;
        }
    }

    score
}

fn score_window(window: [i8; 4], player: i8) -> i32 {
    let mut own = 0;
    let mut opp = 0;

    for cell in window {
        if cell == player {
            own += 1;
        }
        if cell == -player {
            opp += 1;
        }
    }

    match (own, opp) {
        (4, 0) => 100,
        (3, 0) => 5,
        (2, 0) => 2,
        (1, 0) => 0,
        (0, 4) => -100,
        (0, 3) => -5,
        (0, 2) => -2,
        (0, 1) => 0,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn scores() {
        assert_eq!(score_window([1, 1, 1, 0], 1), 5);
        assert_eq!(score_window([1, 1, 0, -1], 1), 0);
        assert_eq!(score_window([1, 1, 1, 0], -1), -5);
    }

    #[test]
    fn empty_board_is_zero() {
        let board = Board::new();
        assert_eq!(evaluate(&board, 1), 0);
    }

    #[test]
    fn center_stone() {
        let mut board = Board::new();
        board.play(3);
        assert_eq!(evaluate(&board, 1), 3);
        assert_eq!(evaluate(&board, -1), -3);
    }

    #[test]
    fn three_in_a_row_is_good() {
        let mut board = Board::new();
        for col in [0, 6, 1, 6, 2] {
            board.play(col);
        }
        assert!(evaluate(&board, 1) > 0);
        assert!(evaluate(&board, -1) < 0);
    }

    #[test]
    fn symmetric() {
        let mut board = Board::new();
        for col in [3, 2, 4, 4, 1, 0, 5] {
            board.play(col);
        }
        assert_eq!(evaluate(&board, 1), -evaluate(&board, -1));
    }
}
