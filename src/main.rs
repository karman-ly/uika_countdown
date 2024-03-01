mod app;
mod errors;
mod tui;

use app::App;

fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
