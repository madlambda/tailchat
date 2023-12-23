#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub text: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            text: "alice> Hello!\nbob> Hi!\n".into(),
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
