use games::battleship::game::{GameState, UnimplementedBattleshipAi};
use games::play;

fn main() {
    println!("======= BATTLESHIP =======");
    play::play_vs_ai_as_p1::<GameState, UnimplementedBattleshipAi, 2>();
}
