use crate::views::AppView;

use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;

use std::{
    collections::HashMap,
    io,
    sync::mpsc::{Receiver, Sender},
};

pub struct App {
    pub exit: bool,
    pub view: View, // This also allows us to differentiate for the input handling
    pub view_map: HashMap<View, Box<dyn AppView>>,
    pub tx: Sender<Event>,
    pub rx: Receiver<Event>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum View {
    MainMenu,
    CharacterEditor,
    CombatTracker,
}

pub enum Event {
    Exit,
    ChangeView(View),
    Submit,
    Next,
    Previous,
    Undo,
    Input(KeyEvent),
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        Self::setup_input_thread(self.tx.clone());
        while !self.exit {
            // We should block on receiving update events here before rerendering
            terminal.draw(|frame| self.draw(frame))?;
            if let Ok(command) = self.rx.recv() {
                self.handle_event(command);
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        // We should differentiate between the various views here and draw them accordingly.
        self.view_map.get_mut(&self.view).unwrap().draw(frame);
    }

    fn handle_event(&mut self, command: Event) {
        match command {
            Event::Exit => match self.view {
                View::MainMenu => self.exit = true,
                _ => self.view = View::MainMenu,
            },
            Event::ChangeView(view) => self.view = view,
            _ => {
                self.view_map
                    .get_mut(&self.view)
                    .unwrap()
                    .handle_event(command);
            }
        }
    }

    fn setup_input_thread(tx: Sender<Event>) {
        std::thread::spawn(move || {
            loop {
                match crossterm::event::read() {
                    Ok(crossterm::event::Event::Key(key_event)) => {
                        if let Err(e) = tx.send(Event::Input(key_event)) {
                            // We should probably include a logger here because we can't assume
                            // that the console is a safe place to print event failures explicitly BECAUSE we're a TUI
                            eprintln!("Failed to send input event: {}", e);
                            break; // Exit the loop if the receiver has disconnected
                        }
                    }
                    Ok(_) => {} // Ignore other event types
                    Err(e) => {
                        eprintln!("Error reading input event: {}", e);
                        // Continue and try to read the next event
                    }
                }
            }
        });
    }
}
