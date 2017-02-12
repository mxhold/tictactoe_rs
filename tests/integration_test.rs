extern crate tictactoe;
use tictactoe::board::{Board, GameOutcome};
use tictactoe::player::Player;

#[test]
fn win() {
    let mut board = Board::new();

    board.play_x(6).unwrap();
    board.play_o().unwrap();
    board.play_x(7).unwrap();
    board.play_o().unwrap();
    let outcome = board.play_x(8).unwrap().unwrap();
    assert_eq!(outcome, GameOutcome::Winner(Player::X));
}

#[test]
fn lose() {
    let mut board = Board::new();

    board.play_x(3).unwrap();
    board.play_o().unwrap();
    board.play_x(4).unwrap();
    board.play_o().unwrap();
    board.play_x(6).unwrap();
    let outcome = board.play_o().unwrap().unwrap();
    assert_eq!(outcome, GameOutcome::Winner(Player::O));
}

#[test]
fn draw() {
    let mut board = Board::new();

    board.play_x(4).unwrap();
    board.play_o().unwrap();
    board.play_x(3).unwrap();
    board.play_o().unwrap();
    board.play_x(2).unwrap();
    board.play_o().unwrap();
    board.play_x(7).unwrap();
    board.play_o().unwrap();
    let outcome = board.play_x(8).unwrap().unwrap();
    assert_eq!(outcome, GameOutcome::Draw);
}
