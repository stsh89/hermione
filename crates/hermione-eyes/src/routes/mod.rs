use crate::{app::Hook, clients::memories::Client, router::Router, types::Result};

pub mod workspaces;

pub struct Controller<'a> {
    pub memories: &'a Client,
}

impl<'a> Controller<'a> {
    pub fn run(self, route: Router) -> Result<Option<Box<dyn Hook>>> {
        let Controller { memories } = self;

        match route {
            Router::Workspaces(route) => workspaces::Controller { memories }.run(route),
        }
    }
}
