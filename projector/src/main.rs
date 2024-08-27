mod app;
// mod model;

use app::App;

fn main() -> std::io::Result<()> {
    let app = App {};

    app.run()?;

    Ok(())
}
