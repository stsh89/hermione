use crate::smart_input::InputContract;
use hermione_tui::input::Input;

#[derive(Default)]
pub struct BaseInput {
    input: Input,
}

impl InputContract for BaseInput {
    fn delete_char(&mut self) {
        self.input.delete_char();
    }

    fn enter_char(&mut self, c: char) {
        self.input.enter_char(c);
    }

    fn input(&self) -> &Input {
        &self.input
    }

    fn is_empty(&self) -> bool {
        self.input.value().is_empty()
    }

    fn move_cursor_left(&mut self) {
        self.input.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.input.move_cursor_right();
    }

    fn reset(&mut self) {
        self.input.delete_all_chars();
    }

    fn value(&self) -> Option<&str> {
        Some(self.input.value())
    }
}
