mod app;
mod app_error;
mod desktop;
mod organizer;

use app::App;
use app_error::AppError;

type AppResult<T> = Result<T, AppError>;

fn main() -> std::io::Result<()> {
    let app = App {};

    app.run()?;

    Ok(())
}
