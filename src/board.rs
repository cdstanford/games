/*
    Abstraction for squares and boards

    The game will contain 2 boards, one for each player
    (each square includes the public information the other has about
    that square)
*/

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

// Itertools for .join() over Iter<Item = String>
use itertools::Itertools;

use crate::traits::View;

/*
    Coordinates and directions
*/

const BOARD_ROWS: usize = 10;
const BOARD_COLS: usize = 10;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    row: usize,
    col: usize,
}
impl Coord {
    pub fn is_valid(&self) -> bool {
        self.row < BOARD_ROWS && self.col < BOARD_COLS
    }
}

#[derive(Debug)]
pub enum ParseCoordError {
    ParseRowError(ParseIntError),
    ParseColError(ParseIntError),
    TooFewCoords,
    TooManyCoords,
}

impl Display for ParseCoordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseCoordError {}

// Partly copied from:
// https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
impl FromStr for Coord {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')' )
            .split(',')
            .collect();
        match coords.len().cmp(&2) {
            Ordering::Greater => Err(ParseCoordError::TooManyCoords),
            Ordering::Less => Err(ParseCoordError::TooFewCoords),
            Ordering::Equal => {
                let x_fromstr = match coords[0].parse::<usize>() {
                    Ok(x) => x,
                    Err(x) => return Err(ParseCoordError::ParseRowError(x)),
                };
                let y_fromstr = match coords[1].parse::<usize>() {
                    Ok(x) => x,
                    Err(x) => return Err(ParseCoordError::ParseColError(x)),
                };
                Ok(Coord { row: x_fromstr, col: y_fromstr })
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dir {
    drow: isize,
    dcol: isize,
}
impl Dir {
    pub fn is_valid(&self) -> bool {
        (self.drow != 0 || self.dcol != 0)
            && self.drow >= -1
            && self.drow <= 1
            && self.dcol >= -1
            && self.dcol <= 1
    }
}

impl Coord {
    fn add(self, dir: Dir) -> Option<Self> {
        let row = (self.row as isize) + dir.drow;
        let col = (self.col as isize) + dir.dcol;
        if row < 0 || col < 0 {
            None
        } else {
            let row = row as usize;
            let col = col as usize;
            Some(Self { row, col })
        }
    }
}

/*
    Enums for squares in the grid:
    - Square is the ground truth about the cell (on either player's board)
    - HitResult is the result of shooting a square
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
    fn get_square(&self, coord: Coord) -> &Square {
        debug_assert!(coord.is_valid());
        &self.grid[coord.row][coord.col]
    }
    fn get_square_mut(&mut self, coord: Coord) -> &mut Square {
        debug_assert!(coord.is_valid());
        &mut self.grid[coord.row][coord.col]
    }

    /// Get remaining ship squares
    pub fn ship_squares_left(&self) -> usize {
        self.ship_remaining
    }

    /// Fire a shot at a square on the board
    pub fn shoot(&mut self, coord: Coord) -> HitResult {
        debug_assert!(coord.is_valid());
        let result = self.get_square_mut(coord).shoot();
        if result == HitResult::Hit {
            debug_assert!(self.ship_remaining > 0);
            self.ship_remaining -= 1;
        }
        result
    }

    /// Get ground truth about a square
    pub fn get_priv(&self, coord: Coord) -> Square {
        debug_assert!(coord.is_valid());
        *self.get_square(coord)
    }

    /// Get publicly visible info about a square
    pub fn get_pub(&self, coord: Coord) -> Square {
        debug_assert!(coord.is_valid());
        self.get_square(coord).hide()
    }

    /// Place a ship on the board
    /// Returns true if successful
    pub fn place_ship_square(&mut self, coord: Coord) -> bool {
        debug_assert!(coord.is_valid());
        let square = self.get_square(coord);
        if square == &Square::Ship {
            false
        } else {
            // No hits/misses should have occured yet
            debug_assert!(square == &Square::Sea);
            *self.get_square_mut(coord) = Square::Ship;
            self.ship_remaining += 1;
            true
        }
    }

    /// Check if a line of squares is open (sea)
    pub fn valid_ship_line(
        &self,
        coord: Coord,
        dir: Dir,
        length: usize,
    ) -> bool {
        if !coord.is_valid() || !dir.is_valid() {
            false
        } else if length == 0 {
            true
        } else {
            let square = self.get_square(coord);
            if square == &Square::Ship {
                false
            } else {
                debug_assert!(square == &Square::Sea);
                let new_len = length - 1;
                match coord.add(dir) {
                    Some(new) => self.valid_ship_line(new, dir, new_len),
                    None => false,
                }
            }
        }
    }

    /// Place a line of ships on the board
    /// Returns true if successful
    pub fn place_ship_line(
        &mut self,
        mut coord: Coord,
        dir: Dir,
        length: usize,
    ) -> bool {
        if self.valid_ship_line(coord, dir, length) {
            for _ in 0..length {
                assert!(self.place_ship_square(coord));
                coord = coord.add(dir).unwrap();
            }
            true
        } else {
            false
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
