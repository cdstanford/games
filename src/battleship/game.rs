/*
    The game logic
*/

use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::board::{Board, Coord, Dir, ParseCoordError};
use crate::play::TwoPlayers;
use crate::traits::{Game, GameStatus};

const NUM_PLAYERS: usize = 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShipType {
    length: usize,
}

#[derive(Debug)]
pub enum ParseMoveError {
    ParseCoordError(ParseCoordError),
    // TODO
}

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    PlaceShip(ShipType, Coord, Dir),
    Shoot(Coord),
}
impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
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

    fn status(&self) -> GameStatus<TwoPlayers> {
        // This is written in a way agnostic to the number of players
        for i in 0..NUM_PLAYERS {
            if self.boards[i].ship_squares_left() == 0 {
                for j in (i + 1)..NUM_PLAYERS {
                    debug_assert!(self.boards[j].ship_squares_left() > 0);
                }
                return GameStatus::Won(TwoPlayers::from_index(i));
            }
        }
        GameStatus::ToMove(self.to_move)
    }
    fn valid_move(&self, plyr: TwoPlayers, mv: Move) -> bool {
        debug_assert_eq!(self.status(), GameStatus::ToMove(plyr));
        match mv {
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
        debug_assert!(self.valid_move(plyr, mv));
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
