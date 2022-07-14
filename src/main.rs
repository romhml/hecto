#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod row;
mod terminal;

pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;
use std::io;

fn main() -> Result<(), io::Error> {
    match Editor::default().run() {
        Ok(_) => Ok(()),
        Err(e) => {
            Terminal::clear_screen();
            Err(e)
        }
    }
}
