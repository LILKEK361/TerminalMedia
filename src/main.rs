mod ui;
mod cmdhandler;

use std::io;
use crate::ui::build_ui;


fn main() -> io::Result<()> {
    build_ui()?;
    Ok(())
}



