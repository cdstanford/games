/*
    General traits

    - View trait:
      Abstracts the behavior of an object that has both a public and a private view.

    - Game trait:
      Abstracts the behavior of a game with hidden information
      (like Stratego or Battleship)
*/

pub trait View {
    /// Whether two items are equal in ground truth
    fn eq_priv(&self, other: &Self) -> bool;
    /// Whether two items have the same public view
    fn eq_pub(&self, other: &Self) -> bool;
    /// Display ground truth
    fn disp_priv(&self) -> String;
    /// Display public view
    fn disp_pub(&self) -> String;
}

pub enum GameStatus<Player> {
    ToMove(Player),
    Won(Player),
}

pub trait Game {
    type Player;
    type Move;

    /// Who is to move, or (if the game is ended) who has won
    fn status(&self) -> GameStatus<Self::Player>;
    fn is_ended(&self) -> bool {
        match self.status() {
            GameStatus::ToMove(_) => false,
            GameStatus::Won(_) => true,
        }
    }

    /// Whether a move is valid
    fn valid_move(&self, mv: Self::Move) -> bool;

    /// Making the move
    fn make_move(&mut self, mv: Self::Move);
}
