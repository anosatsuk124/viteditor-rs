use std::{
    cmp::{max, min},
    io::{Error, Write},
    path,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Viteditor {
    pub buf: Vec<Vec<char>>,
    pub cursor: Cursor,
    pub row_offset: usize,
}

pub enum KeyEvent {
    Ctrl(char),
    Up,
    Down,
    Left,
    Right,
}

pub struct Position(pub u16, pub u16);

pub trait EditorReader {
    fn event_loop<T: Write>(self, out:&mut T, editor: &mut Viteditor);
}

pub trait Editor {
    fn terminal_size() -> (usize, usize);
    fn clear_all<T: Write>(out: &mut T) -> Result<(), Error>;
    fn goto<T: Write>(out: &mut T, pos: Position) -> Result<(), Error>;
    fn write_str<T: Write>(out: &mut T, str: &str) -> Result<(), Error>;

    fn open(path: &path::Path, editor: &mut Viteditor);
    fn scroll(editor: &mut Viteditor) {
        let (rows, _) = Self::terminal_size();
        editor.row_offset = min(editor.row_offset, editor.cursor.row);
        if editor.cursor.row + 1 >= rows {
            editor.row_offset = max(editor.row_offset, editor.cursor.row + 1 - rows);
        }
    }
    fn cursor_right(editor: &mut Viteditor) {
        editor.cursor.column = min(
            editor.cursor.column + 1,
            editor.buf[editor.cursor.row].len(),
        );
    }
    fn cursor_left(editor: &mut Viteditor) {
        if editor.cursor.column > 0 {
            editor.cursor.column -= 1;
        }
    }
    fn cursor_up(editor: &mut Viteditor) {
        if editor.cursor.row > 0 {
            editor.cursor.row -= 1;
            editor.cursor.column = min(editor.buf[editor.cursor.row].len(), editor.cursor.column);
        }
        Self::scroll(editor);
    }
    fn cursor_down(editor: &mut Viteditor) {
        if editor.cursor.row + 1 < editor.buf.len() {
            editor.cursor.row += 1;
            editor.cursor.column = min(editor.cursor.column, editor.buf[editor.cursor.row].len());
        }
        Self::scroll(editor);
    }
    fn event(event: Option<KeyEvent>, editor: &mut Viteditor) {
            match event {
                Some(KeyEvent::Up) => {
                    Self::cursor_up(editor);
                }
                Some(KeyEvent::Down) => {
                    Self::cursor_down(editor);
                }
                Some(KeyEvent::Left) => {
                    Self::cursor_left(editor);
                }
                Some(KeyEvent::Right) => {
                    Self::cursor_right(editor);
                }
                _ => {}
            }
    }
    fn draw<T: Write>(out: &mut T, editor: &mut Viteditor) {
        let (rows, cols) = Self::terminal_size();

        Self::clear_all(out).unwrap();
        Self::goto(out, Position(1, 1)).unwrap();

        let mut row = 0;
        let mut col = 0;

        let mut display_cursor: Option<(usize, usize)> = None;

        'outer: for i in editor.row_offset..editor.buf.len() {
            for j in 0..=editor.buf[i].len() {
                if editor.cursor == (Cursor { row: i, column: j }) {
                    display_cursor = Some((row, col));
                }

                if let Some(c) = editor.buf[i].get(j) {
                    Self::write_str(out, c.to_string().as_str()).unwrap();
                    col += 1;
                    if col >= cols {
                        row += 1;
                        col = 0;
                        if row >= rows {
                            break 'outer;
                        } else {
                            Self::write_str(out, "\r\n").unwrap();
                        }
                    }
                }
            }
            row += 1;
            col = 0;
            if row >= rows {
                break;
            } else {
                Self::write_str(out, "\r\n").unwrap();
            }
        }

        if let Some((r, c)) = display_cursor {
            Self::goto(out, Position(c as u16 + 1, r as u16 + 1)).unwrap();
        }

        out.flush().unwrap();
    }
}

impl Default for Viteditor {
    fn default() -> Self {
        Self {
            buf: vec![Vec::new()],
            cursor: Cursor { row: 0, column: 0 },
            row_offset: 0,
        }
    }
}
