use std::io;
use std::rc::Rc;
use ratatui::{prelude::*, widgets::*};
use ratatui::layout::Flex;
use ratatui::widgets::Block;



pub fn ui(frame: &mut Frame, songs: Vec<&str>, min_song: i32, key_binds: Vec<&str>) {


    let playlist_widget = Block::default().title("Playlist").borders(Borders::all());

    let media_widget = Block::default().title("Media").borders(Borders::all());

    let key_bind_widget = Block::default().title("Key binds").borders(Borders::all());

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(90),
            Constraint::Percentage(10)
        ]
    ).split(frame.size());

    let top_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
    ).split(main_layout[0]);

    let song_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]
    ).split(playlist_widget.clone().inner(top_layout[0]));

    let key_bind_layout = Layout::new(
        Direction::Horizontal,
        constraints_for_vec(key_binds.clone())
    ).split(key_bind_widget.clone().inner(main_layout[1]));

    frame.render_widget(media_widget.clone(), top_layout[1]);
    frame.render_widget(key_bind_widget.clone(), main_layout[1]);
    frame.render_widget(playlist_widget.clone(), top_layout[0]);

    for i in 0..5{
        frame.render_widget(Paragraph::new(songs[(i + min_song) as usize]).block(Block::new().borders(Borders::all())), song_layout[i as usize]);
    }
    for i in 0..key_binds.len(){
        frame.render_widget(Text::raw(key_binds[i]) , key_bind_layout[i]);
    }

}

pub fn constraints_for_vec(vec: Vec<&str>) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    for i in vec.clone() {
        constraints.push(Constraint::Percentage(i.len() as u16 + 2));
    }
    constraints
}
