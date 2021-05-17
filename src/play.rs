/*
    Code to play (execute) a game
*/

use crate::traits::Game;

/*
    Two-player enum
    Would be cool to do this with const generics
    (Make a type for an integer between 0 and NUM_PLAYERS)
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TwoPlayers {
    One,
    Two,
}

impl TwoPlayers {
    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => TwoPlayers::One,
            1 => TwoPlayers::Two,
            _ => panic!("Bad index provided to initialize player"),
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            TwoPlayers::One => 0,
            TwoPlayers::Two => 1,
        }
    }
}

/*
    Play a game
*/

pub fn play_vs_ai<G>()
where
    G: Game<Player = TwoPlayers>,
{
    unimplemented!()
}
