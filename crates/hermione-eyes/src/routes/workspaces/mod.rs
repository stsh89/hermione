use crate::{app::Hook, clients::memories::Client, Result};

pub mod commands;
pub mod create;
pub mod delete;
pub mod edit;
pub mod list;
pub mod new;
pub mod update;

pub enum Router {
    Commands(commands::Router),
    Create(create::Parameters),
    Delete(delete::Parameters),
    Edit(edit::Parameters),
    List(list::Parameters),
    New,
    Update(update::Parameters),
}

pub struct RouterParameters<'a> {
    pub memories: &'a Client,
}

impl Router {
    pub fn handle(self, parameters: RouterParameters) -> Result<Option<Box<dyn Hook>>> {
        let RouterParameters { memories } = parameters;

        match self {
            Router::Commands(router) => router.handle(commands::RouterParameters { memories }),
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
            Router::New => {
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
