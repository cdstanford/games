/*
    The game logic
*/

use std::collections::HashSet;
use std::str::FromStr;

use super::board::{Board, Coord, Dir};
use crate::play::TwoPlayers;
use crate::traits::{Game, GameStatus};
use crate::util;

const NUM_PLAYERS: usize = 2;
const STARTING_SHIPS: &[usize] = &[3, 4, 5];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShipType {
    length: usize,
}
impl ShipType {
    pub fn from_usize(length: usize) -> Self {
        Self { length }
    }
    pub fn from_isize(length: isize) -> Option<Self> {
        if length >= 0 {
            let length = length as usize;
            Some(Self { length })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    PlaceShip(ShipType, Coord, Dir),
    Shoot(Coord),
}
impl Move {
    pub fn is_valid(&self) -> bool {
        match *self {
            Move::PlaceShip(_ship, coord, dir) => {
                coord.is_valid() && dir.is_valid()
            }
            Move::Shoot(coord) => coord.is_valid(),
        }
    }
    fn parse_core(s: &str) -> Option<Self> {
        let coords = util::parse_vec_isize(s)?;
        if coords.len() == 5 {
            let ship = ShipType::from_isize(coords[0])?;
            let coord = Coord::from_isize(coords[1], coords[2])?;
            let dir = Dir::from_isize(coords[3], coords[4])?;
            let mv = Move::PlaceShip(ship, coord, dir);
            debug_assert!(mv.is_valid());
            Some(mv)
        } else if coords.len() == 2 {
            let coord = Coord::from_isize(coords[0], coords[1])?;
            let mv = Move::Shoot(coord);
            debug_assert!(mv.is_valid());
            Some(mv)
        } else {
            None
        }
    }
}
impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Self::parse_core(s).ok_or(())
    }
}

#[derive(Debug)]
pub struct GameState {
    to_move: TwoPlayers,
    pending_placement: [HashSet<ShipType>; NUM_PLAYERS],
    boards: [Board; NUM_PLAYERS],
}

impl GameState {
    fn get_board(&self, plyr: TwoPlayers) -> &Board {
        &self.boards[plyr.as_index()]
    }
    fn get_board_mut(&mut self, plyr: TwoPlayers) -> &mut Board {
        &mut self.boards[plyr.as_index()]
    }
    fn get_pending(&self, plyr: TwoPlayers) -> &HashSet<ShipType> {
        &self.pending_placement[plyr.as_index()]
    }
    fn get_pending_mut(&mut self, plyr: TwoPlayers) -> &mut HashSet<ShipType> {
        &mut self.pending_placement[plyr.as_index()]
    }
    fn no_pending_placements(&self) -> bool {
        self.pending_placement.iter().all(|set| set.is_empty())
    }
}

impl Game for GameState {
    type Player = TwoPlayers;
    type Move = Move;

    fn new() -> Self {
        let to_move = TwoPlayers::One;
        let ships: HashSet<ShipType> = STARTING_SHIPS
            .iter()
            .map(|&len| ShipType { length: len })
            .collect();
        let pending_placement = [ships.clone(), ships];
        let boards = [Default::default(), Default::default()];
        Self { to_move, pending_placement, boards }
    }

    fn status(&self) -> GameStatus<TwoPlayers> {
        if !self.pending_placement[0].is_empty() {
            GameStatus::ToMove(TwoPlayers::One)
        } else if !self.pending_placement[1].is_empty() {
            GameStatus::ToMove(TwoPlayers::Two)
        } else if self.boards[0].ship_squares_left() == 0 {
            debug_assert!(self.boards[1].ship_squares_left() > 0);
            GameStatus::Won(TwoPlayers::Two)
        } else if self.boards[1].ship_squares_left() == 0 {
            GameStatus::Won(TwoPlayers::One)
        } else {
            GameStatus::ToMove(self.to_move)
        }
    }
    fn valid_move(&self, plyr: TwoPlayers, mv: &Move) -> bool {
        debug_assert_eq!(self.status(), GameStatus::ToMove(plyr));
        match *mv {
            Move::PlaceShip(ship, coord, dir) => {
                let len = ship.length;
                self.get_pending(plyr).contains(&ship)
                    && coord.is_valid()
                    && dir.is_valid()
                    && self.get_board(plyr).valid_ship_line(coord, dir, len)
            }
            Move::Shoot(coord) => {
                self.no_pending_placements() && coord.is_valid()
            }
        }
    }
    fn make_move(&mut self, plyr: TwoPlayers, mv: Move) {
        debug_assert_eq!(self.status(), GameStatus::ToMove(plyr));
        debug_assert!(self.valid_move(plyr, &mv));
        match mv {
            Move::PlaceShip(ship, coord, dir) => {
                let len = ship.length;
                let board = self.get_board_mut(plyr);
                assert!(board.place_ship_line(coord, dir, len));
                self.get_pending_mut(plyr).remove(&ship);
            }
            Move::Shoot(coord) => {
                self.get_board_mut(plyr).shoot(coord);
            }
        }
    }
}
