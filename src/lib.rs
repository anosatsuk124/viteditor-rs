pub mod words_parser;

use std::{
    cmp::{max, min},
    io::{Error, Write},
};

#[derive(Eq, PartialEq, Debug)]
pub struct Cursor {
    pub pos: Position,
    pub word_pos: usize,
}

#[derive(Eq, PartialEq, Debug)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position{
    pub row: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 0, column: 0 }
    }
}

pub trait EditorReader {
    fn event_loop<T: Write>(self, out: &mut T, editor: &mut Viteditor);
}

pub trait Editor {
    fn terminal_size() -> (usize, usize);
    fn clear_all<T: Write>(out: &mut T) -> Result<(), Error>;
    fn goto<T: Write>(out: &mut T, pos: Position) -> Result<(), Error>;
    fn write_str<T: Write>(out: &mut T, str: &str) -> Result<(), Error>;
    fn scroll(editor: &mut Viteditor) {
        let (rows, _) = Self::terminal_size();
        editor.row_offset = min(editor.row_offset, editor.cursor.pos.row);
        if editor.cursor.pos.row + 1 >= rows {
            editor.row_offset = max(editor.row_offset, editor.cursor.pos.row + 1 - rows);
        }
    }
    fn cursor_right(editor: &mut Viteditor) {
        editor.cursor.pos.column = min(
            editor.cursor.pos.column + 1,
            editor.buf[editor.cursor.pos.row].len(),
        );
    }
    fn cursor_left(editor: &mut Viteditor) {
        if editor.cursor.pos.column > 0 {
            editor.cursor.pos.column -= 1;
        }
    }
    fn cursor_up(editor: &mut Viteditor) {
        if editor.cursor.pos.row > 0 {
            editor.cursor.pos.row -= 1;
            editor.cursor.pos.column = min(editor.buf[editor.cursor.pos.row].len(), editor.cursor.pos.column);
        }
        Self::scroll(editor);
    }
    fn cursor_down(editor: &mut Viteditor) {
        if editor.cursor.pos.row + 1 < editor.buf.len() {
            editor.cursor.pos.row += 1;
            editor.cursor.pos.column = min(editor.cursor.pos.column, editor.buf[editor.cursor.pos.row].len());
        }
        Self::scroll(editor);
    }
    fn insert(editor: &mut Viteditor, char: char) {
        editor.buf[editor.cursor.pos.row].insert(editor.cursor.pos.column, char);
        Self::cursor_right(editor);
    }
    // FIXME
    fn skip_word(editor: &mut Viteditor) {
        for _ in 0..=(editor.words.words_len[editor.words.words_index]) {
            Self::cursor_right(editor);
        }
    }
    fn event(event: Option<KeyEvent>, editor: &mut Viteditor) {
        editor.state = match editor.state {
            State::Normal => match event.unwrap() {
                    KeyEvent::Exit => State::Exit, // Will be implemented in the front-ends.
                    KeyEvent::Up | KeyEvent::Char('k') => {
                        Self::cursor_up(editor);
                        State::Normal
                    }
                    KeyEvent::Down | KeyEvent::Char('j') => {
                        Self::cursor_down(editor);
                        State::Normal
                    }
                    KeyEvent::Left | KeyEvent::Char('h') => {
                        Self::cursor_left(editor);
                        State::Normal
                    }
                    KeyEvent::Right | KeyEvent::Char('l') => {
                        Self::cursor_right(editor);
                        State::Normal
                    }
                    KeyEvent::Char('e') => {
                        Self::skip_word(editor);
                        State::Normal
                    }
                    KeyEvent::Char('i') => State::Insert,
                    _ => State::Normal,
                }
            State::Insert => match event.unwrap() {
                KeyEvent::Char(c) => {
                    Self::insert(editor, c);
                    State::Insert
                },
                KeyEvent::Esc => State::Normal,
                _ => editor.state,
            },
            State::Exit => match event.unwrap() {
                _ => State::Exit,
            },
        }
    }
    fn draw<T: Write>(out: &mut T, editor: &mut Viteditor) {
        let terminal_size = Position{
            row: Self::terminal_size().0,
            column: Self::terminal_size().1,
        };

        Self::clear_all(out).unwrap();
        // FIXME: Should be 0-indexed
        Self::goto(out, Position { row: 1, column: 1 }).unwrap();

        let mut pos = Position::default();

        let mut display_cursor: Option<Position> = None;

        'outer: for row in editor.row_offset..editor.buf.len() {
            // TODO: implement numbers of row
            // Self::write_str(out, format!("{}\t", i).as_str()).unwrap();

            for column in 0..=editor.buf[row].len() {
                if editor.words.words_len[editor.words.words_index] == ((terminal_size.column * pos.row) + pos.column) {
                    editor.words.words_index += 1;
                    editor.words.char_index = 0;
                } else {
                    editor.words.char_index += 1;
                }
                if editor.cursor.pos == (Position{ row, column }) {
                    display_cursor = Some(pos);
                }
                if let Some(c) = editor.buf[row].get(column) {
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
            Self::goto(out, Position{ row: p.row + 1 as usize, column: p.column + 1 as usize}).unwrap();
        }

        out.flush().unwrap();
    }
}

impl Default for Viteditor {
    fn default() -> Self {
        Self {
            buf: vec![Vec::new()],
            cursor: Cursor { 
                pos: Position::default(), 
                word_pos: 0,
            },
            row_offset: 0,
            words: Words { words_index: 0, char_index: 0, words_len: Vec::new() },
            state: State::Normal,
        }
    }
}
