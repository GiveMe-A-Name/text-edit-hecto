#[macro_use]
extern crate clap;
mod args;
mod document;
mod editor;
mod terminal;

use args::Args;
use document::Document;
use document::Row;
use editor::Editor;
use editor::Position;
use terminal::Terminal;

pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    Editor::default().run();
}
