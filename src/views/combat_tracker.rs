use ratatui::{style::Stylize, text::Line, widgets::Widget};

pub struct CombatTracker {
    
}

impl Widget for CombatTracker {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Line::from("Combat Tracker")
            .bold()
            .render(area, buf);
    }
}