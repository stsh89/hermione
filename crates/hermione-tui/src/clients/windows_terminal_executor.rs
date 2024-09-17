use std::{io::Write, process::Stdio};

use crate::{entities::Command, Result};

pub struct Client<'a> {
    pub command: &'a Command,
}

impl<'a> Client<'a> {
    pub fn execute(&self) -> Result<()> {
        let program = format!("wt pwsh -Command {{{}; Read-Host}}", self.command.program);
        let mut cmd = std::process::Command::new("PowerShell");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut process = cmd.spawn()?;
        let stdin = process
            .stdin
            .as_mut()
            .ok_or(anyhow::anyhow!("stdin access failure"))?;
        writeln!(stdin, "{}", program)?;
        process.wait_with_output()?;

        Ok(())
    }

    pub fn new(command: &'a Command) -> Self {
        Self { command }
    }
}