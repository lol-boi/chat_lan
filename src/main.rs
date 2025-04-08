use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    chat_messages: Vec<String>, // Add a field to store chat messages
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // Create a layout with two columns, left for chat, right for other content
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(frame.area());

        // Render the chat area (left block)
        let chat_area = chunks[0];
        frame.render_widget(self, chat_area); // We'll use the Widget implementation for rendering chat

        // Render the right block (currently blank)
        let right_block = Block::default()
            .title(Span::styled("Right Panel", Style::default().add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL);
        frame.render_widget(right_block, chunks[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Left => {
                            self.counter = self.counter.saturating_sub(1);
                        }
                        KeyCode::Right => {
                            self.counter = self.counter.saturating_add(1);
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            self.exit = true;
                        }
                        KeyCode::Char('\n') => {
                            // Simulate adding a new message on Enter
                            self.chat_messages.push(format!("User: Message {}", self.chat_messages.len() + 1));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chat_block = Block::default()
            .title(Span::styled("Chat", Style::default().add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL);

        // Create Text from chat messages
        let mut chat_text = Text::from(
            self.chat_messages
                .iter()
                .map(|msg| Line::from(vec![Span::raw(msg)]))
                .collect::<Vec<Line>>(),
        );
        chat_text.clone().patch_style(Style::default().fg(Color::White));

        Paragraph::new(chat_text)
            .block(chat_block)
            .scroll((
                self.chat_messages
                    .len()
                    .saturating_sub(area.height as usize)
                    .try_into()
                    .unwrap_or(0), // Convert to u16, default to 0 on error
                0,
            ))
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
