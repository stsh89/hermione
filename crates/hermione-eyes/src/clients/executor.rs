use crate::types::Result;

pub struct Client<'a> {
    pub program: &'a str,
    pub location: &'a str,
    pub execute_immediately: bool,
}

impl<'a> Client<'a> {
    pub fn execute(&self) -> Result<()> {
        let command = if self.execute_immediately {
            self.program
        } else {
            &format!("Set-Clipboard '{}'", &self.program)
        };

        std::process::Command::new("wt")
            .args([
                "pwsh",
                "-NoExit",
                "-WorkingDirectory",
                self.location,
                "-Command",
                command,
            ])
            .spawn()?;

        Ok(())
    }
}
