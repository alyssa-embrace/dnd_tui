use ratatui::{style::Stylize, text::Line, widgets::Widget};

pub struct CharacterEditor {

}

impl Widget for CharacterEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Line::from("Character Editor")
            .bold()
            .render(area, buf);
    }
}