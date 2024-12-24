mod enter_terminal;
mod update;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, Default, Subcommand)]
enum Command {
    #[default]
    Run,
    Update,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Update) => update::run()?,
        Some(Command::Run) | None => enter_terminal::run()?,
    }

    Ok(())
}
