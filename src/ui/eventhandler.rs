
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::ui;


pub fn handle_events(min_song: &mut i32,songs: Vec<&str>) -> io::Result<bool> {

    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('w') => {
                        if *min_song > 0 {
                            *min_song -= 1;

                        }
                    }
                    KeyCode::Char('s') => {
                        if *min_song < (songs.len() - 9).try_into().unwrap() {
                            *min_song += 1;


                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}