# games

Rust implementation of a game interface and some example games.
Just for fun.
Run `cargo run --bin` for the games you can play.

## Dependencies

**Rust version:** This code uses const generics. So requires Rust at least `1.51` (or `cargo +nightly`).

## Games implemented

- [Battleship](https://en.wikipedia.org/wiki/Battleship_(game))

- [Nim](https://en.wikipedia.org/wiki/Nim)

### Adding a game

To add a game:

- Make a new folder in `src` that contains a module for your game, with a `mod.rs` file.
  Add your module (folder name) to `lib.rs`

- Make a struct for your game which implements the AbstractGame trait.
  If your game is simple, you can put the whole game in `mod.rs`.

- Make a binary for your game in `bin` which calls in to your module,
  using one of the functions in the `play` module.
