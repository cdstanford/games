/*
    Abstract trait for a game which makes as few assumptions as possible:
    there can be any number of players and there are no assumptions
    about the precise amount of state available to each player.

    Particular types of games (game with complete info, partial info,
    or randomness) are special cases of this trait.

    Additionally, the GameStatus struct is used to indicate the state of a
    game (whether in progress or ended).
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStatus<Player> {
    ToMove(Player),
    Won(Player),
}

pub trait AbstractGame {
    type Player;
    type Move;

    /*
        Provided methods
    */

    /// Number of players -- should be constant for any instance of the type
    /// (Generally constant for the type as well, but not necessarily)
    // fn num_players(&self) -> usize;
    // TODO

    /// Starting position
    fn new() -> Self;

    /// Who is to move, or (if the game is ended) who has won
    fn status(&self) -> GameStatus<Self::Player>;

    /// Whether a move is valid
    fn valid_move(&self, plyr: Self::Player, mv: &Self::Move) -> bool;

    /// Making the move -- ok to assume that it is valid
    fn make_move(&mut self, plyr: Self::Player, mv: Self::Move);

    /// Game state visible to a particular player
    fn print_state_visible(&self, plyr: Self::Player) -> String;

    /*
        Derived functionality
    */

    /// Whether the game has ended
    fn is_ended(&self) -> bool {
        match self.status() {
            GameStatus::ToMove(_) => false,
            GameStatus::Won(_) => true,
        }
    }
}

pub trait Ai<G: AbstractGame> {
    /// Initialize
    fn new() -> Self;

    /// Given a state, choose a valid move
    /// This must satisfy: (1) only uses information that is
    /// be available to that player; (2) the returned move should be valid
    fn ai_move(&mut self, game: &G, plyr: G::Player) -> G::Move;
}
