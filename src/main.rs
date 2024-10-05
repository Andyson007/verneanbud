//! The main entry point
//!
//! This is a simple app used for storing ideas in an easily accessible way
use futures::executor::block_on;
use std::io;
use verneanbud::{app::App, errors, ui::ui};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

type Backend = CrosstermBackend<io::Stdout>;

fn main() -> color_eyre::Result<()> {
    let mut terminal = setup_terminal()?;
    // create app and run it
    let mut app = block_on(App::new())?;
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    if let Ok(_do_print) = res {
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn setup_terminal() -> color_eyre::Result<Terminal<Backend>> {
    errors::install_hooks()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<Backend>) -> color_eyre::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(terminal: &mut Terminal<Backend>, app: &mut App) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            if matches!(
                key,
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
            ) {
                restore_terminal(terminal)?;
                std::process::exit(130);
            }
            if app.handle_input(key) {
                return Ok(());
            };
            app.run_db_actions()?;
        }
    }
}
