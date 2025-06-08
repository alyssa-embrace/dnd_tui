use std::{str::FromStr, sync::mpsc::Sender};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use log::error;

use crate::{
    app::Event,
    parser::{
        character_editor_lexer::CharacterEditorLexerError, CharacterEditorLexer,
        CharacterEditorParser, Lexer, Parser,
    },
};

use super::AppView;

pub struct CharacterEditor {
    tx: Sender<Event>,
    input: String,
    feedback: String,
    lexer: CharacterEditorLexer,
    parser: CharacterEditorParser,
}

impl CharacterEditor {
    pub fn new(tx: Sender<Event>) -> Self {
        CharacterEditor {
            tx,
            input: String::new(),
            feedback: String::new(),
            lexer: CharacterEditorLexer {},
            parser: CharacterEditorParser {},
        }
    }

    fn handle_lexer_error(&mut self, error: CharacterEditorLexerError) {
        match error {
            CharacterEditorLexerError::UnexpectedCharacter {
                input,
                unexpected_char,
            } => {
                self.feedback = format!(
                    "Unexpected character '{}' in input \"{:?}\"",
                    unexpected_char, input
                );

                error!(
                    "Unexpected character '{}' in input \"{:?}\"",
                    unexpected_char, input
                );
            }
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
                    match self.lexer.lex(self.input.clone()) {
                        Ok(tokens) => {
                            // We need to handle parse success and error failure
                            match self.parser.parse(tokens) {
                                Ok(event) => {
                                    self.tx.send(event).unwrap();
                                }
                                Err(err) => error!("A parsing error has occurred"),
                            }
                        }
                        Err(err) => {
                            self.handle_lexer_error(err);
                        } // We should log the error somehow for user consumption
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

        let [_, feedback_area, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let block = ratatui::widgets::Block::bordered()
            .title(Line::from("Character Editor").bold().centered())
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
        block.render(data_area, frame.buffer_mut());

        if !self.feedback.is_empty() {
            Line::from(self.feedback.clone())
                .bold()
                .bg(ratatui::style::Color::Rgb(128, 0, 255))
                .render(feedback_area, frame.buffer_mut());
        }

        // This is the input line. We need to treat this area as sacrosanct.
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
