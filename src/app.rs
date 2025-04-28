use crate::views::AppView;

use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use std::{collections::HashMap, io, sync::mpsc::{Receiver, Sender}};

pub struct App {
    pub exit: bool,
    pub view: View, // This also allows us to differentiate for the input handling
    pub view_map: HashMap<View, Box<dyn AppView>>,
    pub command_tx: Sender<Command>,
    pub command_rx: Receiver<Command>,
}

pub enum Event {
    Input(crossterm::event::KeyEvent)
}

#[derive(PartialEq, Eq, Hash)]
pub enum View {
    MainMenu,
    CharacterEditor,
    CombatTracker,
}

pub enum Command {
    Exit,
    View(View),
    Submit,
    Next,
    Previous,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            // We should block on receiving update events here before rerendering
            terminal.draw(|frame| self.draw(frame))?;
            match rx.try_recv() {
                Ok(Event::Input(key_event)) => self.handle_key_event(key_event)?,
                Err(_) => {}
            }
            match self.command_rx.try_recv() {
                Ok(command) => self.handle_command(command),
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        // We should differentiate between the various views here and draw them accordingly.
        self.view_map
            .get_mut(&self.view)
            .unwrap()
            .draw(frame);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code { // This is a placeholder for the actual key event handling
                // We should differentiate between the various views here and handle the input accordingly.
                KeyCode::Esc => self.command_tx.send(Command::Exit).unwrap(),
                KeyCode::Char('1') => self.view = View::MainMenu,
                KeyCode::Char('2') => self.view = View::CharacterEditor,
                KeyCode::Char('3') => self.view = View::CombatTracker,
                _ => {
                    self.view_map
                        .get_mut(&self.view)
                        .unwrap()
                        .handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Exit => self.exit = true,
            Command::View(view) => self.view = view,
            _ => {
                self.view_map
                    .get_mut(&self.view)
                    .unwrap()
                    .handle_command(command);
            }
        }
    }

}
