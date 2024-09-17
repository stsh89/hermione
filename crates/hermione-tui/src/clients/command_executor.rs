use crate::{entities::Command, Result};
use std::{env, io::Write, process::Stdio};

pub struct Client<'a> {
    pub command: &'a Command,
}

pub struct Output {
    pub stderr: String,
    pub stdout: String,
}

impl<'a> Client<'a> {
    pub fn execute(&self) -> Result<Output> {
        let location = env::current_dir()?.display().to_string();
        let mut cmd = std::process::Command::new("pwsh");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut process = cmd.spawn()?;
        let stdin = process
            .stdin
            .as_mut()
            .ok_or(anyhow::anyhow!("stdin access failure"))?;

        let program = format!("cd {}; {}", location, self.command.program);
        writeln!(stdin, "{}", program)?;
        let out = process.wait_with_output()?;
        let stderr = std::str::from_utf8(out.stderr.as_slice())?.to_string();
        let stdout = std::str::from_utf8(out.stdout.as_slice())?.to_string();
        let output = Output { stderr, stdout };

        Ok(output)
    }

    pub fn new(command: &'a Command) -> Self {
        Self { command }
    }
}
