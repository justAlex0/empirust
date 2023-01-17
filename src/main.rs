extern crate mpd;

use config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mpd::Client;
use std::error::Error;
use tui::{backend::CrosstermBackend, Terminal};
use ui::App;

mod config;
mod input;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // connect to mpd server
    let mut client = Client::connect("127.0.0.1:6600").unwrap();

    // parse config
    let config = Config::new().unwrap();

    // setup UI
    let mut app = App::build(&mut client, &config).unwrap();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    if let Err(e) = input::input(&mut terminal, &mut app, client, config) {
        println!("Input Error: {:?}\r", e);
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
