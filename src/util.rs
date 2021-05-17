/*
    Utility
*/

use std::io::{self, BufRead};
use std::str::FromStr;

pub fn user_input(query: &str) -> String {
    print!("{}", query);
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    line_iter
        .next()
        .expect("failed to get line from stdin")
        .expect("failed to get line from stdin")
}

pub fn from_user_input<T: FromStr>(query: &str, query_again: &str) -> T {
    let mut result = user_input(query).parse().ok();
    while result.is_none() {
        result = user_input(query_again).parse().ok();
    }
    result.unwrap()
}

pub fn from_user_input_satisfying<T, F>(
    query: &str,
    query_again: &str,
    query_invalid: &str,
    predicate: F,
) -> T
where
    T: FromStr,
    F: Fn(&T) -> bool,
{
    let mut result = from_user_input(query, query_again);
    while !predicate(&result) {
        result = from_user_input(query_invalid, query_again);
    }
    result
}
