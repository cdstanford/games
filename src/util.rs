/*
    Utility

    - Getting user input from stdin

    - String parsing functions
*/

use std::io::{self, BufRead, Write};
use std::str::FromStr;

/*
    Getting user input
*/

pub fn user_input(query: &str) -> String {
    print!("{}", query);
    io::stdout().flush().expect("failed to flush stdout");
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    line_iter
        .next()
        .expect("failed to get line from stdin")
        .expect("failed to get line from stdin")
}

/// The FromStrHelp trait
///
/// This is analagous to FromStr in the standard library,
/// but instead of just including a custom error type it requires
/// the user to implement a help string which says what went wrong.
///
/// Notice that from_str_help returns Result<T, String> instead of
/// Result<T, Err> for a custom type Err.
/// The motivation is that when getting user input, having a structured
/// error type (explicitly enumerated fail modes) is less useful because
/// the only use for this type is to tell the user what went wrong, not
/// to catch and respond to errors in a more general context. Also,
/// the Display implementation for this error type may not be very
/// specifically helpful.
///
/// An alternate design would be to have FromStrHelp require FromStr,
/// and remove from_str_help here.
/// For users who only need FromStrHelp this would be more work,
/// but this would be more convenient for some types which already
/// have a good implementation of FromStr.
pub trait FromStrHelp: Sized {
    /// Initial query to the user
    /// This should end in ": " or similar
    fn query() -> String;
    /// Parse the user input.
    /// If invalid, return a description of what went wrong.
    /// If Self: FromStr, this can be implemented as something like
    /// s.parse().map_err(|err| format!("{}", err))
    fn from_str_help(s: &str) -> Result<Self, String>;
    /// ... and provide detailed help text to display, if needed for this type
    fn help() -> Option<String>;
    /// ... and finally, optionally override the 'try again' query text
    fn requery() -> String {
        "Try again: ".to_string()
    }
}

fn query_once<T: FromStrHelp>(query: &str) -> Result<T, String> {
    T::from_str_help(&user_input(query))
}

pub fn from_user_input<T: FromStrHelp>() -> T {
    let mut result = query_once(&T::query());
    while let Err(err) = result {
        println!("{}", err);
        if let Some(help) = T::help() {
            println!("{}", help);
        }
        result = query_once(&T::requery());
    }
    result.unwrap()
}

pub fn from_user_input_satisfying<T, F>(predicate: F) -> T
where
    T: FromStrHelp,
    F: Fn(&T) -> bool,
{
    let mut result = from_user_input(query, query_again);
    while !predicate(&result) {
        result = from_user_input(query_invalid, query_again);
    }
    result
}

/*
    Parsing functions
*/

pub fn parse_vec<T: FromStr>(raw: &str) -> Option<Vec<T>> {
    raw.trim_matches(|p| p == '(' || p == ')' || p == ',')
        .split(' ')
        .map(|s| s.parse::<T>().ok())
        .collect()
}

pub fn parse_vec_usize(raw: &str) -> Option<Vec<usize>> {
    parse_vec(raw)
}

pub fn parse_vec_isize(raw: &str) -> Option<Vec<isize>> {
    parse_vec(raw)
}
