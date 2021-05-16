/*
    The game logic
*/

use crate::board::Board;
// use crate::traits::View;

#[derive(Debug)]
pub enum Player {
    One,
    Two,
}

#[derive(Debug)]
pub struct ShipToPlace {
    length: usize,
}

#[derive(Debug)]
pub struct Game {
    to_move: Player,
    pending_placement1: Vec<ShipToPlace>,
    pending_placement2: Vec<ShipToPlace>,
    board1: Board,
    board2: Board,
}
