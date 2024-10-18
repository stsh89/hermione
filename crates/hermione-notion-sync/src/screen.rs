use crate::Result;

pub struct Input<'a> {
    /// A prompt that is displayed to the user during the data entry process.
    prompt: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { prompt: name }
    }

    /// Read user input from stdin.
    pub fn read(&self) -> Result<String> {
        use std::io::Write;

        let mut buf = String::new();
        print!("{}: ", self.prompt);
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buf)?;

        Ok(buf.trim().to_string())
    }
}

pub fn clear_and_reset_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print(text: &str) {
    println!("{text}");
}
