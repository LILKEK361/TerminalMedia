mod eventhandler;

mod tui;


use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub fn build_ui() -> io::Result<()>{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    let mut min_song: i32 = 0;
    let mut max_song: i32 = 5;


    let songs = vec!["1","2","3","4","5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
    while !should_quit {

        terminal.draw(|f| tui::ui(f,songs.clone() , min_song.clone(), max_song.clone()))?;
        should_quit = eventhandler::handle_events(&mut min_song, &mut max_song, songs.clone())?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
