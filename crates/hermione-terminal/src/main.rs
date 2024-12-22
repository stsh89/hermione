mod keyboard;
mod program;
mod terminal;

fn main() -> anyhow::Result<()> {
    program::run()
}
