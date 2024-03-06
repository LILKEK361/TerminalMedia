mod ui;



use std::io;
use clap::{Arg, Command, Parser};
use clap::Args;

fn main() -> io::Result<()> {

    ui::build_ui()?;

    Ok(())
}



