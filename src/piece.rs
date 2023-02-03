use structopt::clap::arg_enum;

use std::{fmt::{self, Display}};


arg_enum!{
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Team {
    Muscovites,
    Swedes,
}
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Piece {
    King,
    Muscovite,
    Swede,
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Piece::Swede => "S",
            Piece::Muscovite => "M",
            Piece::King => "K",
        };
        write!(f, "{}", s)
    }
}

impl Piece {
    pub fn get_team(self) -> Team {
        match self {
            Piece::King => Team::Swedes,
            Piece::Muscovite => Team::Muscovites,
            Piece::Swede => Team::Swedes,
        }
    }
}
