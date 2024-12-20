// mod coordinator;
// mod handlers;
// mod layouts;
// mod message;
// mod models;
// mod params;
// mod router;
// mod routes;
// mod screen;
// mod smart_input;
// mod themes;
// mod tui;
// mod widgets;

// pub(crate) use handlers::*;
// use hermione_drive::Engine;
// pub(crate) use message::*;
// pub(crate) use params::*;
// pub(crate) use routes::*;

// use coordinator::Coordinator;
// use router::TerminalRouter;

// type Error = anyhow::Error;
// type Result<T> = anyhow::Result<T>;

// fn main() -> Result<()> {
//     let Engine {
//         service_factory,
//         logs_worker_guard: _logs_worker_guard,
//     } = hermione_drive::start()?;

//     let coordinator = Coordinator { service_factory };

//     let router = TerminalRouter {
//         coordinator,
//         theme: themes::github_dark(),
//     };

//     if let Err(err) = tui::run(router) {
//         tracing::error!(error = ?err);
//         return Err(err);
//     };

//     Ok(())
// }

mod keyboard;
mod program;
mod terminal;

fn main() -> anyhow::Result<()> {
    program::run()
}
