use std::sync::mpsc::Sender;

use ratatui::{style::Stylize, text::Line, widgets::Widget};

use crate::app::Command;

use super::AppView;

pub struct CharacterEditor {
    command_tx: Sender<Command>,
}

impl CharacterEditor {
    pub fn new(tx: Sender<Command>) -> Self {
        CharacterEditor {
            command_tx: tx,
        }
    }
}

impl AppView for CharacterEditor {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let block = ratatui::widgets::Block::bordered()
            .title(Line::from("Character Editor").bold().centered())
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
        block.render(area, frame.buffer_mut());
    }

    fn handle_key_event(&mut self, _key_event: crossterm::event::KeyEvent) {
        // Handle key events here
    }

    fn handle_command(&mut self, _command: crate::app::Command) {
        // Handle commands here
    }
}