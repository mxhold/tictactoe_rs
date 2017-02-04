use std::io::{self, Write};
use std::fmt;
use std::process;

fn abort(message: &'static str) {
    println!("{}", message);
    process::exit(1);
}

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    loop {
        let position = prompt_for_position();

        let res: Result<(), &'static str> = board.play_x(position);
        res.unwrap_or_else(abort);
        check_for_winner(&board);

        board.play_o().unwrap();
        check_for_winner(&board);

        println!("\n{}", board);
    }
}

fn prompt_for_position() -> u8 {
    print!("\nWhere you would like to go? Press 0-8: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().expect("invalid position")
}

fn check_for_winner(board: &Board) {
    if board.winner().is_some() {
        println!("\n{}", board);
        println!("\nWinner: {}", board.winner().unwrap());
        process::exit(0);
    }
}

#[derive(PartialEq)]
#[derive(Copy,Clone)]
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

    fn play_x(&mut self) -> Result<(), &'static str> {
        match self.owner {
            Some(_) => Err("Tile already occupied"),
            None => Ok({ self.owner = Some(Player::X) }),
        }
    }

    fn play_o(&mut self) -> Result<(), &'static str> {
        match self.owner {
            Some(_) => Err("Tile already occupied"),
            None => Ok({ self.owner = Some(Player::O) }),
        }
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

    fn play_x(&mut self, position: u8) -> Result<(), &'static str>  {
        let tile = &mut self.tiles[position as usize];
        tile.play_x()
    }

    fn play_o(&mut self) -> Result<(), &'static str> {
        let tile = self.pick_empty_tile();
        tile.play_o()
    }

    fn pick_empty_tile(&mut self) -> &mut Tile {
        let tile: &mut Tile = self.tiles
            .iter_mut()
            .find(|ref t| t.owner == None)
            .expect("Draw!");
        tile
    }

    fn winner(&self) -> Option<Player> {
        const winning_positions: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
        winning_positions.iter().map(|winning_position| {
            [
                self.tiles[winning_position[0]].owner,
                self.tiles[winning_position[1]].owner,
                self.tiles[winning_position[2]].owner,
            ]
        }).find(|owners| {
            owners.iter().all(|owner| owner.is_some() && *owner == owners[0])
        }).map(|owners| owners[0].unwrap())
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
