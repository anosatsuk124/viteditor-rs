use std::{io::{stdin, stdout, Write}, env, fs};

use termion::{event::{Event, Key}, raw::IntoRawMode, input::TermRead, clear, cursor, screen::AlternateScreen};

fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Read from the file
    let buf: Vec<Vec<char>> = match args_parse().unwrap().filepath {
        Some(p) => fs::read_to_string(p).ok()
            .map(|s| {s.lines()
            .map(|line| line.chars().collect()).collect()}).unwrap(),
        _ => panic!("not exists"),
    };

    // Clear screen
    write!(stdout, "{}", clear::All).unwrap();

    // Set the cursor 1-indexed
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    for line in &buf {
        for &c in line {
            write!(stdout, "{}", c).unwrap();
        }
        write!(stdout, "\r\n").unwrap();
    }

    stdout.flush().unwrap();

    for event in stdin.events() {
        if event.unwrap() == Event::Key(Key::Ctrl('c')) {
            return;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Args {
    filepath: Option<String>,
    help: Option<bool>,
}

fn args_parse() -> Result<Args, ()>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => Ok(Args{filepath: Some(args[1].clone()), help: None}),
        _ => Err(()),
    }
}
