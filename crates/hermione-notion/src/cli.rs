use crate::Result;
use clap::{Parser, Subcommand};

pub trait Run {
    type Command;

    async fn run(self, command: Self::Command) -> Result<()>;
}

#[derive(Debug, Parser)]
#[command(about)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: CliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    DeleteCredentials,
    Import,
    SaveCredentials,
    ShowCredentials,
    Export,
    VerifyCredentials,
}

impl Cli {
    pub async fn run<C>(handler: impl Run<Command = C>) -> Result<()>
    where
        C: From<Cli>,
    {
        let args = Cli::parse();
        let command = args.into();

        handler.run(command).await
    }
}
