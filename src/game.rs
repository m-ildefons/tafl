use std::{io, fmt::{self, Display}};
use structopt::clap::arg_enum;


// pub const SIZE_ALEA_EVANGELII: u16 = 19;
// pub const SIZE_ARD_RI: u16 = 7;
// pub const SIZE_BRANDUBH: u16 = 7;
pub const SIZE_HNEFATAFL_11: u16 = 11;
pub const SIZE_HNEFATAFL_13: u16 = 13;
pub const SIZE_TABLUT: u16 = 9;
// pub const SIZE_TAWLBWRRD: u16 = 11;


pub type Coord = (usize, usize);


arg_enum!{
#[derive(Debug, Clone, Copy)]
pub enum Rule {
    Hnefatafl11,
    Hnefatafl13,
    Tablut,
}
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Team {
    Muscovites,
    Swedes,
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


#[derive(Clone, Copy)]
pub enum Status {
    Corner,
    Castle,
}

#[derive(Clone, Copy)]
pub struct Square {
    pub piece: Option<Piece>,
    pub status: Option<Status>,
}


impl Square {
    pub fn new() -> Self {
        Self{
            piece: None,
            status: None,
        }
    }
}


pub struct Board {
    pub board: Vec<Vec<Square>>,
    pub size: u16,
    pub rule: Rule,
}


impl Board {
    pub fn new(rule: Rule) -> Self {
        let size = match rule {
            Rule::Hnefatafl11 => SIZE_HNEFATAFL_11,
            Rule::Hnefatafl13 => SIZE_HNEFATAFL_13,
            Rule::Tablut => SIZE_TABLUT,
        };

        let mut board =
            (0..size)
            .map(|_| (0..size)
            .map(|_| Square::new())
            .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mid_of_board = ((size - 1) / 2) as usize;
        board[mid_of_board][mid_of_board].piece = Some(Piece::King);
        board[mid_of_board][mid_of_board].status = Some(Status::Castle);

        board[0][0].status = Some(Status::Corner);
        board[0][(size - 1) as usize].status = Some(Status::Corner);
        board[(size - 1) as usize][0].status = Some(Status::Corner);
        board[(size - 1) as usize][(size - 1) as usize].status = Some(Status::Corner);

        match rule {
            Rule::Hnefatafl11 | Rule::Hnefatafl13 => {
                // Swedes
                board[mid_of_board - 1][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board + 1][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board - 2][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board + 2][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board - 1].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board + 1].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board - 2].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board + 2].piece = Some(Piece::Swede);
                board[mid_of_board - 1][mid_of_board - 1].piece = Some(Piece::Swede);
                board[mid_of_board - 1][mid_of_board + 1].piece = Some(Piece::Swede);
                board[mid_of_board + 1][mid_of_board - 1].piece = Some(Piece::Swede);
                board[mid_of_board + 1][mid_of_board + 1].piece = Some(Piece::Swede);

                //Muscovites
                board[mid_of_board][0].piece = Some(Piece::Muscovite);
                board[mid_of_board][1].piece = Some(Piece::Muscovite);
                board[mid_of_board - 1][0].piece = Some(Piece::Muscovite);
                board[mid_of_board + 1][0].piece = Some(Piece::Muscovite);
                board[mid_of_board - 2][0].piece = Some(Piece::Muscovite);
                board[mid_of_board + 2][0].piece = Some(Piece::Muscovite);

                board[mid_of_board][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board][(size - 2) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board - 1][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board + 1][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board - 2][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board + 2][(size - 1) as usize].piece = Some(Piece::Muscovite);

                board[0][mid_of_board].piece = Some(Piece::Muscovite);
                board[1][mid_of_board].piece = Some(Piece::Muscovite);
                board[0][mid_of_board - 1].piece = Some(Piece::Muscovite);
                board[0][mid_of_board + 1].piece = Some(Piece::Muscovite);
                board[0][mid_of_board - 2].piece = Some(Piece::Muscovite);
                board[0][mid_of_board + 2].piece = Some(Piece::Muscovite);

                board[(size - 1) as usize][mid_of_board].piece = Some(Piece::Muscovite);
                board[(size - 2) as usize][mid_of_board].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board - 1].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board + 1].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board - 2].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board + 2].piece = Some(Piece::Muscovite);
            },
            Rule::Tablut => {
                // Swedes
                board[mid_of_board - 1][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board + 1][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board - 2][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board + 2][mid_of_board].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board - 1].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board + 1].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board - 2].piece = Some(Piece::Swede);
                board[mid_of_board][mid_of_board + 2].piece = Some(Piece::Swede);

                //Muscovites
                board[mid_of_board][0].piece = Some(Piece::Muscovite);
                board[mid_of_board][1].piece = Some(Piece::Muscovite);
                board[mid_of_board - 1][0].piece = Some(Piece::Muscovite);
                board[mid_of_board + 1][0].piece = Some(Piece::Muscovite);

                board[mid_of_board][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board][(size - 2) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board - 1][(size - 1) as usize].piece = Some(Piece::Muscovite);
                board[mid_of_board + 1][(size - 1) as usize].piece = Some(Piece::Muscovite);

                board[0][mid_of_board].piece = Some(Piece::Muscovite);
                board[1][mid_of_board].piece = Some(Piece::Muscovite);
                board[0][mid_of_board - 1].piece = Some(Piece::Muscovite);
                board[0][mid_of_board + 1].piece = Some(Piece::Muscovite);

                board[(size - 1) as usize][mid_of_board].piece = Some(Piece::Muscovite);
                board[(size - 2) as usize][mid_of_board].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board - 1].piece = Some(Piece::Muscovite);
                board[(size - 1) as usize][mid_of_board + 1].piece = Some(Piece::Muscovite);
            },
        }

        Self {
            board,
            rule,
            size,
        }
    }


    pub fn get_piece_at(&mut self, at: Coord) -> Option<Piece> {
        return self.board[at.0][at.1].piece;
    }

    pub fn get_status_at(&mut self, at: Coord) -> Option<Status> {
        return self.board[at.0][at.1].status;
    }

    pub fn move_piece(&mut self, from: Coord, to: Coord) {
        let piece = self.board[from.0][from.1].piece;
        self.board[from.0][from.1].piece = None;
        self.board[to.0][to.1].piece = piece;
    }
}

#[test]
fn test_new_board() -> io::Result<()> {
    let _game = Board::new(Rule::Tablut);
    Ok(())
}


pub struct Game {
    pub rule: Rule,
    pub board: Board,
    pub winner: Option<Team>,
    pub turn: usize,
}

impl Game {
    pub fn new(rule: Rule) -> Self {
        Self {
            rule,
            board: Board::new(rule),
            winner: None,
            turn: 0,
        }
    }

    pub fn move_piece(&mut self, from: Coord, to: Coord) {
        let piece_at = self.board.get_piece_at(from);
        match piece_at {
            Some(piece) => {
                let target_at = self.board.get_piece_at(to);
                match target_at {
                    Some(_) => {},
                    None => {
                        let invalid =
                            (from.0 != to.0) && (from.1 != to.1)
                            || self.check_jump(from, to)
                            || self.check_status(from, to);

                        if ! invalid {
                            self.board.move_piece(from, to);
                            self.check_kill(piece, to);
                            self.check_king_escape(piece, to);
                        }
                    },
                }
                self.turn += 1;
            },
            None => {},
        }
    }

    fn check_status(&mut self, from: Coord, to: Coord) -> bool {
        let piece = self.board.get_piece_at(from);
        let status = self.board.get_status_at(to);

        match status {
            Some(_) => {
                match piece {
                    Some(Piece::King) => { return false },
                    _ => { return true },
                }
            },
            None => { return false },
        }
    }

    fn check_jump(&mut self, from: Coord, to: Coord) -> bool {
        if from.0 < to.0 {
            for i in (from.0 + 1)..(to.0) {
                if self.check_piece((i, from.1)) { return true ; }
            }
        } else if from.0 > to.0 {
            for i in (to.0)..(from.0) {
                if self.check_piece((i, from.1)) { return true ; }
            }
        } else if from.1 < to.1 {
            for i in (from.1 + 1)..(to.1) {
                if self.check_piece((from.0, i)) { return true ; }
            }
        } else if from.1 > to.1 {
            for i in (to.1)..(from.1) {
                if self.check_piece((from.0, i)) { return true ; }
            }
        }
        return false;
    }

    fn check_piece(&mut self, at: Coord) -> bool {
        let s = self.board.get_piece_at(at);
        match s {
            Some(_) => return true,
            None => return false,
        }
    }

    fn check_kill(&mut self, piece: Piece, to: Coord) {
        if to.0 > 1 {
            let to_left = (to.0 - 1, to.1);
            let over_left = (to.0 - 2, to.1);

            if self.check_kill_rule(piece, to_left, over_left) {
                self.board.board[to.0 - 1][to.1].piece = None;
            }
        }

        if to.0 < ((self.board.size - 2) as usize) {
            let to_right = (to.0 + 1, to.1);
            let over_right = (to.0 + 2, to.1);

            if self.check_kill_rule(piece, to_right, over_right) {
                self.board.board[to.0 + 1][to.1].piece = None;
            }
        }

        if to.1 > 1 {
            let to_up = (to.0, to.1 - 1);
            let over_up = (to.0, to.1 - 2);

            if self.check_kill_rule(piece, to_up, over_up) {
                self.board.board[to.0][to.1 - 1].piece = None;
            }
        }

        if to.1 < ((self.board.size - 2) as usize) {
            let to_down = (to.0, to.1 + 1);
            let over_down = (to.0, to.1 + 2);

            if self.check_kill_rule(piece, to_down, over_down) {
                self.board.board[to.0][to.1 + 1].piece = None;
            }
        }
    }

    fn check_kill_rule(&mut self, piece: Piece, next_place: Coord, over_next_place: Coord) -> bool{
        let team = piece.get_team();
        let next = self.board.get_piece_at(next_place);
        let over_next = self.board.get_piece_at(over_next_place);
        let mut kill: bool = false;

        match next {
            Some(n) => {
                match over_next {
                    Some(nn) => {
                        // TODO: Check if king is in castle surrounded on four sides
                        kill = (n.get_team() != team) && (nn.get_team() == team);
                    },
                    None => {
                        let over_next_stat = self.board.get_status_at(over_next_place);
                        match over_next_stat {
                            Some(Status::Castle) | Some(Status::Corner) => {
                                if n.get_team() != team { kill = true }
                            },
                            _ => {},
                        }
                    },
                }
            },
            None => {},
        }

        let kill_king = (next == Some(Piece::King)) && kill;
        if kill_king { self.winner = Some(Team::Muscovites); }

        return kill;
    }

    fn check_king_escape(&mut self, piece: Piece, to: Coord) {
        let s = self.board.get_status_at(to);
        match piece {
            Piece::King => {
                match s {
                    Some(Status::Corner) => { self.winner = Some(Team::Swedes) },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
