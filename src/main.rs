use anyhow::Result;

use crate::app::App;

mod app;
mod sorting;

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
