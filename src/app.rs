use crate::views::{
    main_menu::{self}, CharacterEditor, CombatTracker, MainMenu
};

use crossterm::event::KeyCode;
use ratatui::{widgets::ListState, DefaultTerminal};
use std::{io, sync::mpsc::Receiver};

/*
    The app will have multiple main widgets.
    - A selection or main menu where users can select which mode they are in
    - A character editor where users can create D&D characters
    - A combat tracker where users can track the current state of the game
 */

pub struct App<'a> {
    pub exit: bool,
    pub view: View, // This also allows us to differentiate for the input handling
    pub main_menu: &'a mut MainMenu,
}

pub enum Event {
    Input(crossterm::event::KeyEvent)
}

pub enum View {
    MainMenu,
    CharacterEditor,
    CombatTracker,
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            // We should block on receiving update events here before rerendering
            terminal.draw(|frame| self.draw(frame))?;
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
            }
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame) {
        // We should differentiate between the various views here and draw them accordingly.
        match self.view {
            View::MainMenu => {
                // Draw the main menu
                self.main_menu.draw(frame);
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

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code { // This is a placeholder for the actual key event handling
                // We should differentiate between the various views here and handle the input accordingly.
                KeyCode::Esc => self.exit = true,
                KeyCode::Char('1') => self.view = View::MainMenu,
                KeyCode::Char('2') => self.view = View::CharacterEditor,
                KeyCode::Char('3') => self.view = View::CombatTracker,
                KeyCode::Up => {
                    // Handle up key event
                    match self.view {
                        View::MainMenu => self.main_menu.next(),
                        _ => {}
                    }
                },
                KeyCode::Enter => {
                    // Handle enter key event
                    match self.view {
                        View::MainMenu => {
                            if let Some(selected) = self.main_menu.main_menu_state.selected() {
                                match selected {
                                    0 => self.view = View::CharacterEditor,
                                    1 => self.view = View::CombatTracker,
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }

}
