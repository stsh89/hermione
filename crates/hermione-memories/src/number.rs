#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Number(usize);

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number(value)
    }
}

impl From<Number> for usize {
    fn from(value: Number) -> Self {
        value.0
    }
}
