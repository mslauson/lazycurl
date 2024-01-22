use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};

use crate::ui::textarea;

mod ui;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = [TextArea::default(), TextArea::default()];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());

    let mut which = 0;
    textarea::activate(&mut textarea[0]);
    textarea::inactivate(&mut textarea[1]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            for (textarea, chunk) in textarea.iter().zip(chunks.iter()) {
                let widget = textarea.widget();
                f.render_widget(widget, *chunk);
            }
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Char('x'),
                ctrl: true,
                ..
            } => {
                textarea::inactivate(&mut textarea[which]);
                which = (which + 1) % 2;
                textarea::activate(&mut textarea[which]);
            }
            input => {
                textarea[which].input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Left textarea: {:?}", textarea[0].lines());
    println!("Right textarea: {:?}", textarea[1].lines());
    Ok(())
}
