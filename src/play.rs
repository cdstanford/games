/*
    Code to play (execute) a game
*/

use super::abstract_game::{AbstractGame, Ai, GameStatus};
use super::player::Player;
use super::util;

use std::fmt::Display;

/*
    Code to play (execute) a game
*/

/// Execute the game where you play the move for every player
/// N is the number of players
pub fn play_vs_yourself<G, const N: usize>()
where
    G: AbstractGame<N>,
    G::Move: Display,
{
    let mut game = G::new();
    loop {
        match game.status() {
            GameStatus::ToMove(plyr) => {
                println!("===== {}'s turn =====", plyr);
                println!("{}", game.print_state_visible(plyr));

                let query = game.query();
                let mv = util::from_user_input_parsing(&query, |raw| {
                    game.parse_valid_move(&raw)
                });
                debug_assert!(game.is_valid_move(&mv));

                println!("Move chosen: {}", mv);
                game.make_move(mv);
            }
            GameStatus::Won(plyr) => {
                println!("{} wins!", plyr);
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
    G::Move: Display,
    A: Ai<G, N>,
{
    let mut game = G::new();
    let mut ai = A::new();
    loop {
        match game.status() {
            GameStatus::ToMove(plyr) => {
                if plyr == you {
                    println!("===== Your turn =====");
                    println!("{}", game.print_state_visible(you));

                    let query = game.query();
                    let mv = util::from_user_input_parsing(&query, |raw| {
                        game.parse_valid_move(&raw)
                    });
                    debug_assert!(game.is_valid_move(&mv));

                    println!("Your move: {}", mv);
                    game.make_move(mv);
                } else {
                    println!("===== Computer {}'s turn =====", plyr);
                    let mv = ai.ai_move(&game, plyr);
                    debug_assert!(game.is_valid_move(&mv));
                    println!("Computer {}'s move: {}", plyr, mv);
                    game.make_move(mv);
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
    G::Move: Display,
    A: Ai<G, N>,
{
    play_vs_ai::<G, A, N>(
        Player::from_index(0)
            .expect("Error: tried to play a game with 0 players"),
    );
}

// /// Play the game vs AIs, where you choose what player to play
pub fn play_vs_ai_choose_player<G, A, const N: usize>()
where
    G: AbstractGame<N>,
    G::Move: Display,
    A: Ai<G, N>,
{
    let query = format!("Choose a player between 1 and {}: ", N);
    let requery = format!("Not between 1 and {}. Try again: ", N);
    let player = util::from_user_input(&query, &requery);
    play_vs_ai::<G, A, N>(player);
}
