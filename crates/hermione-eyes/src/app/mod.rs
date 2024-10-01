mod message;

use crate::Result;
use ratatui::{backend::Backend, Terminal};
use tracing::instrument;

pub use message::{Hook, Message};

pub struct Parameters<B, H, T>
where
    B: Backend,
    H: Handle<T>,
{
    pub terminal: Terminal<B>,
    pub router: H,
    pub route: T,
}

pub trait Handle<T> {
    fn handle(&self, route: T) -> Result<Option<Box<dyn Hook<T>>>>;
}

#[instrument(skip_all)]
pub fn run<B, H, T>(parameters: Parameters<B, H, T>) -> Result<()>
where
    B: Backend,
    H: Handle<T>,
{
    tracing::info!("App started");

    let Parameters {
        mut terminal,
        router,
        route,
    } = parameters;

    let Some(mut model) = router.handle(route)? else {
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
