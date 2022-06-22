mod args;

use std::{
    io::{stdin, stdout},
    path,
};

use termion::{raw::IntoRawMode, screen::AlternateScreen};

fn main() {
    let mut state = viteditor_rs::Editor::default();

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Read from the file
    match crate::args::args_parse().unwrap().filepath {
        Some(p) => state.open(path::Path::new(&p)),
        _ => panic!("not exists"),
    };

    state.draw(&mut stdout);

    state.event_loop(stdin, &mut stdout);
}
