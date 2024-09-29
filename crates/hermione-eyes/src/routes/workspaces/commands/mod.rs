use crate::{
    app::Hook, clients::memories::Client, router::workspaces::commands::Router, types::Result,
};

pub mod copy_to_clipboard;
pub mod create;
pub mod delete;
pub mod edit;
pub mod execute;
pub mod get;
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
            Router::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler { memories };

                handler.handle(parameters)?;

                Ok(None)
            }
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
            Router::Execute(parameters) => {
                let handler = execute::Handler { memories };

                handler.handle(parameters)?;

                Ok(None)
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
