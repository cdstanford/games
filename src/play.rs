/*
    Code to play (execute) a game
*/

use super::abstract_game::{AbstractGame, Ai, GameStatus};
use super::player::TwoPlayers;
use super::util;

use std::fmt::Display;
use std::str::FromStr;

/*
    Code to play (execute) a game
*/

pub fn play_vs_yourself<G>()
where
    G: AbstractGame<2>,
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
    G: AbstractGame<2>,
    A: Ai<G, 2>,
    G::Move: FromStr + Display,
{
    let mut game = G::new();
    let mut ai = A::new();
    loop {
        match game.status() {
            GameStatus::ToMove(TwoPlayers::ONE) => {
                println!("===== Your turn =====");
                println!("{}", game.print_state_visible(TwoPlayers::ONE));
                let mv = util::from_user_input_satisfying(
                    "Move: ",
                    "Invalid syntax, try again: ",
                    "Invalid move, try again: ",
                    |mv| game.valid_move(TwoPlayers::ONE, mv),
                );
                debug_assert!(game.valid_move(TwoPlayers::ONE, &mv));
                println!("Your move: {}", mv);
                game.make_move(TwoPlayers::ONE, mv);
            }
            GameStatus::ToMove(TwoPlayers::TWO) => {
                println!("===== Opponent's turn =====");
                let mv = ai.ai_move(&game, TwoPlayers::TWO);
                debug_assert!(game.valid_move(TwoPlayers::TWO, &mv));
                println!("Opponent's move: {}", mv);
                game.make_move(TwoPlayers::TWO, mv);
            }
            GameStatus::Won(TwoPlayers::ONE) => {
                println!("You win!");
                return;
            }
            GameStatus::Won(TwoPlayers::TWO) => {
                println!("You lose!");
                return;
            }
            _ => {
                panic!("invariant violated, player not 1 or 2");
            }
        }
    }
}
