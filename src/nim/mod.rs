/*
    Implementation of the game of Nim
*/

use crate::abstract_game::{AbstractGame, GameStatus};
use crate::player::Player;
use crate::util;

use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug)]
pub struct NimState<const N: usize> {
    piles: Vec<usize>,
    total_sticks: usize,
    to_move: Player<N>,
}

impl<const N: usize> NimState<N> {
    pub fn new(piles: Vec<usize>) -> Self {
        let total_sticks = piles.iter().sum();
        let to_move = Player::from_index(0).unwrap();
        Self { piles, total_sticks, to_move }
    }
    pub fn new_from_user_input() -> Self {
        let mut piles: Vec<usize> = Vec::new();
        let num_piles: usize = util::from_user_input(
            "Number of piles? ",
            "Type a nonnegative integer. ",
        );
        for i in 1..=num_piles {
            let query = format!("Pile {} size? ", i);
            piles.push(util::from_user_input_satisfying(
                &query,
                "Type a positive integer. ",
                "Type a positive integer. ",
                |&pile| pile > 0,
            ));
        }
        debug_assert_eq!(piles.len(), num_piles);
        println!("Piles: {:?}", piles);

        Self::new(piles)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NimMove {
    pile: usize,
    take: usize,
}
impl Display for NimMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Take {} from pile {}", self.take, self.pile)
    }
}
impl FromStr for NimMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let ints = util::parse_vec_usize(s).ok_or(())?;
        if ints.len() == 2 {
            Ok(Self { pile: ints[0], take: ints[1] })
        } else {
            Err(())
        }
    }
}

impl<const N: usize> AbstractGame<N> for NimState<N> {
    type Move = NimMove;

    fn new() -> Self {
        Self::new_from_user_input()
    }
    fn status(&self) -> GameStatus<N> {
        if self.total_sticks == 0 {
            GameStatus::Won(self.to_move.prev_player())
        } else {
            GameStatus::ToMove(self.to_move)
        }
    }
    fn is_valid_move(&self, mv: &NimMove) -> bool {
        mv.pile < self.piles.len()
            && mv.take >= 1
            && mv.take <= self.piles[mv.pile]
    }
    fn make_move(&mut self, mv: Self::Move) {
        self.piles[mv.pile] -= mv.take;
        self.total_sticks -= mv.take;
        self.to_move.advance_player();
    }
    fn print_state_visible(&self, _plyr: Player<N>) -> String {
        format!("Piles: {:?}", self.piles)
    }
}
