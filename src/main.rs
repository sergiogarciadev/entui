mod app;
mod entropy;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io, path::PathBuf, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    file: PathBuf,

    #[arg(short, long, default_value_t = 256)]
    block_size: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !args.file.exists() {
        eprintln!("File not found: {:?}", args.file);
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(args.file, args.block_size);

    if let Err(e) = app {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        eprintln!("Error analyzing file: {}", e);
        return Ok(());
    }

    let mut app = app.unwrap();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        if let Err(err) = terminal.draw(|f| ui::draw(f, app)) {
          panic!("Terminal error: {}", err.to_string());
        }

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.on_quit();
                        return Ok(());
                    }
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Up | KeyCode::Char('+') | KeyCode::Char('=') => app.on_zoom_in(),
                    KeyCode::Down | KeyCode::Char('-') | KeyCode::Char('_') => app.on_zoom_out(),
                    KeyCode::Char('h') => app.hex_mode = !app.hex_mode,
                    _ => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
