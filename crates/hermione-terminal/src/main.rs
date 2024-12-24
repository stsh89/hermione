mod keyboard;
mod program;
mod program_lib;
mod terminal;

fn main() -> anyhow::Result<()> {
    program::run()
}
