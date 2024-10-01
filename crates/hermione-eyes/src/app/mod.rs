mod message;

use crate::{clients::memories::Client, router::Router, Result};
use ratatui::{backend::Backend, Terminal};
use tracing::instrument;

pub use message::{Handle, Hook, Message};

pub struct App {
    router: Router,
}

pub struct AppParameters {
    pub memories: Client,
}

impl App {
    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { memories } = parameters;

        Ok(Self {
            router: Router { memories },
        })
    }

    #[instrument(skip_all)]
    pub fn run(self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        tracing::info!("App started");

        let App { router } = self;

        let Some(mut model) = router.handle_initial_route()? else {
            return Ok(());
        };

        while model.is_running() {
            terminal.draw(|f| model.view(f))?;

            let mut maybe_message = model.handle_event()?;

            while let Some(message) = maybe_message {
                maybe_message = model.update(message)?;
            }

            if let Some(route) = model.redirect() {
                if let Some(change) = router.handle(route)? {
                    model = change;
                }
            }
        }

        tracing::info!("App stopped");

        Ok(())
    }
}
