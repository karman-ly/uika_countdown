use app::App;

mod app;
mod errors;
mod tui;
mod widgets;

fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::new()?.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
