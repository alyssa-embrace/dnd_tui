use std::sync::mpsc::Sender;

use ratatui::{style::Stylize, text::Line, widgets::Widget};

use crate::app::Command;

use super::AppView;

pub struct CombatTracker {
    command_tx: Sender<Command>,
}

impl CombatTracker {
    pub fn new(tx: Sender<Command>) -> Self {
        CombatTracker {
            command_tx: tx,
        }
    }
}

impl AppView for CombatTracker {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let block = ratatui::widgets::Block::bordered()
            .title(Line::from("Combat Tracker").bold().centered())
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