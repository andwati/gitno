use color_eyre::Result;
use gitno::app::App;

mod app;
mod github;
mod gitignore;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize error handling
    color_eyre::install()?;

    let mut app = App::new().await?;
    app.run().await
}
