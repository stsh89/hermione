use super::Result;

pub trait SystemProvider {}

pub trait RunProgram: SystemProvider {
    fn run_program(&self, program: &str) -> Result<()>;
}
