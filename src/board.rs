use std::fmt;

pub const SIZE: usize = 9;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [[u8; SIZE]; SIZE],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..SIZE {
            if row % 3 == 0 {
                writeln!(f, "+-------+-------+-------+")?;
            }
            for col in 0..SIZE {
                if col % 3 == 0 {
                    write!(f, "| ")?;
                }
                let val = self.cells[row][col];
                if val == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", val)?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+-------+-------+")?;
        Ok(())
    }
}

impl Board {
    /// Creates a new empty Sudoku board.
    pub fn new() -> Self {
        Self {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Sets a value at the given row and column.
    /// Returns true if the move is valid and performed, false otherwise.
    pub fn set_cell(&mut self, row: usize, col: usize, val: u8) -> bool {
        if row >= SIZE || col >= SIZE || val > 9 {
            return false;
        }

        if val != 0 && !self.is_valid_move(row, col, val) {
            return false;
        }

        self.cells[row][col] = val;
        true
    }

    /// Gets the value at the given row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    /// Checks if placing `val` at `(row, col)` is valid according to Sudoku rules.
    pub fn is_valid_move(&self, row: usize, col: usize, val: u8) -> bool {
        if val == 0 {
            return true;
        }

        // Check row
        for c in 0..SIZE {
            if c != col && self.cells[row][c] == val {
                return false;
            }
        }

        // Check column
        for r in 0..SIZE {
            if r != row && self.cells[r][col] == val {
                return false;
            }
        }

        // Check 3x3 box
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for r in start_row..start_row + 3 {
            for c in start_col..start_col + 3 {
                if (r != row || c != col) && self.cells[r][c] == val {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        for r in 0..SIZE {
            for c in 0..SIZE {
                assert_eq!(board.get_cell(r, c), 0);
            }
        }
    }

    #[test]
    fn test_valid_move() {
        let mut board = Board::new();
        assert!(board.set_cell(0, 0, 5));
        assert_eq!(board.get_cell(0, 0), 5);
    }

    #[test]
    fn test_invalid_row_move() {
        let mut board = Board::new();
        board.set_cell(0, 0, 5);
        assert!(!board.is_valid_move(0, 5, 5));
    }

    #[test]
    fn test_invalid_col_move() {
        let mut board = Board::new();
        board.set_cell(0, 0, 5);
        assert!(!board.is_valid_move(5, 0, 5));
    }

    #[test]
    fn test_invalid_box_move() {
        let mut board = Board::new();
        board.set_cell(0, 0, 5);
        assert!(!board.is_valid_move(1, 1, 5));
    }
}
