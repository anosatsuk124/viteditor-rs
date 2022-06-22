mod args;

use std::{
    io::{stdin, stdout, Stdin, Write},
    path,
};

use termion::{cursor, clear, event::{Event, Key}, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

use viteditor_rs::{Editor, EditorReader, KeyEvent, Viteditor, Position, Cursor};

struct Input (Stdin);

impl Input {
   fn stdin() -> Self {
       Self(stdin())
   } 
}

impl EditorReader for Input {
    fn event_loop<T: Write>(self, out: &mut T, editor: &mut Viteditor) {
        for event in self.0.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl(c)) => {
                return ;
            },
            Event::Key(Key::Up) => {
TuiEditor::event( Some(KeyEvent::Up), editor);
                TuiEditor::draw(out, editor);
            },
            Event::Key(Key::Down) =>{
TuiEditor::event( Some(KeyEvent::Down), editor);
                TuiEditor::draw(out, editor);
            } ,
            Event::Key(Key::Left) =>{
TuiEditor::event(Some(KeyEvent::Left), editor);
                TuiEditor::draw(out, editor);
            } ,
            Event::Key(Key::Right) => {
TuiEditor::event(Some(KeyEvent::Right), editor);
TuiEditor::draw(out, editor);
            },
            _ => {},
        }
        }
    }
}

struct TuiEditor(Viteditor);

impl Editor for TuiEditor {
    fn open(path: &path::Path, editor: &mut Viteditor) {
        editor.buf = std::fs::read_to_string(path)
            .ok()
            .map(|s| s.lines().map(|line| line.chars().collect()).collect())
            .unwrap();

        editor.cursor = Cursor { row: 0, column: 0 };
        editor.row_offset = 0;
    }
    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }

    fn goto<T: std::io::Write>(out: &mut T, pos: Position) -> std::io::Result<()> {
        write!(out, "{}", cursor::Goto(pos.0, pos.1))
    }

    fn clear_all<T: std::io::Write>(out: &mut T) -> Result<(), std::io::Error> {
        write!(out, "{}", clear::All)
    }

    fn write_str<T: std::io::Write>(out: &mut T, str: &str) -> Result<(), std::io::Error> {
        write!(out, "{}", str)
    }
}

fn main() {
    let mut state = Viteditor::default();

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
