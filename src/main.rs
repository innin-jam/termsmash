use crate::input::get_input;
use crate::ui::ui;
use crate::{app::App, input::Input};
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};
use std::io;

mod app;
mod input;
mod ui;

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    //restore terminal
    disable_raw_mode()?;
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    );

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    while !app.should_quit() {
        let _ = terminal.draw(|f| ui(f, app));

        let mut input: Option<Input> = None;

        if event::poll(std::time::Duration::from_millis(33))? {
            if let Event::Key(key) = event::read()? {
                input = get_input(key.code, key.modifiers)
            }
            // TODO: wait here if there was a key event to fix jitteriness from key inputs
        }

        app.tick(input);
    }
    Ok(())
}
