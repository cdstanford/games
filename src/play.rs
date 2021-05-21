/*
    Code to play (execute) a game
*/

use super::abstract_game::{AbstractGame, Ai, GameStatus};
use super::util;

use std::fmt::{self, Display};
use std::str::FromStr;

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
    pub fn opponent(&self) -> Self {
        match self {
            TwoPlayers::One => TwoPlayers::Two,
            TwoPlayers::Two => TwoPlayers::One,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            TwoPlayers::One => "one",
            TwoPlayers::Two => "two",
        }
    }
    pub fn name_upper(&self) -> &str {
        match self {
            TwoPlayers::One => "One",
            TwoPlayers::Two => "Two",
        }
    }
}

impl Display for TwoPlayers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/*
    Code to play (execute) a game
*/

pub fn play_vs_yourself<G>()
where
    G: AbstractGame<Player = TwoPlayers>,
    G::Move: FromStr + Display,
{
    let mut game = G::new();
    loop {
        match game.status() {
            GameStatus::ToMove(plyr) => {
                println!("===== Player {}'s turn =====", plyr);
                println!("{}", game.print_state_visible(plyr));
                let mv = util::from_user_input_satisfying(
                    "Move: ",
                    "Invalid syntax, try again: ",
                    "Invalid move, try again: ",
                    |mv| game.valid_move(plyr, mv),
                );
                debug_assert!(game.valid_move(plyr, &mv));
                println!("Move chosen: {}", mv);
                game.make_move(plyr, mv);
            }
            GameStatus::Won(plyr) => {
                println!("Player {} wins!", plyr);
                return;
            }
        }
    }
}

pub fn play_vs_ai<G, A>()
where
    G: AbstractGame<Player = TwoPlayers>,
    A: Ai<G>,
    G::Move: FromStr + Display,
{
    let mut game = G::new();
    let mut ai = A::new();
    loop {
        match game.status() {
            GameStatus::ToMove(TwoPlayers::One) => {
                println!("===== Your turn =====");
                println!("{}", game.print_state_visible(TwoPlayers::One));
                let mv = util::from_user_input_satisfying(
                    "Move: ",
                    "Invalid syntax, try again: ",
                    "Invalid move, try again: ",
                    |mv| game.valid_move(TwoPlayers::One, mv),
                );
                debug_assert!(game.valid_move(TwoPlayers::One, &mv));
                println!("Your move: {}", mv);
                game.make_move(TwoPlayers::One, mv);
            }
            GameStatus::ToMove(TwoPlayers::Two) => {
                println!("===== Opponent's turn =====");
                let mv = ai.ai_move(&game, TwoPlayers::Two);
                debug_assert!(game.valid_move(TwoPlayers::Two, &mv));
                println!("Opponent's move: {}", mv);
                game.make_move(TwoPlayers::Two, mv);
            }
            GameStatus::Won(TwoPlayers::One) => {
                println!("You win!");
                return;
            }
            GameStatus::Won(TwoPlayers::Two) => {
                println!("You lose!");
                return;
            }
        }
    }
}
