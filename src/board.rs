/*
    Abstraction for squares and boards

    The game will contain 2 boards, one for each player
    (each square includes the public information the other has about
    that square)
*/

// Itertools for .join() over Iter<Item = String>
use itertools::Itertools;

use crate::view::View;

const BOARD_ROWS: usize = 10;
const BOARD_COLS: usize = 10;

/*
    Enums for squares in the grid:
    - Loc is the ground truth about the cell (on either player's board)
    - LocInfo is the info another player might have about a cell
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Square {
    Ship,
    ShipHit,
    Sea,
    SeaMiss,
}

impl Default for Square {
    fn default() -> Self {
        Square::Sea
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HitResult {
    Hit,
    Miss,
}

impl Square {
    /// Hide information about the square that isn't publically visible
    fn hide(&self) -> Square {
        if self == &Square::Ship {
            Square::Sea
        } else {
            *self
        }
    }

    /// Fire a shot at the square
    pub fn shoot(&mut self) -> HitResult {
        // Returns true if a **new** hit
        match *self {
            Square::Ship => {
                *self = Square::ShipHit;
                HitResult::Hit
            }
            Square::Sea => {
                *self = Square::SeaMiss;
                HitResult::Miss
            }
            Square::ShipHit | Square::SeaMiss => HitResult::Miss,
        }
    }
}

impl View for Square {
    fn eq_priv(&self, other: &Self) -> bool {
        self == other
    }
    fn eq_pub(&self, other: &Self) -> bool {
        self.hide() == other.hide()
    }
    fn disp_priv(&self) -> String {
        match *self {
            Square::Ship => "s".to_string(),
            Square::ShipHit => "x".to_string(),
            Square::Sea => "-".to_string(),
            Square::SeaMiss => "o".to_string(),
        }
    }
    fn disp_pub(&self) -> String {
        self.hide().disp_priv()
    }
}

/*
    Core board abstraction
*/

#[derive(Debug, Default)]
pub struct Board {
    grid: [[Square; BOARD_COLS]; BOARD_ROWS],
    ship_remaining: usize,
}

impl Board {
    /// Fire a shot at a square on the board
    pub fn shoot(&mut self, row: usize, col: usize) -> HitResult {
        debug_assert!(row < BOARD_ROWS);
        debug_assert!(col < BOARD_COLS);
        let result = self.grid[row][col].shoot();
        if result == HitResult::Hit {
            debug_assert!(self.ship_remaining > 0);
            self.ship_remaining -= 1;
        }
        result
    }

    /// Get ground truth about a square
    pub fn get_priv(&self, row: usize, col: usize) -> Square {
        debug_assert!(row < BOARD_ROWS);
        debug_assert!(col < BOARD_COLS);
        self.grid[row][col]
    }

    /// Get publicly visible info about a square
    pub fn get_pub(&self, row: usize, col: usize) -> Square {
        debug_assert!(row < BOARD_ROWS);
        debug_assert!(col < BOARD_COLS);
        self.grid[row][col].hide()
    }

    /// Place a ship on the board
    /// Returns true if successful
    pub fn place_ship_square(&mut self, row: usize, col: usize) -> bool {
        debug_assert!(row < BOARD_ROWS);
        debug_assert!(col < BOARD_COLS);
        let square = self.get_priv(row, col);
        if self.get_priv(row, col) == Square::Ship {
            false
        } else {
            // No hits/misses should have occured yet
            debug_assert!(square == Square::Sea);
            self.grid[row][col] = Square::Ship;
            self.ship_remaining += 1;
            true
        }
    }
}

impl View for [Square; BOARD_COLS] {
    fn eq_priv(&self, other: &Self) -> bool {
        (0..BOARD_COLS).all(|i| self[i].eq_priv(&other[i]))
    }
    fn eq_pub(&self, other: &Self) -> bool {
        (0..BOARD_COLS).all(|i| self[i].eq_pub(&other[i]))
    }
    fn disp_priv(&self) -> String {
        self.iter().map(|square| square.disp_priv()).join(" ")
    }
    fn disp_pub(&self) -> String {
        self.iter().map(|square| square.disp_pub()).join(" ")
    }
}

impl View for Board {
    fn eq_priv(&self, other: &Self) -> bool {
        (0..BOARD_ROWS).all(|i| self.grid[i].eq_priv(&other.grid[i]))
    }
    fn eq_pub(&self, other: &Self) -> bool {
        (0..BOARD_ROWS).all(|i| self.grid[i].eq_pub(&other.grid[i]))
    }
    fn disp_priv(&self) -> String {
        self.grid.iter().map(|row| row.disp_priv()).join("\n")
    }
    fn disp_pub(&self) -> String {
        self.grid.iter().map(|row| row.disp_pub()).join("\n")
    }
}
