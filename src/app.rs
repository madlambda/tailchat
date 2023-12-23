use ratatui::prelude::Line;
use ratatui::widgets::Paragraph;
use tui_input::Input;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub lines: Vec<String>,
    pub input: Input,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            lines: vec![],
            input: Input::default(),
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(format!("me: {}", line));
    }

    pub fn paragraph(&self, height: u16) -> Paragraph {
        // TODO wrap text
        let lines: Vec<Line> = self.lines.iter().map(|l| Line::from(l.clone())).collect();
        let scroll_y = (lines.len() as i32 - height as i32).max(0) as u16;
        Paragraph::new(lines).scroll((scroll_y, 0))
    }
}
