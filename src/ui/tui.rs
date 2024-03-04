
use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::Block;

pub fn ui(frame: &mut Frame, songs: Vec<&str>){

    let song_constrains: Vec<Constraint> =  songs.iter().map(|_| Constraint::Length(1)).collect();
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(80),
            Constraint::Percentage(20)
        ]
    ).split(frame.size());

    let top_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
    ).split(main_layout[0]);

    let playlist_layout = Layout::new(
        Direction::Vertical,
        song_constrains.clone()



    ).split(top_layout[0]);


    let playlist_widget = Block::default().title("Playlist").borders(Borders::all());

    let media_widget = Block::default().title("Media").borders(Borders::all());

    let key_bind_widget = Block::default().title("Key binds").borders(Borders::all());






    frame.render_widget(media_widget, top_layout[1]);
    frame.render_widget(key_bind_widget, main_layout[1]);

    for i in 0..song_constrains.clone().len() {

        frame.render_widget(
            Paragraph::new(songs[i]).block(Block::default().borders(Borders::all())),
            playlist_layout[i]);

    }


}