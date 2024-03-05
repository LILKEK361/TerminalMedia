use std::io;
use std::rc::Rc;
use ratatui::{prelude::*, widgets::*};
use ratatui::layout::Flex;
use ratatui::widgets::Block;



pub fn ui(frame: &mut Frame, songs: Vec<&str>, min_song: i32, max_song: i32) {


    let playlist_widget = Block::default().title("Playlist").borders(Borders::all());

    let media_widget = Block::default().title("Media").borders(Borders::all());

    let key_bind_widget = Block::default().title("Key binds").borders(Borders::all());

    let text_style = Style::default()
        .fg(Color::Green)
        .bg(Color::Black)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(85),
            Constraint::Percentage(15)
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



    //let song_layout = ;

    frame.render_widget(media_widget.clone(), top_layout[1]);
    frame.render_widget(key_bind_widget.clone(), main_layout[1]);
    frame.render_widget(playlist_widget.clone(), top_layout[0]);

    let mut song_elements = vec![];

    for i in 0..songs.len() {
        let song = Text::styled(songs[i].to_owned() + "\n Blalalalla",text_style.clone());
        song_elements.push(song);
    }

    for i in 0..5{
        frame.render_widget(Paragraph::new(songs[(i + min_song) as usize]).block(Block::new().borders(Borders::all())), song_layout[i as usize]);
    }

    /*
    let song_list = List::new(song_elements)
        .block(Block::default().borders(Borders::all()).title("Songs"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(song_list, song_layout[0]);
    */
}



