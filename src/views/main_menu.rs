use std::sync::mpsc::Sender;

use ratatui::{layout::{Constraint, Layout}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, List, ListState}, Frame};
use crate::{app::Event, views::app_view::AppView};

pub struct MainMenu {
    pub main_menu_state: ListState,
    items: Vec<String>,
    command_tx: Sender<Event>,
}

impl MainMenu {
    pub fn new(tx: Sender<Event>) -> Self {
        let mut main_menu_state = ListState::default();
        main_menu_state.select(Some(0));
        MainMenu {
            main_menu_state,
            items: vec![
                "Character Editor".to_string(),
                "Combat Tracker".to_string(),
            ],
            command_tx: tx,
        }
    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
        let i = match self.main_menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.main_menu_state.select(Some(i));
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Esc => self.command_tx.send(Event::Exit).unwrap(),
                crossterm::event::KeyCode::Up => self.command_tx.send(Event::Previous).unwrap(),
                crossterm::event::KeyCode::Down => self.command_tx.send(Event::Next).unwrap(),
                crossterm::event::KeyCode::Enter => self.command_tx.send(Event::Submit).unwrap(),
                _ => {}
            }
        }
    }
}

impl AppView for MainMenu {
    fn draw(&mut self, frame: &mut Frame) {
        let [_, menu_area, _] = Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)]).areas(frame.area());
        let [_, centered_menu_area, _] = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)]).areas(menu_area);
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

        frame.render_stateful_widget(list, centered_menu_area, &mut self.main_menu_state);
    }

    fn handle_event(&mut self, command: Event) {
        match command {
            Event::Next => self.next(),
            Event::Previous => self.previous(),
            Event::Submit => {
                match self.main_menu_state.selected() {
                    Some(0) => {
                        self.command_tx.send(Event::ChangeView(crate::app::View::CharacterEditor)).unwrap();
                    }
                    Some(1) => {
                        self.command_tx.send(Event::ChangeView(crate::app::View::CombatTracker)).unwrap();
                    }
                    _ => {}
                }
            },
            Event::Input(key_event) => self.handle_key_event(key_event),
            _ => {}
        }
    }
}