/*
    Type for a fixed finite number of players
*/

use super::util::FromStrHelp;

use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

/// Struct representing a player (player 0, player 1, etc.)
/// (not with respect to any specific game)
/// N is the number of players
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Player<const N: usize>(usize);
impl<const N: usize> Player<N> {
    /// Internal: invariant check
    fn is_valid(&self) -> bool {
        self.0 < N
    }

    /// Constructor from usize
    /// A player should only be created through this method, not directly
    pub fn from_index(n: usize) -> Option<Self> {
        let result = Self(n);
        if result.is_valid() {
            Some(result)
        } else {
            None
        }
    }

    /// Convert back to a usize
    pub fn as_index(&self) -> usize {
        debug_assert!(self.is_valid());
        self.0
    }

    /// Cycle between players
    pub fn next_player(&self) -> Self {
        debug_assert!(self.is_valid());
        Self::from_index((self.0 + 1) % N).unwrap()
    }

    /// Human-readable name -- lower case and upper case versions
    /// Note: human-readable names start from 1 instead of 0
    pub fn name_lower(&self) -> String {
        debug_assert!(self.is_valid());
        format!("player {}", self.0 + 1)
    }
    pub fn name_upper(&self) -> String {
        debug_assert!(self.is_valid());
        format!("Player {}", self.0 + 1)
    }
}

impl<const N: usize> Display for Player<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_assert!(self.is_valid());
        write!(f, "{}", self.name_upper())
    }
}

/// Custom error struct for parsing player from string
#[derive(Debug)]
pub enum ParsePlayerErr {
    NotUsize(ParseIntError),
    IndexZero,
    IndexTooLarge(usize),
}
impl From<ParseIntError> for ParsePlayerErr {
    fn from(err: ParseIntError) -> Self {
        Self::NotUsize(err)
    }
}
impl Display for ParsePlayerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUsize(x) => {
                write!(f, "invalid integer ({})", x)
            }
            Self::IndexZero => {
                write!(f, "player number must be > 0")
            }
            Self::IndexTooLarge(x) => {
                write!(f, "player number too large: {}", x)
            }
        }
    }
}
impl<const N: usize> FromStr for Player<N> {
    type Err = ParsePlayerErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let player_num = s.parse::<usize>()?;
        if player_num == 0 {
            Err(ParsePlayerErr::IndexZero)
        } else if player_num > N {
            Err(ParsePlayerErr::IndexTooLarge(player_num))
        } else {
            Ok(Self::from_index(player_num - 1).unwrap())
        }
    }
}

impl<const N: usize> FromStrHelp for Player<N> {
    fn query() -> String {
        format!("Choose a player between 1 and {}: ", N)
    }
    fn from_str_help(s: &str) -> Result<Self, String> {
        if let Ok(player_num) = s.parse::<usize>() {
            if player_num == 0 || player_num > N {
                Err(format!("Not beteween 1 and {}.", N))
            } else {
                Ok(Self::from_index(player_num - 1).unwrap())
            }
        } else {
            Err("Not an integer.".to_string())
        }
    }
    fn help() -> Option<String> {
        None
    }
}

/// Functionality specific to two players
pub type TwoPlayers = Player<2>;
impl TwoPlayers {
    pub const ONE: Self = Self(0);
    pub const TWO: Self = Self(1);
    pub fn opponent(&self) -> Self {
        self.next_player()
    }
    pub fn from_bool(b: bool) -> Self {
        if b {
            Self::TWO
        } else {
            Self::ONE
        }
    }
    pub fn as_bool(&self) -> bool {
        match self.0 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ONE: TwoPlayers = TwoPlayers::ONE;
    const TWO: TwoPlayers = TwoPlayers::TWO;
    const INVALID1: TwoPlayers = Player(2);
    const INVALID2: TwoPlayers = Player(3);

    #[test]
    fn test_is_valid() {
        assert!(ONE.is_valid());
        assert!(TWO.is_valid());
        assert!(!INVALID1.is_valid());
        assert!(!INVALID2.is_valid());
    }

    #[test]
    fn test_from_index() {
        assert_eq!(TwoPlayers::from_index(0), Some(ONE));
        assert_eq!(TwoPlayers::from_index(1), Some(TWO));
        assert_eq!(TwoPlayers::from_index(2), None);
        assert_eq!(TwoPlayers::from_index(3), None);
    }

    #[test]
    fn test_as_index() {
        assert_eq!(ONE.as_index(), 0);
        assert_eq!(TWO.as_index(), 1);
    }

    #[test]
    fn test_next_player() {
        assert_eq!(ONE.next_player(), TWO);
        assert_eq!(ONE.opponent(), TWO);
        assert_eq!(TWO.next_player(), ONE);
        assert_eq!(TWO.opponent(), ONE);
    }

    #[test]
    fn test_name() {
        assert_eq!(&ONE.name_upper(), "Player 1");
        assert_eq!(&ONE.name_lower(), "player 1");
        assert_eq!(&TWO.name_upper(), "Player 2");
        assert_eq!(&TWO.name_lower(), "player 2");
    }

    #[test]
    fn test_to_from_bool() {
        assert_eq!(ONE.as_bool(), false);
        assert_eq!(TwoPlayers::from_bool(false), ONE);
        assert_eq!(TWO.as_bool(), true);
        assert_eq!(TwoPlayers::from_bool(true), TWO);
    }
}
