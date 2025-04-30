mod app;
mod views;

use std::{collections::HashMap, io, sync::mpsc};

use views::AppView;
// This function should be used to compose all of the various services.
fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();

    let (tx, rx) = mpsc::channel::<app::Event>();

    let mut view_map: HashMap<app::View, Box<dyn AppView>> = HashMap::new();
    view_map.insert(app::View::MainMenu, Box::new(views::MainMenu::new(tx.clone())));
    view_map.insert(app::View::CharacterEditor, Box::new(views::CharacterEditor::new(tx.clone())));
    view_map.insert(app::View::CombatTracker, Box::new(views::CombatTracker::new(tx.clone())));

    let mut app = app::App {
        exit: false,
        view: app::View::MainMenu,
        view_map: view_map,
        tx: tx.clone(),
        rx: rx,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}