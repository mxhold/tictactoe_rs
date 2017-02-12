use std::io::{self, Write};
use std::process;

use board::{Board, GameOutcome};
use error::{BadMoveError, Result};
use tile::Tile;
use player::Player;

pub mod board;
pub mod error;
mod tile;
pub mod player;

pub fn prompt_for_position() -> Result<u8> {
    print!("\nWhere you would like to go? Press 0-8: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    match buffer.trim().parse() {
        Ok(position @ 0...8) => Ok(position),
        _ => Err(BadMoveError::InvalidPosition),
    }
}

pub fn check_for_winner(outcome: Option<GameOutcome>, board: &Board) -> Result<()> {
    match outcome {
        Some(GameOutcome::Winner(player)) => {
            println!("\n{}", board);
            match player {
                Player::X => println!("\nYou won!"),
                Player::O => println!("\nYou lost."),
            }
            process::exit(0);
        }
        Some(GameOutcome::Draw) => {
            println!("\n{}", board);
            println!("\nDraw.");
            process::exit(0);
        }
        None => {
            Ok(())
        }
    }
}

