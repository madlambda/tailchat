pub mod app;

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
use tui_input::backend::crossterm::EventHandler;

pub fn run() -> io::Result<()> {
    set_up_terminal()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("failed to reset the terminal");
        panic_hook(panic);
    }));

    let mut app = app::App::new();

    while !app.should_quit {
        terminal.draw(|frame| ui(&app, frame))?;
        handle_events(&mut app)?;
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

fn handle_events(app: &mut app::App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        let ev = event::read()?;
        match ev {
            Event::Key(key)
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc =>
            {
                app.quit();
            }
            Event::Key(key)
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Enter =>
            {
                app.add_line(app.input.value().into());
                app.input.reset();
            }
            // TODO use arrow keys, PgUp, PgDown to scroll
            _ => {
                app.input.handle_event(&ev);
            }
        }
    }
    Ok(())
}

fn ui(app: &app::App, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    let text_height = layout[0].height - 2;
    let chat_widget = app.paragraph(text_height).block(
        Block::default()
            .title("Chat with bob")
            .borders(Borders::ALL),
    );
    frame.render_widget(chat_widget, layout[0]);

    let width = layout[1].width.max(3) - 3; // reserve 2 chars for borders and 1 for the cursor
    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value());
    let input_widget = input
        .scroll((0, scroll as u16))
        .block(Block::default().title("Input").borders(Borders::ALL));
    frame.render_widget(input_widget, layout[1]);
    frame.set_cursor(
        layout[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        layout[1].y + 1,
    );

    let help_widget = Paragraph::new("Enter: send, Esc: quit");
    frame.render_widget(help_widget, layout[2]);
}
