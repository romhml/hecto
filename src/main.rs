#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
mod document;
mod editor;
mod errors;
mod result;
mod row;
mod terminal;

pub use errors::Error;
pub use result::Result;

pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;

fn main() -> Result<()> {
    match Editor::default().run() {
        Ok(_) => Ok(()),
        Err(e) => {
            Terminal::clear_screen();
            Err(e)
        }
    }
}
