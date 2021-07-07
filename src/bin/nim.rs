use games::nim::NimState;
use games::play;

fn main() {
    println!("======= NIM =======");
    play::play_vs_yourself::<NimState<2>, 2>();
}
