use super::Result;

pub trait SystemService {}

pub trait ExecuteProgram: SystemService {
    fn execute_program(&self, program: &str) -> Result<()>;
}
