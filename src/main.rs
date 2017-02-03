use std::io::{self, Write};
use std::fmt;
use std::process;

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    loop {
        print!("\nWhere you would like to go? Press 0-8: ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let position: u8 = buffer.trim().parse().expect("invalid position");

        board.play_x(position);
        if board.winner().is_some() {
            println!("\n{}", board);
            println!("\nWinner: {}", board.winner().unwrap());
            process::exit(0);
        }
        board.play_o();
        if board.winner().is_some() {
            println!("\n{}", board);
            println!("\nWinner: {}", board.winner().unwrap());
            process::exit(0);
        }
        println!("\n{}", board);
    }
}

#[derive(PartialEq)]
enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Tile {
    owner: Option<Player>,
    position: u8,
}

impl Tile {
    fn new(position: u8) -> Tile {
        Tile {
            position: position,
            owner: None,
        }
    }

    fn play_x(&mut self) {
        self.owner = Some(Player::X);
    }

    fn play_o(&mut self) {
        self.owner = Some(Player::O);
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.owner {
            None => write!(f, "{}", self.position),
            Some(Player::X) => write!(f, "X"),
            Some(Player::O) => write!(f, "O"),
        }
    }
}

struct Board {
    tiles: [Tile; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            tiles: [
                Tile::new(0), Tile::new(1), Tile::new(2),
                Tile::new(3), Tile::new(4), Tile::new(5),
                Tile::new(6), Tile::new(7), Tile::new(8),
            ],
        }
    }

    fn play_x(&mut self, position: u8) {
        let tile = &mut self.tiles[position as usize];
        if tile.owner == None {
            tile.play_x();
        } else {
            panic!("tile already occupied");
        }
    }

    fn play_o(&mut self) {
        let tile = self.pick_empty_tile();
        tile.play_o();
    }

    fn pick_empty_tile(&mut self) -> &mut Tile {
        let tile: &mut Tile = self.tiles
            .iter_mut()
            .find(|ref t| t.owner == None)
            .expect("Draw!");
        tile
    }

    fn has_winner(&self) -> bool {
        let winning_positions = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
        winning_positions.iter().any(|&winning_position| {
            let tiles = [
                &self.tiles[winning_position[0]],
                &self.tiles[winning_position[1]],
                &self.tiles[winning_position[2]],
                ];
            (tiles[0].owner != None &&
             tiles[1].owner != None &&
             tiles[2].owner != None
            ) && (
                tiles[0].owner == tiles[1].owner &&
                tiles[1].owner == tiles[2].owner
                )
        })
    }

    fn winner(&self) -> Option<Player> {
        let winning_positions = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
        for winning_position in winning_positions.iter() {
            let tiles = [
                &self.tiles[winning_position[0]],
                &self.tiles[winning_position[1]],
                &self.tiles[winning_position[2]],
                ];
            if (tiles[0].owner != None &&
             tiles[1].owner != None &&
             tiles[2].owner != None
            ) && (
                tiles[0].owner == tiles[1].owner &&
                tiles[1].owner == tiles[2].owner
                ) {
                return match tiles[0].owner {
                    Some(Player::X) => Some(Player::X),
                    Some(Player::O) => Some(Player::O),
                    None => None,
                }
            }
        }
        None
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}",
               self.tiles[0],
               self.tiles[1],
               self.tiles[2],
               self.tiles[3],
               self.tiles[4],
               self.tiles[5],
               self.tiles[6],
               self.tiles[7],
               self.tiles[8],
               )
    }
}
