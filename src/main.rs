mod app;
mod views;

use std::{collections::HashMap, io, sync::mpsc};

use views::AppView;
// This function should be used to compose all of the various services.
fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();

    let (command_tx, command_rx) = mpsc::channel::<app::Command>();

    let mut view_map: HashMap<app::View, Box<dyn AppView>> = HashMap::new();
    view_map.insert(app::View::MainMenu, Box::new(views::MainMenu::new(command_tx.clone())));
    view_map.insert(app::View::CharacterEditor, Box::new(views::CharacterEditor::new(command_tx.clone())));
    view_map.insert(app::View::CombatTracker, Box::new(views::CombatTracker::new(command_tx.clone())));

    let mut app = app::App {
        exit: false,
        view: app::View::MainMenu,
        view_map: view_map,
        command_rx: command_rx,
    };

    let (event_tx, event_rx) = mpsc::channel::<app::Event>();

    setup_input_thread(event_tx.clone());

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result
}

fn setup_input_thread(tx: mpsc::Sender<app::Event>) {
    std::thread::spawn(move || {
        loop {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    tx.send(app::Event::Input(key_event)).unwrap();
                }
                _ => {}
            }
        }
    });
}