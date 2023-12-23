use std::io::stdout;
use std::{io, panic};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::prelude::{Constraint, CrosstermBackend, Direction, Frame, Layout, Terminal};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn run() -> io::Result<()> {
    set_up_terminal()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("failed to reset the terminal");
        panic_hook(panic);
    }));

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    reset_terminal()?;

    Ok(())
}

fn set_up_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn reset_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        let ev = event::read()?;
        match ev {
            Event::Key(key)
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc =>
            {
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let sample_chat = "alice> Hello!\nbob> Hi!\n";
    let text = Paragraph::new(sample_chat);
    let chat_widget = text.block(
        Block::default()
            .title("Chat with bob")
            .borders(Borders::ALL),
    );

    // TODO use https://github.com/sayanarijit/tui-input
    let input = Paragraph::new("");
    let input_widget = input.block(Block::default().title("Input").borders(Borders::ALL));

    let help_widget = Paragraph::new("Esc to quit");

    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(chat_widget, layout[0]);
    frame.render_widget(input_widget, layout[1]);
    frame.render_widget(help_widget, layout[2]);
}
