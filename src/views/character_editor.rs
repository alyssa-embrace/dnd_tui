use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{layout::{Constraint, Layout}, style::Stylize, text::Line, widgets::Widget};

use crate::app::Command;

use super::AppView;

pub struct CharacterEditor {
    command_tx: Sender<Command>,
    command_input: String,
}

impl CharacterEditor {
    pub fn new(tx: Sender<Command>) -> Self {
        CharacterEditor {
            command_tx: tx,
            command_input: String::new(),
        }
    }
}

impl AppView for CharacterEditor {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let [data_area, command_input_area] = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)]).areas(frame.area());

        let block = ratatui::widgets::Block::bordered()
            .title(Line::from("Character Editor").bold().centered())
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
        block.render(data_area, frame.buffer_mut());

        Line::from(self.command_input.clone()).bold().render(command_input_area, frame.buffer_mut());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        // Handle key events here
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => {
                    self.command_tx.send(Command::Exit).unwrap();
                }
                KeyCode::Char(c) => {
                    if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        match c {
                            'z' => {
                                self.command_tx.send(Command::Undo).unwrap();
                            }
                            _ => {}
                        }
                    } else {
                        if c.is_alphanumeric() || c == ' ' || c.is_ascii_punctuation(){
                            self.command_input.push(if key_event.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
                                c.to_ascii_uppercase()
                            } else {
                                c
                            });
                        }
                    }
                }
                KeyCode::Backspace => {
                    // We should convert this to a command and send it to the command channel
                    self.command_input.pop();
                }
                KeyCode::Enter => {
                    // Handle command submission
                    self.command_input.clear(); // Clear input after submission
                }
                _ => {                    
                }
            }
        }
    }

    fn handle_command(&mut self, _command: crate::app::Command) {
        // Handle commands here
    }
}