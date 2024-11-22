use crate::{
    services::{ExecuteProgram, SystemService},
    Result,
};

pub struct ExecuteProgramOperation<'a, S>
where
    S: SystemService,
{
    pub system: &'a S,
}

pub struct ExecuteProgramParameters<'a> {
    pub program: &'a str,
}

impl<'a, S> ExecuteProgramOperation<'a, S>
where
    S: ExecuteProgram,
{
    pub fn execute(&self, parameters: ExecuteProgramParameters) -> Result<()> {
        let ExecuteProgramParameters { program } = parameters;

        self.system.execute_program(program)
    }
}
