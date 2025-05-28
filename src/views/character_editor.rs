use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use crate::{
    app::Event,
    parser::{CharacterEditorLexer, Lexer},
};

use super::AppView;

pub struct CharacterEditor {
    tx: Sender<Event>,
    input: String,
    lexer: CharacterEditorLexer,
}

impl CharacterEditor {
    pub fn new(tx: Sender<Event>) -> Self {
        CharacterEditor {
            tx,
            input: String::new(),
            lexer: CharacterEditorLexer {},
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => {
                    self.tx.send(Event::Exit).unwrap();
                }
                KeyCode::Char(c) => {
                    if key_event
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        match c {
                            'z' => {
                                self.tx.send(Event::Undo).unwrap();
                            }
                            _ => {}
                        }
                    } else {
                        if c.is_alphanumeric() || c == ' ' || c.is_ascii_punctuation() {
                            self.input.push(
                                if key_event
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::SHIFT)
                                {
                                    c.to_ascii_uppercase()
                                } else {
                                    c
                                },
                            );
                        }
                    }
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    let tokens = self.lexer.lex(self.input.clone());
                    for token in tokens.iter() {
                        match token {
                            crate::parser::character_editor_lexer::CharacterEditorToken::Word(
                                word,
                            ) => {
                                println!("{word}");
                            }
                            crate::parser::character_editor_lexer::CharacterEditorToken::Number(
                                num,
                            ) => {
                                println!("{num}");
                            }
                        }
                    }
                    self.input.clear(); // Clear input after submission
                }
                _ => {}
            }
        }
    }
}

impl AppView for CharacterEditor {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let [data_area, command_input_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.area());

        let block = ratatui::widgets::Block::bordered()
            .title(Line::from("Character Editor").bold().centered())
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
        block.render(data_area, frame.buffer_mut());

        Line::from(self.input.clone())
            .bold()
            .render(command_input_area, frame.buffer_mut());
    }

    fn handle_event(&mut self, event: crate::app::Event) {
        if let Event::Input(key_event) = event {
            self.handle_key_event(key_event);
        }
    }
}
