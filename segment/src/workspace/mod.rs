mod name;
mod number;

use crate::{CoreError, Instruction};
use name::Name;
use number::Number;

pub struct Workspace {
    instructions: Vec<Instruction>,
    name: Name,
    number: Number,
}

pub struct WorkspaceAttributes {
    pub instructions: Vec<Instruction>,
    pub name: String,
    pub number: u32,
}

impl Workspace {
    pub fn build(attributes: WorkspaceAttributes) -> Result<Self, CoreError> {
        let WorkspaceAttributes {
            instructions,
            name,
            number,
        } = attributes;

        Ok(Self {
            name: Name::new(name),
            instructions,
            number: Number::new(number)?,
        })
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> &u32 {
        &self.number
    }
}
