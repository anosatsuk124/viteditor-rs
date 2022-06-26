pub mod words_parser;

use std::{
    cmp::{max, min},
    io::{Error, Read, Write},
};

#[derive(Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cursor {
    pub pos: Position,
    pub word_pos: usize,
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Words {
    pub words_index: usize,
    pub char_index: usize,
    pub words_len: Vec<usize>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Viteditor {
    pub buf: Vec<Vec<char>>,
    pub cursor: Cursor,
    pub row_offset: usize,
    pub words: Words,
    pub state: State,
}

#[derive(Debug)]
pub enum KeyEvent {
    Ctrl(char),
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Esc,
    Exit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum State {
    Normal,
    Insert,
    Exit,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

/* FIXME: remove this
impl Position {
    accessor_impl!((set = set_row) row: usize);
    accessor_impl!((set = set_column) column: usize);
}

impl Words {
    accessor_impl!((set = set_wordss_index) words_index: usize);
    accessor_impl!((set = set_char_index) char_index: usize);
}
*/

#[macro_export]
macro_rules! accessor {
    ((get=$getter:ident) : $type:ty ) => {
        fn $getter(&self) -> $type;
    };
    ((set=$setter:ident) : $type:ty ) => {
        fn $setter(&mut self, value: $type);
    };
    ((get=$getter:ident, set=$setter:ident) : $type:ty ) => {
        accessor!((get = $getter): $type);
        accessor!((set = $setter): $type);
    };
}

#[macro_export]
macro_rules! accessor_impl {
    ((get=$getter:ident) ($($name:ident),*) : $type:ty ) => {
        fn $getter(&self) -> $type {
            self.$($name).*
        }
    };
    ((set=$setter:ident) ($($name:ident),*) : $type:ty ) => {
        fn $setter(&mut self, value: $type) {
            self.$($name).* = value;
        }
    };
    ((get=$getter:ident, set=$setter:ident) $name:tt : $type:ty ) => {
        accessor_impl!((get=$getter) $name:$type);
        accessor_impl!((set=$setter) $name:$type);
    };
}

pub trait Editor {
    // FIXME: improve macros usage
    // accessor!((get = get_buf): Vec<Vec<char>>);
    fn get_buf_len(&self) -> usize;
    fn get_buf_get(&self, index: usize) -> Vec<char>;
    fn set_buf_line(&mut self, line: Vec<char>, index: usize);
    accessor!((get = get_row_offset, set = set_row_offset): usize);
    // accessor!((get = get_wordss, set = set_wordss): Words);
    accessor!((get = get_state, set = set_state): State);
    /* TODO:
    accessor!((get = get_words_words_index, set = set_words_words_index): usize);
    accessor!((get = get_words_char_index, set = set_words_char_index): usize);
    accessor!((get = get_words_words_len): Vec<usize>);
    */
    // FIXME: replace to `get_cursor_pos().column`
    // accessor!((get = get_cursor_pos): Position);
    accessor!((get = get_cursor_pos_column, set = set_cursor_pos_column): usize);
    accessor!((get = get_cursor_pos_row, set = set_cursor_pos_row): usize);
    fn event_loop<T: Write, R: Read>(&mut self, input: R, out: &mut T);
    fn terminal_size() -> (usize, usize);
    fn clear_all<T: Write>(out: &mut T) -> Result<(), Error>;
    fn goto<T: Write>(out: &mut T, pos: Position) -> Result<(), Error>;
    fn write_str<T: Write>(out: &mut T, str: &str) -> Result<(), Error>;
    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.set_row_offset(min(self.get_row_offset(), self.get_cursor_pos_row()));
        if self.get_cursor_pos_row() + 1 >= rows {
            self.set_row_offset(max(
                self.get_row_offset(),
                self.get_cursor_pos_row() + 1 - rows,
            ));
        }
    }
    fn cursor_right(&mut self) {
        self.set_cursor_pos_column(min(
            self.get_cursor_pos_column() + 1,
            self.get_buf_get(self.get_cursor_pos_row()).len(),
        ));
    }
    fn cursor_left(&mut self) {
        if self.get_cursor_pos_column() > 0 {
            self.set_cursor_pos_column(self.get_cursor_pos_column() - 1);
        }
    }
    fn cursor_up(&mut self) {
        if self.get_cursor_pos_row() > 0 {
            self.set_cursor_pos_row(self.get_cursor_pos_row() - 1);
            self.set_cursor_pos_column(min(
                self.get_buf_get(self.get_cursor_pos_row()).len(),
                self.get_cursor_pos_column(),
            ));
        }
        self.scroll();
    }
    fn cursor_down(&mut self) {
        if self.get_cursor_pos_row() + 1 < self.get_buf_len() {
            self.set_cursor_pos_row(self.get_cursor_pos_row() + 1);
            self.set_cursor_pos_column(min(
                self.get_cursor_pos_column(),
                self.get_buf_get(self.get_cursor_pos_row()).len(),
            ));
        }
        self.scroll();
    }
    fn insert(&mut self, char: char) {
        let mut buf = self.get_buf_get(self.get_cursor_pos_row());
        buf.insert(self.get_cursor_pos_column(), char);
        self.set_buf_line(buf, self.get_cursor_pos_row());
        // self.get_buf_get(self.get_cursor_pos_row())
        //     .insert(self.get_cursor_pos_column(), char);
        self.cursor_right();
    }
    /* TODO
    fn skip_word(&mut self) {
        for _ in 0..=(self.get_words_words_len()[self.get_words_words_index()]) {
            self.cursor_right();
        }
    }
    */
    fn event(&mut self, event: KeyEvent) {
        match self.get_state() {
            State::Normal => match event {
                KeyEvent::Exit => self.set_state(State::Exit), // Will be implemented in the front-ends.
                KeyEvent::Up | KeyEvent::Char('k') => {
                    self.cursor_up();
                    self.set_state(State::Normal);
                }
                KeyEvent::Down | KeyEvent::Char('j') => {
                    self.cursor_down();
                    self.set_state(State::Normal);
                }
                KeyEvent::Left | KeyEvent::Char('h') => {
                    self.cursor_left();
                    self.set_state(State::Normal);
                }
                KeyEvent::Right | KeyEvent::Char('l') => {
                    self.cursor_right();
                    self.set_state(State::Normal);
                }
                KeyEvent::Char('e') => {
                    // self.skip_word();
                    self.set_state(State::Normal);
                }
                KeyEvent::Char('i') => self.set_state(State::Insert),
                _ => self.set_state(State::Normal),
            },
            State::Insert => match event {
                KeyEvent::Char(c) => {
                    self.insert(c);
                    self.set_state(State::Insert);
                }
                KeyEvent::Esc => self.set_state(State::Normal),
                _ => (),
            },
            State::Exit => self.set_state(State::Exit),
        }
    }
    fn draw<T: Write>(&mut self, out: &mut T) {
        let terminal_size = Position {
            row: Self::terminal_size().0,
            column: Self::terminal_size().1,
        };

        Self::clear_all(out).unwrap();
        // FIXME: Should be 0-indexed
        Self::goto(out, Position { row: 0, column: 0 }).unwrap();

        let mut pos = Position::default();

        let mut display_cursor: Option<Position> = None;

        'outer: for row in self.get_row_offset()..self.get_buf_len() {
            // TODO: implement numbers of row
            // Self::write_str(out, format!("{}\t", i).as_str()).unwrap();

            for column in 0..=self.get_buf_get(row).len() {
                /* TODO: implement moving between words
                if self.get_words_words_len()[self.get_words_words_index()] == ((terminal_size.column * pos.row) + pos.column) {
                    self.set_words_words_index(self.get_words_words_index() + 1);
                    self.set_words_char_index(0);
                } else {
                    self.set_words_char_index(self.get_words_char_index() + 1);
                }
                */
                if (Position {
                    row: self.get_cursor_pos_row(),
                    column: self.get_cursor_pos_column(),
                }) == (Position { row, column })
                {
                    display_cursor = Some(pos);
                }
                if let Some(c) = self.get_buf_get(row).get(column) {
                    Self::write_str(out, c.to_string().as_str()).unwrap();
                    pos.column += 1;
                    if pos.column >= terminal_size.column {
                        pos.row += 1;
                        pos.column = 0;
                        if pos.row >= terminal_size.row {
                            break 'outer;
                        } else {
                            Self::write_str(out, "\r\n").unwrap();
                        }
                    }
                }
            }
            pos.row += 1;
            pos.column = 0;
            if pos.row >= terminal_size.row {
                break;
            } else {
                Self::write_str(out, "\r\n").unwrap();
            }
        }

        if let Some(p) = display_cursor {
            Self::goto(
                out,
                Position {
                    row: p.row,
                    column: p.column,
                },
            )
            .unwrap();
        }

        out.flush().unwrap();
    }
}

impl Default for Viteditor {
    fn default() -> Self {
        Self {
            buf: vec![Vec::new()],
            cursor: Cursor::default(),
            row_offset: 0,
            words: Words::default(),
            state: State::Normal,
        }
    }
}
