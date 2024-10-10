use crate::Result;

pub fn clear_and_reset_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn read_stdin(title: &str) -> Result<String> {
    use std::io::Write;

    let mut buf = String::new();
    print!("{title}");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}

pub fn print(text: &str) {
    println!("{text}");
}
