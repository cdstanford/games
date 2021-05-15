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

impl Square {
    fn is_ship(&self) -> bool {
        self == &Square::Ship || self == &Square::ShipHit
    }
    fn is_sea(&self) -> bool {
        self == &Square::Sea || self == &Square::SeaMiss
    }
    fn to_public(&self) -> Square {
        if self == &Square::Ship {
            Square::Sea
        } else {
            *self
        }
    }

    pub fn hit(&mut self) -> bool {
        // True if a new hit or a redundant hit
        if self.is_ship() {
            *self = Square::ShipHit;
            true
        } else {
            debug_assert!(self.is_sea());
            *self = Square::SeaMiss;
            false
        }
    }
}

impl View for Square {
    fn eq_priv(&self, other: &Self) -> bool {
        self == other
    }
    fn eq_pub(&self, other: &Self) -> bool {
        self.to_public() == other.to_public()
    }
    // Display ground truth
    fn disp_priv(&self) -> String {
        match *self {
            Square::Ship => "s".to_string(),
            Square::ShipHit => "x".to_string(),
            Square::Sea => "-".to_string(),
            Square::SeaMiss => "o".to_string(),
        }
    }
    // Display public view
    fn disp_pub(&self) -> String {
        self.to_public().disp_priv()
    }
}

/*
    Core board abstraction
*/

#[derive(Debug, Default)]
pub struct Board {
    grid: [[Square; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    pub fn hit(&mut self, row: usize, col: usize) -> bool {
        debug_assert!(row < BOARD_ROWS);
        debug_assert!(col < BOARD_COLS);
        self.grid[row][col].hit()
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
