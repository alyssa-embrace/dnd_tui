use ratatui::{style::Stylize, text::Line, widgets::Widget};

pub struct MainMenu {}

impl Widget for MainMenu {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Line::from("Main Menu")
            .bold()
            .centered()
            .render(area, buf);
    }
}