use std::io;

// TUI significa Text User Interface (Interfaz de Usuario en Texto).
// Es una interfaz gráfica que se dibuja dentro de la terminal, usando caracteres en lugar de ventanas del sistema operativo.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub fn start() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .title("DevCenter")
                .borders(Borders::ALL);

            frame.render_widget(block, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}