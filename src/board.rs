use Tile;
use Player;
use std::fmt;
use Result;

pub struct Board {
    tiles: [Tile; 9],
}

pub enum GameOutcome {
    Winner(Player),
    Draw,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    Tile::new(0), Tile::new(1), Tile::new(2),
                    Tile::new(3), Tile::new(4), Tile::new(5),
                    Tile::new(6), Tile::new(7), Tile::new(8),
            ],
        }
    }

    pub fn play_x(&mut self, position: u8) -> Result<()> {
        self.tiles[position as usize].play_x()
    }

    pub fn play_o(&mut self) {
        self.pick_empty_tile().map(|tile| {
            tile.play_o();
        });
    }

    fn pick_empty_tile(&mut self) -> Option<&mut Tile> {
        self.tiles.iter_mut().find(|ref t| t.owner.is_none())
    }

    fn winner(&self) -> Option<Player> {
        const WINNING_POSITIONS: [[usize; 3]; 8] = [
            #[cfg_attr(rustfmt, rustfmt_skip)]
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
        WINNING_POSITIONS.iter()
            .map(|winning_position| {
                [self.tiles[winning_position[0]].owner,
                 self.tiles[winning_position[1]].owner,
                 self.tiles[winning_position[2]].owner]
            })
            .find(|owners| {
                owners.iter()
                    .all(|owner| owner.is_some() && *owner == owners[0])
            })
            .map(|owners| owners[0].unwrap())
    }

    fn is_draw(&self) -> bool {
        self.tiles.iter().all(|t| t.owner.is_some()) && self.winner().is_none()
    }

    pub fn outcome(&self) -> Option<GameOutcome> {
        if self.winner().is_some() {
            Some(GameOutcome::Winner(self.winner().unwrap()))
        } else if self.is_draw() {
            Some(GameOutcome::Draw)
        } else {
            None
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}",
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
