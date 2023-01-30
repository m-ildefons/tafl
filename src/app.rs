use termion::{
    event::Key,
};

use super::game::{Game, Rule, Coord, Team, Piece};


pub struct App {
    pub title: String,
    pub game: Game,
    pub cursor: Coord,
    pub selected: Option<Coord>,
    pub help: bool,
}


impl App {
    pub fn new(title: String, rule: Rule) -> Self {
        App {
            title,
            game: Game::new(rule),
            cursor: (0, 0),
            selected: None,
            help: false,
        }
    }

    pub fn on_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') | Key::Char('q') => {
                self.game.winner = Some(Team::Swedes);
            },
            Key::Char('h') => {
                self.help ^= true;
            },
            Key::Left => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                }
            },
            Key::Up => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                }
            },
            Key::Right => {
                if self.cursor.0 < (self.game.board.size - 1) as usize {
                    self.cursor.0 += 1;
                }
            },
            Key::Down => {
                if self.cursor.1 < (self.game.board.size - 1) as usize {
                    self.cursor.1 += 1;
                }
            },
            Key::Char(' ') => {
                match self.selected {
                    Some(selected) => {
                        self.game.move_piece(selected, self.cursor);
                        self.selected = None;
                    },
                    None => {
                        match self.game.board.get_piece_at(self.cursor) {
                            Some(piece) => {
                                if self.check_turn(piece) {
                                    self.selected = Some((self.cursor.0, self.cursor.1))
                                }
                            },
                            None => {},  // Nothing
                        }
                    },
                }
            },
            _ => (),
        }
    }

    fn check_turn(&mut self, piece: Piece) -> bool {
        let team = piece.get_team();
        let turn = self.game.turn;

        let muscovites = ((turn % 2) == 0) && team == Team::Muscovites;
        let swedes = ((turn % 2) != 0) && team == Team::Swedes;

        return muscovites || swedes;
    }
}
