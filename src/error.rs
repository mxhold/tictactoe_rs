use std::fmt;
use std::result;

#[derive(Debug)]
pub enum BadMoveError {
    InvalidPosition,
    AlreadyOccupied,
}

impl fmt::Display for BadMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BadMoveError::InvalidPosition => write!(f, "Invalid position"),
            BadMoveError::AlreadyOccupied => write!(f, "Position already occupied"),
        }
    }
}

pub type Result<T> = result::Result<T, BadMoveError>;
