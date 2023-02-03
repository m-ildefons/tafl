use termion::event::Key;
use super::game::{Game, Rule, Coord};
use super::piece::Team;


use std::io;
use std::net::UdpSocket;


pub struct App {
    pub title: String,
    pub game: Game,
    pub team: Team,
    pub cursor: Coord,
    pub selected: Option<Coord>,
    pub help: bool,
    pub server: bool,
    pub socket: UdpSocket,
}


impl App {
    pub fn new(title: String, rule: Rule, team: Team, server: bool, socket: UdpSocket) -> Self {
        App {
            title,
            game: Game::new(rule),
            team,
            cursor: (0, 0),
            selected: None,
            help: false,
            server,
            socket,
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
            Key::Left if self.cursor.0 > 0 => {
                self.cursor.0 -= 1;
            },
            Key::Up if self.cursor.1 > 0 => {
                self.cursor.1 -= 1;
            },
            Key::Right if self.cursor.0 < (self.game.board.size - 1) as usize => {
                self.cursor.0 += 1;
            },
            Key::Down if self.cursor.1 < (self.game.board.size - 1) as usize => {
                self.cursor.1 += 1;
            },
            Key::Char(' ') => {
                match self.selected {
                    Some(selected) => {
                        self.game.move_piece(selected, self.cursor);
                        self.socket.send(&['m' as u8, selected.0 as u8, selected.1 as u8, self.cursor.0 as u8, self.cursor.1 as u8]);
                        self.selected = None;
                    },
                    None => {
                        match self.game.board.get_piece_at(self.cursor) {
                            Some(piece) if self.game.get_turn() == self.team && piece.get_team() == self.team => {
                                self.selected = Some((self.cursor.0, self.cursor.1))
                            },
                            _ => {},  // Nothing
                        }
                    },
                }
            },
            _ => (),
        }
    }

    pub fn read_sock(&mut self) {
        let mut buffer = [0; 8];
        match self.socket.recv_from(&mut buffer) {
            Ok((n, addr)) => {
                if buffer[0] == ('m' as u8) && n >= 5 {
                    let from = (buffer[1] as usize, buffer[2] as usize);
                    let to = (buffer[3] as usize, buffer[4] as usize);
                    self.game.move_piece(from, to);
                } else if buffer[0] == ('h' as u8) {
                    self.socket.connect(addr).expect("failed to reach back");
                    let t = if self.team == Team::Muscovites { 's' } else { 'm' };
                    self.socket.send(&['t' as u8, t as u8]).unwrap();
                    self.title = format!("Tafl :: {:?} :: {:?} ({:?})", self.game.rule, self.team, addr);
                } else if buffer[0] == ('t' as u8) {
                    if buffer[1] == ('m' as u8) {
                        self.team = Team::Muscovites;
                    } else if buffer[1] == ('s' as u8) {
                        self.team = Team::Swedes;
                    }
                    self.title = format!("Tafl :: {:?} :: {:?}", self.game.rule, self.team);
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
            Err(e) => { println!("{e}") },
        }
    }
}
