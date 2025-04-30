pub trait AppInputHandler {
    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent);
}