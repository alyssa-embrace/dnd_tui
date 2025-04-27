mod app;
mod views;

use std::io;

fn main() -> io::Result<()>{
    // This function should be used to compose all of the various services.
    let mut terminal = ratatui::init();

    let mut app = app::App {
        exit: false,
        view: app::View::MainMenu, // This should be a widget that is a menu or something similar
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}