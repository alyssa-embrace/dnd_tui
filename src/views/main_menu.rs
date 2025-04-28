use ratatui::{style::{Color, Style, Stylize}, text::Line, widgets::{Block, List, ListState}, Frame};

pub struct MainMenu {
    pub main_menu_state: ListState,
    items: Vec<String>,
}

impl MainMenu {
    pub fn default() -> Self {
        let mut main_menu_state = ListState::default();
        main_menu_state.select(Some(0));
        MainMenu {
            main_menu_state,
            items: vec![
                "Character Editor".to_string(),
                "Combat Tracker".to_string(),
            ],
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::bordered()
            .title(Line::from("Main Menu").bold().centered())
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black));
        let list_items = self.items.iter()
            .map(|li| { 
                Line::from(li.clone())
                    .centered()
            })
            .collect::<Vec<_>>();
        let list = List::new(list_items).highlight_symbol(">").block(block);

        frame.render_stateful_widget(list, area, &mut self.main_menu_state);
    }

    pub fn next(&mut self) {
        let i = match self.main_menu_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.main_menu_state.select(Some(i));
    }
}