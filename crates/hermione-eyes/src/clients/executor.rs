use crate::types::Result;
use std::{
    io::Write,
    process::{Child, Stdio},
};

pub struct Client<'a> {
    pub program: &'a str,
    pub location: &'a str,
}

impl<'a> Client<'a> {
    pub fn execute(&self) -> Result<()> {
        let mut pwsh = powershell()?;

        let stdin = pwsh
            .stdin
            .as_mut()
            .ok_or(anyhow::anyhow!("Powershell stdin access failure"))?;

        writeln!(stdin, "{}", self.command())?;

        pwsh.wait_with_output()?;

        Ok(())
    }

    fn command(&self) -> String {
        format!(
            "wt pwsh -NoExit -WorkingDirectory {} -Command {{{}}}",
            self.location, self.program
        )
    }
}

fn powershell() -> Result<Child> {
    let mut cmd = std::process::Command::new("pwsh");

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd.spawn()?;

    Ok(child)
}
