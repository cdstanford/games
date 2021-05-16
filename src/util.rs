/*
    Utility
*/

use std::io::{self, BufRead};

pub fn user_input(query: &str) -> String {
    print!("{}", query);
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    line_iter
        .next()
        .expect("failed to get line from stdin")
        .expect("failed to get line from stdin")
}
