use super::game::{
    SIZE_HNEFATAFL_11,
    SIZE_HNEFATAFL_13,
    SIZE_TABLUT,
    Rule,
    Coord
};

use super::piece::{Piece};

#[derive(Clone, Copy, Debug, PartialEq)]
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
            piece: None::<Piece>,
            status: None::<Status>,
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

    /* Same as `get_piece_at`, but will return `None` if `at` is not within the board.*/
    pub fn get_piece_at_safe(&mut self, at: Coord) -> Option<Piece> {
        if (at.0 < self.size as usize) && (at.1 < self.size as usize) {
            return self.board[at.0][at.1].piece;
        } else {
            return None;
        }
    }

    pub fn get_status_at(&mut self, at: Coord) -> Option<Status> {
        return self.board[at.0][at.1].status;
    }

    pub fn move_piece(&mut self, from: Coord, to: Coord) {
        // let piece = self.board[from.0][from.1].piece;
        let piece = self.get_piece_at(from);
        self.board[from.0][from.1].piece = None;
        self.board[to.0][to.1].piece = piece;
    }
}

#[test]
fn test_new_board() -> std::io::Result<()> {
    let board = Board::new(Rule::Tablut);
    assert_eq!(board.size, SIZE_TABLUT, "Board is not sized for Tablut");
    Ok(())
}

#[test]
fn test_get_piece_at_empty() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let piece = board.get_piece_at((0, 0));
    assert_eq!(piece, None, "piece is not None");
    Ok(())
}

#[test]
fn test_get_piece_at_non_empty() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let piece = board.get_piece_at((0, 5));
    assert_eq!(piece, Some(Piece::Muscovite), "piece is not some muscovite");
    Ok(())
}

#[test]
fn test_get_status_at_normal() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let stat = board.get_status_at((1, 1));
    assert_eq!(stat, None, "square is not normal");
    Ok(())
}

#[test]
fn test_get_status_at_corner() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let stat00 = board.get_status_at((0, 0));
    assert_eq!(stat00, Some(Status::Corner), "square (0, 0) is not corner");
    Ok(())
}

#[test]
fn test_get_status_at_center() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let mid = ((board.size - 1) / 2) as usize;
    let stat_center = board.get_status_at((mid, mid));
    assert_eq!(stat_center, Some(Status::Castle), "center square is not castle");
    Ok(())
}

#[test]
fn test_move_piece() -> std::io::Result<()> {
    let mut board = Board::new(Rule::Tablut);
    let piece1 = board.get_piece_at((0, 4));
    board.move_piece((0, 4), (3, 4));
    let from = board.get_piece_at((0, 4));
    let piece2 = board.get_piece_at((3, 4));
    assert_eq!(from, None, "piece didn't move");
    assert_eq!(piece1, piece2, "moving piece failed");
    Ok(())
}
