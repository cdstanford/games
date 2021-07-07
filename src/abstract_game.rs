/*
    Abstract trait for a game which makes as few assumptions as possible:
    there can be any number of players and there are no assumptions
    about the precise amount of state available to each player.

    Particular types of games (game with complete info, partial info,
    or randomness) can be considered special cases of this trait.
*/

use super::player::Player;

/// Type to indicate the state of a game (whether in progress or ended)
/// N is the number of players
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStatus<const N: usize> {
    ToMove(Player<N>),
    Won(Player<N>),
}

/// Main game trait
/// N is the number of players
/// Note: N should be >= 0, but we don't really have to enforce this. What happens
/// if N = 0 is that it's impossible to implement `fn status()` since GameStatus
/// is uninhabited.
pub trait AbstractGame<const N: usize> {
    type Move: Eq;

    /*
        Provided methods
    */

    /// Starting position
    fn new() -> Self;

    /// Who is to move, or (if the game is ended) who has won
    fn status(&self) -> GameStatus<N>;

    /// Given a move, return whether or not it is valid
    fn is_valid_move(&self, mv: &Self::Move) -> bool;

    /// Making the move -- ok to assume that it is valid
    fn make_move(&mut self, mv: Self::Move);

    /// Query to the user to make a move
    fn query(&self) -> String;

    /// Parsing the move from a string.
    /// On error, print a helpful error message.
    /// It is recommended to also error in case of an invalid move, so that the
    /// error message will be more specific.
    fn parse_move(&self, raw: &str) -> Result<Self::Move, String>;

    /// Game state visible to a particular player
    fn print_state_visible(&self, plyr: Player<N>) -> String;

    /*
        Derived functionality
    */

    /// Parse a move and print "invalid string" if the move is invalid
    fn parse_valid_move(&self, raw: &str) -> Result<Self::Move, String> {
        self.parse_move(raw).and_then(|mv| {
            if self.is_valid_move(&mv) {
                Ok(mv)
            } else {
                Err("Invalid move".to_string())
            }
        })
    }

    /// Number of players in the game
    fn num_players(&self) -> usize {
        N
    }

    /// Whether the game has ended
    fn is_ended(&self) -> bool {
        match self.status() {
            GameStatus::ToMove(_) => false,
            GameStatus::Won(_) => true,
        }
    }

    /// Current player (if not ended)
    fn cur_player(&self) -> Option<Player<N>> {
        match self.status() {
            GameStatus::ToMove(plyr) => Some(plyr),
            GameStatus::Won(_) => None,
        }
    }
}

pub trait Ai<G, const N: usize>
where
    G: AbstractGame<N>,
{
    /// Initialize
    fn new() -> Self;

    /// Given a state, choose a valid move
    /// This must satisfy: (1) only uses information that is
    /// be available to that player; (2) the returned move should be valid
    fn ai_move(&mut self, game: &G, plyr: Player<N>) -> G::Move;
}
