use ratatui::Frame;

use crate::app::Command;

pub trait AppView {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent);
    fn handle_command(&mut self, command: Command);
}