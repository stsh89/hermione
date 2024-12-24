use clap::{Parser, Subcommand};

mod keyboard;
mod program;
mod terminal;

#[derive(Parser)]
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Update) => program::update()?,
        Some(Command::Run) => program::run()?,
        None => program::run()?,
    }

    Ok(())
}
