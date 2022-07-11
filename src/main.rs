#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;

pub use editor::Position;
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
