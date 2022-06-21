mod args;
use std::{
    cmp::{max, min},
    fs,
    io::{stdin, stdout, Write},
    path,
};

use termion::{
    clear, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

#[derive(Eq, PartialEq, Debug)]
struct Cursor {
    row: usize,
    column: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Editor {
    buf: Vec<Vec<char>>,
    cursor: Cursor,
    row_offset: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            buf: vec![Vec::new()],
            cursor: Cursor { row: 0, column: 0 },
            row_offset: 0,
        }
    }
}

impl Editor {
    fn open(&mut self, path: &path::Path) {
        self.buf = fs::read_to_string(path)
            .ok()
            .map(|s| s.lines().map(|line| line.chars().collect()).collect())
            .unwrap();

        self.cursor = Cursor { row: 0, column: 0 };
        self.row_offset = 0;
    }

    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }

    fn draw<T: Write>(&self, out: &mut T) {
        let (rows, cols) = Self::terminal_size();

        write!(out, "{}", clear::All).unwrap();
        write!(out, "{}", cursor::Goto(1, 1)).unwrap();

        let mut row = 0;
        let mut col = 0;

        let mut display_cursor: Option<(usize, usize)> = None;

        'outer: for i in self.row_offset..self.buf.len() {
            for j in 0..=self.buf[i].len() {
                if self.cursor == (Cursor { row: i, column: j }) {
                    display_cursor = Some((row, col));
                }

                if let Some(c) = self.buf[i].get(j) {
                    write!(out, "{}", c).unwrap();
                    col += 1;
                    if col >= cols {
                        row += 1;
                        col = 0;
                        if row >= rows {
                            break 'outer;
                        } else {
                            write!(out, "\r\n").unwrap();
                        }
                    }
                }
            }
            row += 1;
            col = 0;
            if row >= rows {
                break;
            } else {
                write!(out, "\r\n").unwrap();
            }
        }

        if let Some((r, c)) = display_cursor {
            write!(out, "{}", cursor::Goto(c as u16 + 1, r as u16 + 1)).unwrap();
        }

        out.flush().unwrap();
    }
    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cursor.row);
        if self.cursor.row + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cursor.row + 1 - rows);
        }
    }
    fn cursor_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.column = min(self.buf[self.cursor.row].len(), self.cursor.column);
        }
        self.scroll();
    }
    fn cursor_down(&mut self) {
        if self.cursor.row + 1 < self.buf.len() {
            self.cursor.row += 1;
            self.cursor.column = min(self.cursor.column, self.buf[self.cursor.row].len());
        }
        self.scroll();
    }
    fn cursor_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        }
    }
    fn cursor_right(&mut self) {
        self.cursor.column = min(self.cursor.column + 1, self.buf[self.cursor.row].len());
    }
}

fn main() {
    let mut state = Editor::default();

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Read from the file
    match crate::args::args_parse().unwrap().filepath {
        Some(p) => state.open(path::Path::new(&p)),
        _ => panic!("not exists"),
    };

    state.draw(&mut stdout);

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            Event::Key(Key::Up) => {
                state.cursor_up();
            }
            Event::Key(Key::Down) => {
                state.cursor_down();
            }
            Event::Key(Key::Left) => {
                state.cursor_left();
            }
            Event::Key(Key::Right) => {
                state.cursor_right();
            }
            _ => {}
        }
        state.draw(&mut stdout);
    }
}
