pub struct Selector<T> {
    values: Vec<T>,
    cursor: Option<usize>,
}

impl<T> Selector<T> {
    pub fn select_first(&mut self) {
        if !self.values.is_empty() {
            self.cursor = Some(0);
        }
    }

    pub fn item(&self) -> Option<&T> {
        self.cursor.and_then(|current| self.values.get(current))
    }

    pub fn items(&self) -> &[T] {
        &self.values
    }

    pub fn item_number(&self) -> Option<usize> {
        self.cursor
    }

    pub fn select_last(&mut self) {
        if !self.values.is_empty() {
            self.cursor = Some(self.values.len() - 1);
        }
    }

    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            cursor: None,
        }
    }

    pub fn next(&mut self) {
        if self.values.is_empty() {
            return;
        }

        if let Some(current) = self.cursor {
            if current < self.values.len() - 1 {
                self.cursor = Some(current + 1);
            } else {
                self.cursor = Some(0);
            }
        } else {
            self.cursor = Some(0);
        }
    }

    pub fn prev(&mut self) {
        if self.values.is_empty() {
            return;
        }

        if let Some(current) = self.cursor {
            if current > 0 {
                self.cursor = Some(current - 1);
            } else {
                self.cursor = Some(self.values.len() - 1);
            }
        } else {
            self.cursor = Some(self.values.len() - 1);
        }
    }

    pub fn unselect(&mut self) {
        self.cursor = None;
    }
}
