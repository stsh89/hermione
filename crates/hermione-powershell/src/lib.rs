mod core;

use core::{CopyToClipboard, Error, Execute, OpenWindowsTerminal, PowerShellArguments};
use std::{
    io,
    process::{Child, Command, Stdio},
    sync::RwLock,
};

pub struct Client {
    inner: RwLock<Child>,
}

pub struct PowerShellParameters<'a> {
    pub command: Option<&'a str>,
    pub no_exit: bool,
    pub working_directory: Option<&'a str>,
}

impl Execute for RwLock<Child> {
    fn execute(&self, input: &str) -> Result<(), Error> {
        let mut process = self.write().map_err(|err| eyre::eyre!("{}", err))?;

        let stdin = process.stdin.as_mut().ok_or(eyre::eyre!(
            "Can't obtain handle for writing to the PowerShell's standard input"
        ))?;

        use io::Write;
        writeln!(stdin, "{}", input).map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl Client {
    pub fn new() -> anyhow::Result<Self> {
        let mut cmd = Command::new("pwsh");

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;

        Ok(Self {
            inner: RwLock::new(child),
        })
    }

    pub fn copy_to_clipboard(&self, text: &str) -> anyhow::Result<()> {
        CopyToClipboard {
            powershell: &self.inner,
        }
        .execute(text)?;

        Ok(())
    }

    pub fn open_windows_terminal(
        &self,
        parameters: Option<PowerShellParameters>,
    ) -> anyhow::Result<()> {
        let arguments = parameters.map(|args| {
            let PowerShellParameters {
                working_directory,
                command,
                no_exit,
            } = args;

            PowerShellArguments {
                command: command.map(ToString::to_string),
                no_exit,
                working_directory: working_directory.map(ToString::to_string),
            }
        });

        OpenWindowsTerminal {
            powershell: &self.inner,
        }
        .execute(arguments)?;

        Ok(())
    }
}
