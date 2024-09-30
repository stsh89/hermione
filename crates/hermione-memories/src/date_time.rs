#[derive(Clone, PartialEq, PartialOrd)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl From<DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl From<&DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(value: &DateTime) -> Self {
        value.0
    }
}
