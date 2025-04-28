mod app;
mod views;

use std::{io, sync::mpsc};

// This function should be used to compose all of the various services.
fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();

    let mut main_menu = views::MainMenu::default();

    let mut app = app::App {
        exit: false,
        view: app::View::MainMenu,
        main_menu: &mut main_menu,
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