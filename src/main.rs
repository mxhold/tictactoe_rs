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
        if board.has_winner() {
            println!("\n{}", board);
            println!("\nWinner: {}", board.winner());
            process::exit(0);
        }
        board.play_o();
        if board.has_winner() {
            println!("\n{}", board);
            println!("\nWinner: {}", board.winner());
            process::exit(0);
        }
        println!("\n{}", board);
    }
}

#[derive(PartialEq)]
enum TileOwner {
    Empty,
    X,
    O,
}

struct Tile {
    owner: TileOwner,
    position: u8,
}

impl Tile {
    fn new(position: u8) -> Tile {
        Tile {
            position: position,
            owner: TileOwner::Empty,
        }
    }

    fn play_x(&mut self) {
        self.owner = TileOwner::X;
    }

    fn play_o(&mut self) {
        self.owner = TileOwner::O;
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.owner {
            TileOwner::Empty => write!(f, "{}", self.position),
            TileOwner::X => write!(f, "X"),
            TileOwner::O => write!(f, "O"),
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
        if tile.owner == TileOwner::Empty {
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
            .find(|ref t| t.owner == TileOwner::Empty)
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
            (tiles[0].owner != TileOwner::Empty &&
             tiles[1].owner != TileOwner::Empty &&
             tiles[2].owner != TileOwner::Empty
            ) && (
                tiles[0].owner == tiles[1].owner &&
                tiles[1].owner == tiles[2].owner
                )
        })
    }

    fn winner(&self) -> String {
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
            if (tiles[0].owner != TileOwner::Empty &&
             tiles[1].owner != TileOwner::Empty &&
             tiles[2].owner != TileOwner::Empty
            ) && (
                tiles[0].owner == tiles[1].owner &&
                tiles[1].owner == tiles[2].owner
                ) {
                return match tiles[0].owner {
                    TileOwner::X => "X".to_string(),
                    TileOwner::O => "O".to_string(),
                    TileOwner::Empty => unreachable!(),
                }
            }
        }
        unreachable!()
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
