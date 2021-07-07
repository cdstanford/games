/*
    The game logic
*/

use std::collections::HashSet;
use std::fmt::{self, Display};

use super::board::{Board, Coord, Dir};

use crate::abstract_game::{AbstractGame, Ai, GameStatus};
use crate::player::TwoPlayers;
use crate::util;
use crate::view::View;

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
impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Move::PlaceShip(ship, coord, dir) => {
                write!(f, "Place({:?}, {:?}, {:?})", ship, coord, dir)
            }
            Move::Shoot(coord) => {
                write!(f, "Shoot({:?})", coord)
            }
        }
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
    fn print_pending(&self, plyr: TwoPlayers) -> String {
        let mut result = String::new();
        for &ship in self.get_pending(plyr).iter() {
            result.push_str(&format!("{} ", ship.length));
        }
        result
    }
    fn no_pending_placements(&self) -> bool {
        self.pending_placement.iter().all(|set| set.is_empty())
    }
    fn is_valid_move_core(&self, mv: &Move) -> bool {
        let plyr = self.cur_player().unwrap();
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
}

impl AbstractGame<NUM_PLAYERS> for GameState {
    type Move = Move;

    fn new() -> Self {
        let to_move = TwoPlayers::ONE;
        let ships: HashSet<ShipType> = STARTING_SHIPS
            .iter()
            .map(|&len| ShipType { length: len })
            .collect();
        let pending_placement = [ships.clone(), ships];
        let boards = [Default::default(), Default::default()];
        Self { to_move, pending_placement, boards }
    }

    fn status(&self) -> GameStatus<NUM_PLAYERS> {
        if !self.pending_placement[0].is_empty() {
            GameStatus::ToMove(TwoPlayers::ONE)
        } else if !self.pending_placement[1].is_empty() {
            GameStatus::ToMove(TwoPlayers::TWO)
        } else if self.boards[0].ship_squares_left() == 0 {
            debug_assert!(self.boards[1].ship_squares_left() > 0);
            GameStatus::Won(TwoPlayers::TWO)
        } else if self.boards[1].ship_squares_left() == 0 {
            GameStatus::Won(TwoPlayers::ONE)
        } else {
            GameStatus::ToMove(self.to_move)
        }
    }
    fn query(&self) -> String {
        // TODO: make more helpful
        "Move: ".to_string()
    }
    fn parse_move(&self, raw: &str) -> Result<Move, String> {
        // TODO: make this more helpful
        Move::parse_core(raw)
            .ok_or_else(|| "Could not parse move. ".to_string())
    }
    fn check_move(&self, mv: &Move) -> Result<(), String> {
        // TODO: make this more helpful
        if self.is_valid_move_core(mv) {
            Ok(())
        } else {
            Err("Invalid move".to_string())
        }
    }
    fn make_move(&mut self, mv: Move) {
        let plyr = self.cur_player().unwrap();
        debug_assert!(self.is_valid_move(&mv));
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
    fn print_state_visible(&self, plyr: TwoPlayers) -> String {
        if self.get_pending(plyr).is_empty() {
            let other = plyr.opponent();
            format!(
                "=== Your Board ===\n{}\n=== Shots ===\n{}\n",
                self.get_board(plyr).disp_priv(),
                self.get_board(other).disp_pub(),
            )
        } else {
            format!(
                "=== Your Board ===\n{}\n=== Ships to Place ===\n{}\n",
                self.get_board(plyr).disp_priv(),
                self.print_pending(plyr),
            )
        }
    }
}

pub struct UnimplementedBattleshipAi {}

impl Ai<GameState, NUM_PLAYERS> for UnimplementedBattleshipAi {
    fn new() -> Self {
        Self {}
    }
    fn ai_move(&mut self, _game_state: &GameState, _plyr: TwoPlayers) -> Move {
        unimplemented!()
    }
}
