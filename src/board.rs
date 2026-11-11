pub const ROWS: usize = 6;
pub const COLUMNS: usize = 7;
pub const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

pub struct Board {
    active_player: i8,
    board_state: [[i8; COLUMNS]; ROWS],
    moves: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            active_player: 1,
            board_state: [[0; COLUMNS]; ROWS],
            moves: 0,
        }
    }

    pub fn can_play(&self, col: usize) -> bool {
        col < COLUMNS && self.board_state[0][col] == 0
    }

    pub fn play(&mut self, col: usize) -> Option<usize> {
        if !self.can_play(col) {
            return None;
        }

        for row in (0..ROWS).rev() {
            if self.board_state[row][col] == 0 {
                self.board_state[row][col] = self.active_player;
                self.active_player *= -1;
                self.moves += 1;
                return Some(row);
            }
        }
        None
    }

    pub fn undo(&mut self, col: usize) {
        for row in 0..ROWS {
            if self.board_state[row][col] != 0 {
                self.board_state[row][col] = 0;
                self.active_player *= -1;
                self.moves -= 1;
                return;
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> i8 {
        self.board_state[row][col]
    }

    pub fn is_win(&self, row: usize, col: usize) -> bool {
        for (dr, dc) in DIRECTIONS {
            let total = self.count(row, col, dr, dc) + self.count(row, col, -dr, -dc) + 1;
            if total >= 4 {
                return true;
            }
        }
        false
    }

    pub fn is_full(&self) -> bool {
        self.moves == ROWS * COLUMNS
    }

    fn count(&self, row: usize, col: usize, dr: i32, dc: i32) -> usize {
        let player = self.board_state[row][col];
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        let mut n = 0;

        while r >= 0
            && r < ROWS as i32
            && c >= 0
            && c < COLUMNS as i32
            && self.board_state[r as usize][c as usize] == player
        {
            n += 1;
            r += dr;
            c += dc;
        }
        n
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_play_on_empty_board() {
        let board = Board::new();
        assert!(board.can_play(0));
        assert!(!board.can_play(7));
    }

    #[test]
    fn play_on_empty_board() {
        let mut board = Board::new();
        assert_eq!(board.play(3), Some(5));
        assert_eq!(board.get(5, 3), 1);
    }

    #[test]
    fn double_play_on_empty_board() {
        let mut board = Board::new();
        assert_eq!(board.play(3), Some(5));
        assert_eq!(board.get(5, 3), 1);
        assert_eq!(board.play(3), Some(4));
        assert_eq!(board.get(4, 3), -1);
    }

    #[test]
    fn full_column_returns_none() {
        let mut board = Board::new();
        for expected in (0..ROWS).rev() {
            assert_eq!(board.play(3), Some(expected));
        }
        assert_eq!(board.play(3), None);
        assert!(!board.can_play(3));
    }

    #[test]
    fn win_four_horizontal() {
        let mut board = Board::new();
        for col in [0, 6, 1, 6, 2, 6] {
            board.play(col);
        }
        let row = board.play(3).unwrap();
        assert!(board.is_win(row, 3));
    }

    #[test]
    fn win_four_horizontal_middle() {
        let mut board = Board::new();
        for col in [0, 6, 1, 6, 3, 6] {
            board.play(col);
        }
        let row = board.play(2).unwrap();
        assert!(board.is_win(row, 2));
    }

    #[test]
    fn win_four_vertical() {
        let mut board = Board::new();
        for col in [0, 1, 0, 1, 0, 1] {
            board.play(col);
        }
        let row = board.play(0).unwrap();
        assert!(board.is_win(row, 0));
    }

    #[test]
    fn no_win_three_vertical() {
        let mut board = Board::new();
        for col in [0, 1, 0, 1, 0] {
            board.play(col);
        }
        let row = board.play(1).unwrap();
        assert!(!board.is_win(row, 1));
    }

    #[test]
    fn win_four_diagonal_1_1() {
        let mut board = Board::new();
        for col in [3, 2, 2, 1, 1, 0, 1, 0, 0, 6] {
            board.play(col);
        }
        let row = board.play(0).unwrap();
        assert!(board.is_win(row, 0));
    }

    #[test]
    fn win_four_diagonal_1_minus_1() {
        let mut board = Board::new();
        for col in [3, 4, 4, 5, 5, 6, 5, 6, 6, 0] {
            board.play(col);
        }
        let row = board.play(6).unwrap();
        assert!(board.is_win(row, 6));
    }

    #[test]
    fn undo_move() {
        let mut board = Board::new();
        board.play(3);
        board.undo(3);
        assert_eq!(board.get(5, 3), 0);
        assert_eq!(board.play(3), Some(5));
        assert_eq!(board.get(5, 3), 1);
    }

    #[test]
    fn undo_on_empty_board() {
        let mut board = Board::new();
        board.undo(3);
    }

    #[test]
    fn full_board() {
        let mut board = Board::new();
        assert!(!board.is_full());
        for col in 0..COLUMNS {
            for _ in 0..ROWS {
                board.play(col);
            }
        }
        assert!(board.is_full())
    }
}
