extern crate tictactoe;

pub use tictactoe::board::Board;
pub use tictactoe::error::{self, Result};
pub use tictactoe::{check_for_winner, prompt_for_position};

fn main() {
    let mut board = Board::new();
    loop {
        println!("\n{}", board);

        prompt_for_position()
            .and_then(|position| board.play_x(position))
            .and_then(|outcome| check_for_winner(outcome, &board))
            .and_then(|_| board.play_o())
            .and_then(|outcome| check_for_winner(outcome, &board))
            .or_else(|error| -> Result<()> {
                println!("\n{}", error);
                Ok(())
            }).unwrap();
    }
}

