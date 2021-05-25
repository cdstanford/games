use games::battleship::game::{GameState, UnimplementedBattleshipAi};
use games::play;

fn main() {
    println!("======= BATTLESHIP =======");
    play::play_vs_ai_choose_player::<GameState, UnimplementedBattleshipAi, 2>();
}
