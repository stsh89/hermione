#[derive(Default)]
pub struct Statistics {
    created: u32,
    updated: u32,
    verified: u32,
}

pub enum Action {
    Create,
    Update,
    Verify,
}

impl Statistics {
    pub fn counter(&self, action: Action) -> u32 {
        match action {
            Action::Create => self.created,
            Action::Update => self.updated,
            Action::Verify => self.verified,
        }
    }

    pub fn track_action(&mut self, action: Action) {
        match action {
            Action::Create => self.created += 1,
            Action::Update => self.updated += 1,
            Action::Verify => self.verified += 1,
        };
    }

    pub fn total(&self) -> u32 {
        let total: u32 = 0;

        total
            .saturating_add(self.created)
            .saturating_add(self.updated)
            .saturating_add(self.verified)
    }
}
