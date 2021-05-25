/*
    Interfaces for the behavior of a game with hidden information.

    Examples of such games include Stratego and Battleship.

    TODO: complete this, specializing AbstractGame?
    Or delete
*/

use super::abstract_game::AbstractGame;

pub trait PartialInfoGame<const N: usize>: AbstractGame<N> {
    /* Additional methods */
    // ???
}

// Do this instead:
// impl AbstractGame for PartialInfoGame {
// }
