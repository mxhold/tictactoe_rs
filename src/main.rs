use std::io::{self, Write};
use std::process;

mod board;
use board::{Board, GameOutcome};

mod tile;
use tile::Tile;

mod player;
use player::Player;

mod error;
use error::{BadMoveError, Result};

fn main() {
    let mut board = Board::new();
    loop {
        println!("\n{}", board);

        match prompt_for_position().and_then(|position| board.play_x(position)) {
            Ok(_) => {
                check_for_winner(&board);
                board.play_o();
                check_for_winner(&board);
            }
            Err(error) => println!("\n{}", error),
        }
    }
}

fn prompt_for_position() -> Result<u8> {
    print!("\nWhere you would like to go? Press 0-8: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    match buffer.trim().parse() {
        Ok(position @ 0...8) => Ok(position),
        _ => Err(BadMoveError::InvalidPosition),
    }
}

fn check_for_winner(board: &Board) {
    match board.outcome() {
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
        None => (),
    }
}
