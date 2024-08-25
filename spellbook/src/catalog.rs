use crate::Spell;

pub struct Catalog {
    spells: Vec<Spell>,
}

pub enum CatalogError {
    SpellNotFound,
}

impl Catalog {
    pub fn empty() -> Self {
        Self {
            spells: vec![],
        }
    }

    pub fn add(&mut self, spell: Spell) {
        self.spells.push(spell);
    }

    pub fn get(&self, index: usize) -> Result<&Spell, CatalogError> {
        self.spells
            .get(index)
            .ok_or(CatalogError::SpellNotFound)
    }

    pub fn list(&self) -> &[Spell] {
        &self.spells
    }
}
