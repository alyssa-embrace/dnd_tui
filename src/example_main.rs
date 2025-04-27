use std::{io, sync::mpsc, thread, time::Duration};

use ratatui::{layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, symbols::border, text::Line, widgets::{Block, Gauge, Widget}, DefaultTerminal};
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App { 
        exit: false, 
        progress_bar_color: ratatui::style::Color::Green,
        background_progress: 0_f64,
    };

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_background_progress_events = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_to_background_progress_events);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
    }
}

pub struct App {
    exit: bool,
    progress_bar_color: ratatui::style::Color,
    background_progress: f64,
}

impl App {
    fn run (&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Progress(progress) => self.background_progress = progress,
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Esc => self.exit = true,
                crossterm::event::KeyCode::Char('c') => {
                    self.progress_bar_color = if self.progress_bar_color == ratatui::style::Color::Green {
                        ratatui::style::Color::Yellow
                    } else {
                        ratatui::style::Color::Green
                    };
                }
                _ => {}
            }
        }
        
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Render a title in the top of the layout

        let vertical_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);

        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from("Process overview").bold().render(title_area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]).centered();

        let block = Block::bordered()
            .title(Line::from("Background Processes"))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!("Process 1: {:.2}%", self.background_progress * 100_f64))
            .ratio(0.5);

        progress_bar.render(Rect {
            x: gauge_area.left(),
            y: gauge_area.top(),
            width: gauge_area.width,
            height: 3,
        }, buf);
    }
}