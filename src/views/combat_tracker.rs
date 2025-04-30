use std::sync::mpsc::Sender;

use ratatui::{style::Stylize, text::Line, widgets::Widget};

use crate::app::Event;

use super::AppView;

pub struct CombatTracker {
    tx: Sender<Event>,
}

impl CombatTracker {
    pub fn new(tx: Sender<Event>) -> Self {
        CombatTracker { 
            tx 
        }
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Esc => {
                    self.tx.send(Event::Exit).unwrap();
                }
                _ => {}
            }
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

    fn handle_event(&mut self, event: crate::app::Event) {
        match event {
            Event::Input(key_event) => self.handle_key_event(key_event),
            _ => {}
        }
    }
}