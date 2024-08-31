use handbag::OrganizerError;

pub enum AppError {
    OrganizerError(OrganizerError),
}

impl From<OrganizerError> for AppError {
    fn from(value: OrganizerError) -> Self {
        Self::OrganizerError(value)
    }
}

impl From<AppError> for std::io::Error {
    fn from(_value: AppError) -> Self {
        Self::new(std::io::ErrorKind::Other, "oh no!")
    }
}
