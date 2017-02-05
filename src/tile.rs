use Player;
use Result;
use BadMoveError;
use std::fmt;

pub struct Tile {
    pub owner: Option<Player>,
    position: u8,
}

impl Tile {
    pub fn new(position: u8) -> Tile {
        Tile {
            position: position,
            owner: None,
        }
    }

    pub fn play_x(&mut self) -> Result<()> {
        self.play(Player::X)
    }

    pub fn play_o(&mut self) -> Result<()> {
        self.play(Player::O)
    }

    pub fn play(&mut self, player: Player) -> Result<()> {
        match self.owner {
            Some(_) => Err(BadMoveError::AlreadyOccupied),
            None => {
                self.owner = Some(player);
                Ok(())
            }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.owner {
            None => write!(f, "{}", self.position),
            Some(Player::X) => write!(f, "X"),
            Some(Player::O) => write!(f, "O"),
        }
    }
}
