/*
    Code to play (execute) a game
*/

use super::abstract_game::{AbstractGame, Ai, GameStatus};
use super::player::Player;
use super::util;

use std::fmt::Display;
use std::str::FromStr;

/*
    Code to play (execute) a game
*/

/// Execute the game where you play the move for every player
/// N is the number of players
pub fn play_vs_yourself<G, const N: usize>()
where
    G: AbstractGame<N>,
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

/// Play the game using AIs for all the other players
/// N is the number of players and 'you' is your player
pub fn play_vs_ai<G, A, const N: usize>(you: Player<N>)
where
    G: AbstractGame<N>,
    A: Ai<G, N>,
    G::Move: FromStr + Display,
{
    let mut game = G::new();
    let mut ai = A::new();
    loop {
        match game.status() {
            GameStatus::ToMove(plyr) => {
                if plyr == you {
                    println!("===== Your turn =====");
                    println!("{}", game.print_state_visible(you));
                    let mv = util::from_user_input_satisfying(
                        "Move: ",
                        "Invalid syntax, try again: ",
                        "Invalid move, try again: ",
                        |mv| game.valid_move(you, mv),
                    );
                    debug_assert!(game.valid_move(you, &mv));
                    println!("Your move: {}", mv);
                    game.make_move(you, mv);
                } else {
                    println!("===== Computer {}'s turn =====", plyr);
                    let mv = ai.ai_move(&game, plyr);
                    debug_assert!(game.valid_move(plyr, &mv));
                    println!("Computer {}'s move: {}", plyr, mv);
                    game.make_move(plyr, mv);
                }
            }
            GameStatus::Won(plyr) => {
                if plyr == you {
                    println!("You win!");
                } else {
                    println!("You lose! Player {} wins.", plyr);
                }
                return;
            }
        }
    }
}

/// Play the game vs AIs, where you are player 1
/// Panics if N = 0, but N = 0 should not really be possible (see
/// comment in abstract_game.rs)
pub fn play_vs_ai_as_p1<G, A, const N: usize>()
where
    G: AbstractGame<N>,
    A: Ai<G, N>,
    G::Move: FromStr + Display,
{
    play_vs_ai::<G, A, N>(
        Player::from_index(0)
            .expect("Error: tried to play a game with 0 players"),
    );
}
