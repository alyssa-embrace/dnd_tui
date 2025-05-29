mod app;
mod parser;
mod views;

use flexi_logger::{FileSpec, Logger};
use std::{collections::HashMap, io, sync::mpsc};

use views::AppView;
// This function should be used to compose all of the various services.
fn main() -> io::Result<()> {
    let _logger = Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default())
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    let mut terminal = ratatui::init();

    let (tx, rx) = mpsc::channel::<app::Event>();

    let mut view_map: HashMap<app::View, Box<dyn AppView>> = HashMap::new();
    view_map.insert(
        app::View::MainMenu,
        Box::new(views::MainMenu::new(tx.clone())),
    );
    view_map.insert(
        app::View::CharacterEditor,
        Box::new(views::CharacterEditor::new(tx.clone())),
    );
    view_map.insert(
        app::View::CombatTracker,
        Box::new(views::CombatTracker::new(tx.clone())),
    );

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
