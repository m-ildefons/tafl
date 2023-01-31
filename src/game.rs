use structopt::clap::arg_enum;

use super::piece::{Piece, Team};
use super::board::{Board, Status};

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
            winner: None::<Team>,
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

    fn check_kill_rule(&mut self, piece: Piece, next_place: Coord, over_next_place: Coord) -> bool {
        let team = piece.get_team();
        let next = self.board.get_piece_at(next_place);
        let next_stat = self.board.get_status_at(next_place);
        let over_next = self.board.get_piece_at_safe(over_next_place);
        let over_next_stat = self.board.get_status_at(over_next_place);
        let mut kill: bool = false;

        let dir = next_place.0 == over_next_place.0;
        let right_place = if dir { (next_place.0, next_place.1 + 1) }
                          else { (next_place.0 + 1, next_place.1) };
        let left_place = if dir { (next_place.0, next_place.1 - 1) }
                         else { (next_place.0 - 1, next_place.1) };
        let right = self.board.get_piece_at_safe(right_place);
        let left = self.board.get_piece_at_safe(left_place);

        if next_stat != Some(Status::Castle) {
            match (next, over_next) {
                (Some(n), Some(on)) => {
                    // regular kill
                    kill = (n.get_team() != team)
                        && (on.get_team() == team);
                },
                (Some(n), None) => {
                    // kill against corner or kill of non-king against castle
                    kill = (n.get_team() != team)
                        && ((over_next_stat == Some(Status::Corner))
                            || (over_next_stat == Some(Status::Castle) && n != Piece::King));
                }
                _ => {},
            }
            match (next, over_next, left, right) {
                (Some(n), None, Some(l), Some(r)) => {
                    // kill of king against castle
                    kill = (n == Piece::King)
                        && (over_next_stat == Some(Status::Castle))
                        && (l.get_team() == team)
                        && (r.get_team() == team)
                },
                _ => {},
            }
        } else {
            match (next, over_next, left, right) {
                (Some(n), Some(on), Some(l), Some(r)) => {
                    // kill in castle
                    kill = (n.get_team() != team)
                        && (on.get_team() == team)
                        && (l.get_team() == team)
                        && (r.get_team() == team)
                },
                _ => {},
            }
        }


        let kill_king = (next == Some(Piece::King)) && kill;
        if kill_king { self.winner = Some(Team::Muscovites); }

        return kill;
    }

    fn check_king_escape(&mut self, piece: Piece, to: Coord) {
        let s = self.board.get_status_at(to);
        if (piece == Piece::King) && s == Some(Status::Corner) {
            self.winner = Some(Team::Swedes);
        }
    }
}
