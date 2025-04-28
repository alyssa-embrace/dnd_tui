use std::sync::mpsc::Sender;

use ratatui::{style::{Color, Style, Stylize}, text::Line, widgets::{Block, List, ListState}, Frame};
use crate::{app::Command, views::app_view::AppView};

pub struct MainMenu {
    pub main_menu_state: ListState,
    items: Vec<String>,
    command_tx: Sender<Command>,
}

impl MainMenu {
    pub fn new(tx: Sender<Command>) -> Self {
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
}

impl AppView for MainMenu {
    fn draw(&mut self, frame: &mut Frame) {
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

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Up => self.command_tx.send(Command::Previous).unwrap(),
                crossterm::event::KeyCode::Down => self.command_tx.send(Command::Next).unwrap(),
                crossterm::event::KeyCode::Enter => self.command_tx.send(Command::Submit).unwrap(),
                _ => {}
            }
        }
    }
    
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Next => self.next(),
            Command::Previous => self.previous(),
            Command::Submit => {
                match self.main_menu_state.selected() {
                    Some(0) => {
                        self.command_tx.send(Command::View(crate::app::View::CharacterEditor)).unwrap();
                    }
                    Some(1) => {
                        self.command_tx.send(Command::View(crate::app::View::CombatTracker)).unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}