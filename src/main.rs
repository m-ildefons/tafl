mod app;
mod event;
mod game;
mod ui;
mod piece;
mod board;


use std::error::Error;
use std::io;
use std::time::Duration;

use app::App;
use event::{Event, Events};
use game::Rule;
use piece::Team;
use structopt::StructOpt;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};


use std::net::UdpSocket;
use std::net::Ipv4Addr;


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, possible_values= &Rule::variants(), default_value="Tablut")]
    pub rule: Rule,

    #[structopt(short, long, possible_values= &Team::variants(), default_value="Swedes")]
    pub team: Team,

    #[structopt(short, long)]
    pub server: bool,

    #[structopt(short, long)]
    pub address: String,
}


fn get_socket(server: bool, address: String) -> UdpSocket {
    if server {
        let socket = UdpSocket::bind(address).unwrap();
        socket.set_nonblocking(true).unwrap();
        return socket;
    } else {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        socket.set_nonblocking(true).unwrap();
        socket.connect(address).expect("failed to open socket");
        socket.send(b"hello").unwrap();
        return socket;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let events = Events::new(Duration::from_millis(250));

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let socket = get_socket(opt.server, opt.address);

    let rule = opt.rule;
    let team = opt.team;
    let mut app = App::new(format!(" Tafl :: {rule} :: {team} ").into(), rule, team, opt.server, socket);


    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let _ = &mut app.read_sock();

        match events.next()? {
            Event::Input(key) => app.on_key(key),
            _ => {},
        }

        if app.game.winner != None {
            _ = terminal.clear();
            break;
        }
    }

    Ok(())
}
