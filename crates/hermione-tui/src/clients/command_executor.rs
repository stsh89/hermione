use crate::{data::Command, Result};
use std::{io::Write, process::Stdio};

pub struct Client<'a> {
    pub command: &'a Command,
}

pub struct Output {
    pub stderr: String,
    pub stdout: String,
}

impl<'a> Client<'a> {
    pub fn execute(&self) -> Result<Output> {
        let mut cmd = std::process::Command::new("PowerShell");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut process = cmd.spawn()?;
        let stdin = process
            .stdin
            .as_mut()
            .ok_or(anyhow::anyhow!("stdin access failure"))?;
        writeln!(stdin, "{}", self.command.program).unwrap();
        let out = process.wait_with_output().unwrap();
        let stderr = std::str::from_utf8(out.stderr.as_slice())?.to_string();
        let stdout = std::str::from_utf8(out.stdout.as_slice())?.to_string();
        let output = Output { stderr, stdout };

        Ok(output)
    }

    pub fn new(command: &'a Command) -> Self {
        Self { command }
    }
}
