use std::io;
use ratatui::{prelude::*, widgets::*};

use ratatui::widgets::Block;



pub fn ui(frame: &mut Frame, songs: Vec<&str>, min_song: i32, key_binds: Vec<&str>) {

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ]
    ).split(frame.size());

    render_top_bar(frame, main_layout[0]);
    render_main_content(frame, main_layout[1], songs.clone(), min_song);
    render_key_binds(frame, main_layout[2], key_binds.clone());

}

pub fn constraints_for_vec(vec: Vec<&str>) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    for i in vec.clone() {
        constraints.push(Constraint::Percentage(i.len() as u16 + 2));
    }
    constraints
}

fn render_top_bar(frame: &mut Frame, rect: Rect){
    frame.render_widget(
        Block::default().title("Topbar").borders(Borders::all()),
        rect
    )
}

fn render_main_content(frame: &mut Frame, rect: Rect, songs: Vec<&str>, min_song: i32){

    let content_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
    ).split(rect);

    let playlist_widget = Block::default().title("Playlist").borders(Borders::all());

    let media_widget = Block::default().title("Media").borders(Borders::all());

    let song = |name| Paragraph::new(name).block(Block::new().borders(Borders::all()));

    let playlist_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
            Constraint::Percentage(100 / 8),
        ]
    ).split(playlist_widget.inner(content_layout[0]));

    let media_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(100)
        ],
    ).split(media_widget.inner(content_layout[1]));

    frame.render_widget(playlist_widget, content_layout[0]);
    frame.render_widget(media_widget, content_layout[1]);

    for i in 0..8 {
        frame.render_widget(song(songs.clone()[(i + min_song) as usize]), playlist_layout[i as usize])
    }

}

fn render_key_binds(frame: &mut Frame, rect: Rect, key_binds: Vec<&str>) {

    let key_bind_widget = Block::default().title("Key binds").borders(Borders::all());

    let key_bind_layout = Layout::new(
        Direction::Horizontal,
        constraints_for_vec(key_binds.clone()),

    ).split(key_bind_widget.inner(rect));

    frame.render_widget(key_bind_widget, rect);

    for i in 0..key_binds.clone().len(){
        frame.render_widget(Text::raw(key_binds.clone()[i]), key_bind_layout[i]);
    }
}
