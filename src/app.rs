use crate::views::{
    MainMenu,
    CharacterEditor,
    CombatTracker,
};

use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use std::io;

/*
    The app will have multiple main widgets.
    - A selection or main menu where users can select which mode they are in
    - A character editor where users can create D&D characters
    - A combat tracker where users can track the current state of the game
 */

pub struct App {
    pub exit: bool,
    pub view: View, // This also allows us to differentiate for the input handling
}

pub enum View {
    MainMenu,
    CharacterEditor,
    CombatTracker,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            // We should block on receiving update events here before rerendering
            terminal.draw(|frame| self.draw(frame))?;
            match crossterm::event::read().unwrap() { // This is a placeholder for the actual event handling
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc => self.exit = true,
                        KeyCode::Char('1') => self.view = View::MainMenu,
                        KeyCode::Char('2') => self.view = View::CharacterEditor,
                        KeyCode::Char('3') => self.view = View::CombatTracker,
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        // We should differentiate between the various views here and draw them accordingly.
        match self.view {
            View::MainMenu => {
                // Draw the main menu
                frame.render_widget(
                    MainMenu {},
                    frame.area(),
                );
            }
            View::CharacterEditor => {
                // Draw the character editor
                frame.render_widget(
                    CharacterEditor {},
                    frame.area(),
                );
            }
            View::CombatTracker => {
                // Draw the combat tracker
                frame.render_widget(
                    CombatTracker {},
                    frame.area(),
                );
            }
        }
    }
}