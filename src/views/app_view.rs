use ratatui::Frame;

use crate::app::Event;

pub trait AppView {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_event(&mut self, event: Event);
}