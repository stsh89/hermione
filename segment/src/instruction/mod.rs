mod directive;
mod name;

use directive::Directive;
use name::Name;

pub struct Instruction {
    name: Name,
    directive: Directive,
}

pub struct InstructionAttributes {
    pub name: String,
    pub directive: String,
}

impl Instruction {
    pub fn directive(&self) -> &str {
        &self.directive
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(attributes: InstructionAttributes) -> Self {
        let InstructionAttributes { name, directive } = attributes;

        Self {
            name: Name::new(name),
            directive: Directive::new(directive),
        }
    }
}
