mod app;
mod entropy;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, path::PathBuf, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to analyze
    #[arg(short, long)]
    file: PathBuf,

    /// Block size for entropy calculation
    #[arg(short, long, default_value_t = 256)]
    block_size: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Check if file exists
    if !args.file.exists() {
        eprintln!("File not found: {:?}", args.file);
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let app = App::new(args.file, args.block_size);
    
    // Check for error in loading app (e.g. file read error)
    if let Err(e) = app {
        // Restore terminal before printing error
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

    // Restore terminal
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

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.on_quit();
                        return Ok(());
                    }
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.on_zoom_in(),
                    KeyCode::Char('-') | KeyCode::Char('_') => app.on_zoom_out(),
                    _ => {}
                }
            }
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}
