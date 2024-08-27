pub struct Instruction {
    name: String,
    directive: String,
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

        Self { name, directive }
    }
}
