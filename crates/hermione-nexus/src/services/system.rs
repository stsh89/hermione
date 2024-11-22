use super::Result;

pub trait SystemService {}

pub struct InvokeCommandParameters<'a> {
    pub command: &'a str,
    pub location: Option<&'a str>,
}

pub trait InvokeCommand: SystemService {
    fn invoke_command(&self, parameters: InvokeCommandParameters) -> Result<()>;
}

pub trait SetLocation: SystemService {
    fn set_location(&self, location: Option<&str>) -> Result<()>;
}
