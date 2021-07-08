/*
    Implementation of the game of Nim
*/

use crate::abstract_game::{AbstractGame, GameStatus};
use crate::player::Player;
use crate::util;

use std::fmt::{self, Display};

#[derive(Debug)]
pub struct NimState<const N: usize> {
    piles: Vec<usize>,
    total_sticks: usize,
    to_move: Player<N>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NimMove {
    // Note: pile should be >= 1 (uses 1-indexing)
    pile: usize,
    take: usize,
}
impl Display for NimMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Take {} from pile {}", self.take, self.pile)
    }
}

impl<const N: usize> AbstractGame<N> for NimState<N> {
    type Move = NimMove;
    type SetupParams = Vec<usize>;

    fn setup_from_user_input() -> Vec<usize> {
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

        piles
    }

    fn game_setup(piles: Vec<usize>) -> Self {
        let total_sticks = piles.iter().sum();
        let to_move = Player::from_index(0).unwrap();
        Self { piles, total_sticks, to_move }
    }

    fn status(&self) -> GameStatus<N> {
        if self.total_sticks == 0 {
            GameStatus::Won(self.to_move.prev_player())
        } else {
            GameStatus::ToMove(self.to_move)
        }
    }

    fn query(&self) -> String {
        "Choose a pile and number of sticks: ".to_string()
    }

    fn parse_move(&self, raw: &str) -> Result<NimMove, String> {
        let ints = util::parse_vec_usize(raw).ok_or_else(|| {
            "Move should be two integers separated by a space. ".to_string()
        })?;
        if ints.len() == 2 {
            Ok(NimMove { pile: ints[0], take: ints[1] })
        } else {
            Err("Move should be exactly two integers. ".to_string())
        }
    }

    fn check_move(&self, mv: &NimMove) -> Result<(), String> {
        // Pile uses one indexing
        if mv.pile == 0 || mv.pile > self.piles.len() {
            Err(format!(
                "Pile should be between {} and {}. ",
                1,
                self.piles.len()
            ))
        } else if mv.take == 0 {
            Err("Must take at least one stick. ".to_string())
        } else if mv.take > self.piles[mv.pile - 1] {
            Err("Not enough sticks in that pile. ".to_string())
        } else {
            Ok(())
        }
    }

    fn make_move(&mut self, mv: NimMove) {
        debug_assert!(mv.pile >= 1);
        debug_assert!(mv.pile <= self.piles.len());
        self.piles[mv.pile - 1] -= mv.take;
        self.total_sticks -= mv.take;
        self.to_move.advance_player();
    }

    fn print_state_visible(&self, _plyr: Player<N>) -> String {
        format!("Piles: {:?}", self.piles)
    }
}
