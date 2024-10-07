use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, StatefulWidget},
};

pub struct Widget<'a> {
    pub title: &'a str,
}

pub struct State {
    value: String,
    character_index: usize,
    is_active: bool,
}

pub struct StateParameters {
    pub value: String,
    pub is_active: bool,
}

impl State {
    pub fn new(parameters: StateParameters) -> Self {
        let StateParameters { value, is_active } = parameters;

        let mut input = Self {
            value: String::new(),
            character_index: 0,
            is_active,
        };

        for c in value.chars() {
            input.enter_char(c);
        }

        input
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn character_index(&self) -> usize {
        self.character_index
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.value.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_all_chars(&mut self) {
        self.value.clear();
        self.character_index = 0;
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<'a> StatefulWidget for Widget<'a> {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::all()).title(self.title);
        let paragraph = Paragraph::new(state.value()).block(block);

        use ratatui::widgets::Widget;
        paragraph.render(area, buf);
    }
}