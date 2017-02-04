use std::io::{self, Write};
use std::fmt;
use std::process;
use std::num::ParseIntError;

// 43278 => Draw

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    loop {
        let position = match prompt_for_position() {
            Ok(position) => position,
            Err(_) => {
                println!("\nInvalid position");
                continue;
            },
        };

        match board.play_x(position) {
            Ok(_) => {
                check_for_winner(&board);

                board.play_o();
                check_for_winner(&board);

                println!("\n{}", board);
            },
            Err(message) => {
                println!("\n{}", message);
            },
        }
    }
}

fn prompt_for_position() -> Result<u8, ParseIntError> {
    print!("\nWhere you would like to go? Press 0-8: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse()
}

fn check_for_winner(board: &Board) {
    match board.outcome() {
        Some(GameOutcome::Winner(player)) => {
            println!("\n{}", board);
            println!("\nWinner: {}", player);
            process::exit(0);
        },
        Some(GameOutcome::Draw) => {
            println!("\n{}", board);
            println!("\nDraw!");
            process::exit(0);
        },
        None => (),
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

enum GameOutcome {
    Winner(Player),
    Draw,
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
        let tile = self.tiles.get_mut(position as usize);
        match tile {
            Some(tile) => tile.play_x(),
            None => Err("Invalid position"),
        }
    }

    fn play_o(&mut self) {
        let tile = self.pick_empty_tile_mut();
        if tile.is_some() {
            tile.unwrap().play_o();
        }
    }

    fn pick_empty_tile(&self) -> Option<&Tile> {
        self.tiles
            .iter()
            .find(|ref t| t.owner == None)
    }

    fn pick_empty_tile_mut(&mut self) -> Option<&mut Tile> {
        self.tiles
            .iter_mut()
            .find(|ref t| t.owner == None)
    }

    fn winner(&self) -> Option<Player> {
        const WINNING_POSITIONS: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
        WINNING_POSITIONS.iter().map(|winning_position| {
            [
                self.tiles[winning_position[0]].owner,
                self.tiles[winning_position[1]].owner,
                self.tiles[winning_position[2]].owner,
            ]
        }).find(|owners| {
            owners.iter().all(|owner| owner.is_some() && *owner == owners[0])
        }).map(|owners| owners[0].unwrap())
    }

    fn is_draw(&self) -> bool {
        let tile = self.pick_empty_tile();
        tile.is_none() && self.winner().is_none()
    }

    fn outcome(&self) -> Option<GameOutcome> {
        if self.winner().is_some() {
            Some(GameOutcome::Winner(self.winner().unwrap()))
        } else if self.is_draw() {
            Some(GameOutcome::Draw)
        } else {
            None
        }
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
