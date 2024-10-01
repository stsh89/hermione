pub mod create;
pub mod delete;
pub mod edit;
pub mod get;
pub mod list;
pub mod new;
pub mod update;

use crate::{app::Hook, clients::memories::Client, Result};

pub enum Router {
    Create(create::Parameters),
    Delete(delete::Parameters),
    Edit(edit::Parameters),
    Get(get::Parameters),
    List(list::Parameters),
    New(new::Parameters),
    Update(update::Parameters),
}

pub struct RouterParameters<'a> {
    pub memories: &'a Client,
}

impl Router {
    pub fn handle(self, parameters: RouterParameters) -> Result<Option<Box<dyn Hook>>> {
        let RouterParameters { memories } = parameters;

        match self {
            Router::Create(paramters) => {
                let handler = create::Handler { memories };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            Router::Delete(parameters) => {
                let handler = delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::Edit(parameters) => {
                let handler = edit::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::Get(parameters) => {
                let handler = get::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::List(parameters) => {
                let handler = list::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Router::New(parameters) => {
                let handler = new::Handler { memories };

                let model = handler.handle(parameters)?;

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
