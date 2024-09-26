pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub(crate) fn now() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn from_chrono(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }

    pub fn to_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}
