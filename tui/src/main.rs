mod args;

use std::{
    io::{stdin, stdout, Read, Stdin, Write},
    ops::{Deref, DerefMut},
    path,
};

use termion::{
    clear, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use viteditor_rs::{
    accessor_impl, words_parser::parser, Cursor, Editor, KeyEvent, Position, State, Viteditor,
    Words,
};

// FIXME: Use macro for this definition
struct TuiEditor(Viteditor);

impl Deref for TuiEditor {
    type Target = Viteditor;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TuiEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Editor for TuiEditor {
    // accessor_impl!((get = get_buf) buf: Vec<Vec<char>>);
    accessor_impl!((get = get_row_offset, set = set_row_offset)(row_offset): usize);
    // accessor_impl!((get = get_cursor, set = set_cursor) cursor: Cursor);
    accessor_impl!((get = get_state, set = set_state)(state): State);
    // TODO: fix macro
    fn get_buf_len(&self) -> usize {
        self.buf.len()
    }
    fn get_buf_get(&self, index: usize) -> Vec<char> {
        self.buf[index].clone()
    }
    fn set_buf_line(&mut self, line: Vec<char>, index: usize) {
        self.buf[index] = line;
    }
    /*
    fn set_words_words_index(&mut self,value:usize) {
        self.words.words_index = value;
    }
    fn set_words_char_index(&mut self,value:usize) {
        self.words.char_index = value;
    }
    */
    accessor_impl!(
        (get = get_cursor_pos_column, set = set_cursor_pos_column)(cursor, pos, column): usize
    );
    accessor_impl!((get = get_cursor_pos_row, set = set_cursor_pos_row)(cursor, pos, row): usize);

    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }

    fn event_loop<T: Write, R: Read>(&mut self, input: R, out: &mut T) {
        for event in input.events() {
            match event.unwrap() {
                Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                    self.event(KeyEvent::Exit);
                    return;
                } // defines exit keys
                Event::Key(Key::Up) => self.event(KeyEvent::Up),
                Event::Key(Key::Down) => self.event(KeyEvent::Down),
                Event::Key(Key::Left) => self.event(KeyEvent::Left),
                Event::Key(Key::Right) => self.event(KeyEvent::Right),
                Event::Key(Key::Esc) => self.event(KeyEvent::Esc),
                Event::Key(Key::Ctrl(c)) => self.event(KeyEvent::Ctrl(c)),
                Event::Key(Key::Char(c)) => self.event(KeyEvent::Char(c)),
                _ => {}
            };
            self.draw(out);
        }
    }

    fn goto<T: std::io::Write>(out: &mut T, pos: Position) -> std::io::Result<()> {
        write!(
            out,
            "{}",
            cursor::Goto(pos.column as u16 + 1, pos.row as u16 + 1)
        )
    }

    fn clear_all<T: std::io::Write>(out: &mut T) -> Result<(), std::io::Error> {
        write!(out, "{}", clear::All)
    }

    fn write_str<T: std::io::Write>(out: &mut T, str: &str) -> Result<(), std::io::Error> {
        write!(out, "{}", str)
    }
}

impl TuiEditor {
    fn new() -> Self {
        Self(Viteditor::default())
    }
    fn open(&mut self, path: &path::Path) {
        let str = std::fs::read_to_string(path).ok();
        self.words.words_len = parser(&str.clone().unwrap());
        self.buf = str
            .map(|s| s.lines().map(|line| line.chars().collect()).collect())
            .unwrap();
    }
}

fn main() {
    let mut editor = TuiEditor::new();

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Read from the file
    match crate::args::args_parse().unwrap().filepath {
        Some(p) => editor.open(path::Path::new(&p)),
        _ => panic!("not exists"),
    };

    editor.draw(&mut stdout);

    editor.event_loop(stdin, &mut stdout)
}
