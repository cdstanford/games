/*
    The game logic
*/

use std::collections::HashSet;

use crate::board::{Board, Coord, Dir};
use crate::traits::{Game, GameStatus};

/// Would be cool to do this with const generics
/// Make a type for an integer between 0 and NUM_PLAYERS
const NUM_PLAYERS: usize = 2;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}
impl Player {
    fn from_index(idx: usize) -> Self {
        match idx {
            0 => Player::One,
            1 => Player::Two,
            _ => panic!("Bad index provided to initialize player"),
        }
    }
    fn as_index(&self) -> usize {
        match self {
            Player::One => 0,
            Player::Two => 1,
        }
    }
    // fn num_players() -> usize {
    //     NUM_PLAYERS
    // }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ShipType {
    length: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    PlaceShip(ShipType, Coord, Dir),
    Shoot(Coord),
}

#[derive(Debug)]
pub struct GameState {
    to_move: Player,
    pending_placement: [HashSet<ShipType>; NUM_PLAYERS],
    boards: [Board; NUM_PLAYERS],
}

impl GameState {
    // fn get_board(&self, plyr: Player) -> &Board {
    //     &self.boards[plyr.as_index()]
    // }
    fn get_board_mut(&mut self, plyr: Player) -> &mut Board {
        &mut self.boards[plyr.as_index()]
    }
    // fn get_pending(&self, plyr: Player) -> &HashSet<ShipType> {
    //     &self.pending_placement[plyr.as_index()]
    // }
    // fn get_pending_mut(&mut self, plyr: Player) -> &mut HashSet<ShipType> {
    //     &mut self.pending_placement[plyr.as_index()]
    // }
}

impl Game for GameState {
    type Player = Player;
    type Move = Move;

    fn status(&self) -> GameStatus<Self::Player> {
        // This is written in a way agnostic to the number of players
        for i in 0..NUM_PLAYERS {
            if self.boards[i].ship_squares_left() == 0 {
                for j in (i + 1)..NUM_PLAYERS {
                    debug_assert!(self.boards[j].ship_squares_left() > 0);
                }
                return GameStatus::Won(Player::from_index(i));
            }
        }
        GameStatus::ToMove(self.to_move)
    }
    fn valid_move(&self, plyr: Player, mv: Move) -> bool {
        debug_assert_eq!(self.status(), GameStatus::ToMove(plyr));
        match mv {
            Move::PlaceShip(_ship, _coord, _dir) => {
                // self.get_board(plyr)
                // self.boards[plyr.as_index()]
                // let pending = self.pending[]
                // let board = self.boards[]
                unimplemented!()
                // coord.is_valid() && dir.is_valid() &&
            }
            Move::Shoot(coord) => coord.is_valid(),
        }
    }
    fn make_move(&mut self, plyr: Player, mv: Move) {
        debug_assert_eq!(self.status(), GameStatus::ToMove(plyr));
        debug_assert!(self.valid_move(plyr, mv));
        match mv {
            Move::PlaceShip(_ship, _coord, _dir) => {
                unimplemented!()
            }
            Move::Shoot(coord) => {
                self.get_board_mut(plyr).shoot(coord);
            }
        }
    }
}
