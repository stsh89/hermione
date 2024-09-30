use crate::{app::Hook, clients::memories::Client, router::workspaces::Router, Result};

pub mod commands;
pub mod create;
pub mod delete;
pub mod edit;
pub mod list;
pub mod new;
pub mod update;

pub struct Controller<'a> {
    pub memories: &'a Client,
}

impl<'a> Controller<'a> {
    pub fn run(&self, route: Router) -> Result<Option<Box<dyn Hook>>> {
        let Controller { memories } = self;

        match route {
            Router::Commands(router) => {
                let controller = commands::Controller { memories };

                controller.run(router)
            }
            Router::Create(parameters) => {
                let handler = create::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::Delete(parameters) => {
                let handler = delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::Edit(paramters) => {
                let handler = edit::Handler { memories };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            Router::List(list_parameters) => {
                let handler = list::Handler { memories };

                let model = handler.handle(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::New(_parameters) => {
                let handler = new::Handler {};

                let model = handler.handle();

                Ok(Some(Box::new(model)))
            }
            Router::Update(parameters) => {
                let handler = update::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
