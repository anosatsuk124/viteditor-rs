mod args;

use std::{
    io::{stdin, stdout, Stdin, Write},
    path,
};

use termion::{
    clear, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::{AlternateScreen, ToMainScreen},
};

use viteditor_rs::{words_parser::parser, Editor, State, EditorReader, KeyEvent, Position, Viteditor};

struct Input(Stdin);

impl Input {
    fn stdin() -> Self {
        Self(stdin())
    }
}

impl EditorReader for Input {
    fn event_loop<T: Write>(self, out: &mut T, editor: &mut Viteditor) {
        for event in self.0.events() {
            match event.unwrap() {
                Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) if editor.state == State::Normal => {
                    TuiEditor::event(Some(KeyEvent::Exit), editor);
                    return;
                } // defines exit keys
                Event::Key(Key::Up) => TuiEditor::event(Some(KeyEvent::Up), editor),
                Event::Key(Key::Down) => TuiEditor::event(Some(KeyEvent::Down), editor),
                Event::Key(Key::Left) => TuiEditor::event(Some(KeyEvent::Left), editor),
                Event::Key(Key::Right) => TuiEditor::event(Some(KeyEvent::Right), editor),
                Event::Key(Key::Esc) => TuiEditor::event(Some(KeyEvent::Esc), editor),
                Event::Key(Key::Ctrl(c)) => TuiEditor::event(Some(KeyEvent::Ctrl(c)), editor),
                Event::Key(Key::Char(c)) => TuiEditor::event(Some(KeyEvent::Char(c)), editor),
                _ => {},
            };
            TuiEditor::draw(out, editor);
        }
    }
}

struct TuiEditor(Viteditor);

impl Editor for TuiEditor {
    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }

    fn goto<T: std::io::Write>(out: &mut T, pos: Position) -> std::io::Result<()> {
        write!(out, "{}", cursor::Goto(pos.column as u16, pos.row as u16))
    }

    fn clear_all<T: std::io::Write>(out: &mut T) -> Result<(), std::io::Error> {
        write!(out, "{}", clear::All)
    }

    fn write_str<T: std::io::Write>(out: &mut T, str: &str) -> Result<(), std::io::Error> {
        write!(out, "{}", str)
    }
}

impl TuiEditor {
    fn new() -> Viteditor {
        Viteditor::default()
    }
    fn open(path: &path::Path, editor: &mut Viteditor) {
        let str = std::fs::read_to_string(path).ok();
        editor.words.words_len = parser(&str.clone().unwrap());
        editor.buf = str.map(|s| s.lines().map(|line| line.chars().collect()).collect()).unwrap();
    }
}

fn main() {
    let mut state = TuiEditor::new();

    let stdin = Input::stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Read from the file
    match crate::args::args_parse().unwrap().filepath {
        Some(p) => TuiEditor::open(path::Path::new(&p), &mut state),
        _ => panic!("not exists"),
    };

    TuiEditor::draw(&mut stdout, &mut state);

    stdin.event_loop(&mut stdout, &mut state);
}
