use super::Result;

pub trait SystemService {}

pub trait ExecuteProgram: SystemService {
    fn execute_program(&self, program: &str) -> Result<()>;
}

pub trait SetLocation: SystemService {
    fn set_location(&self, location: Option<&str>) -> Result<()>;
}
