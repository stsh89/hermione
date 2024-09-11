use crate::Result;
use anyhow::anyhow;

pub struct Selector<T> {
    cursor: usize,
    values: Vec<T>,
}

impl<T> Selector<T> {
    pub fn item(&self) -> &T {
        &self.values[self.cursor]
    }

    pub fn items(&self) -> &[T] {
        &self.values
    }

    pub fn item_number(&self) -> usize {
        self.cursor
    }

    pub fn new(values: Vec<T>) -> Result<Self> {
        if values.is_empty() {
            return Err(anyhow!("Selector cannot be empty"));
        }

        let selector = Self { values, cursor: 0 };

        Ok(selector)
    }

    pub fn next(&mut self) {
        self.cursor = if self.cursor < self.values.len() - 1 {
            self.cursor + 1
        } else {
            0
        }
    }

    pub fn previous(&mut self) {
        self.cursor = if self.cursor > 0 {
            self.cursor - 1
        } else {
            self.values.len() - 1
        }
    }
}
