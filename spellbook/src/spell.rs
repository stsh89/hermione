use crate::{name::Name, essence::Essence};

pub struct Spell {
    name: Name,
    essence: Essence,
}

pub struct SpellComponents {
    pub name: String,
    pub essence: String,
}

impl Spell {
    pub fn new(spell_components: SpellComponents) -> Self {
        let SpellComponents {
            name,
            essence,
        } = spell_components;

        Self {
            name: Name::new(name),
            essence: Essence::new(essence),
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn essence(&self) -> &Essence {
        &self.essence
    }
}
