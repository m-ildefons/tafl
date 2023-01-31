mod app;
mod event;
mod game;
mod ui;
mod piece;
mod board;


use std::{
    error::Error,
    io::{self},
    time::Duration,
};

use app::{App};
use event::{Event, Events};
use game::{Rule};
use structopt::StructOpt;
use termion::{
    raw::IntoRawMode,
};
use tui::{backend::TermionBackend, Terminal};


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, possible_values= &Rule::variants(), default_value="Tablut")]
    pub rule: Rule,
}


fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let events = Events::new(Duration::from_millis(250));

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Tafl".into(), opt.rule);

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

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
