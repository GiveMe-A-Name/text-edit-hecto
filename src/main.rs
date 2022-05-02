#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
#[macro_use]
extern crate clap;
mod args;
mod document;
mod editor;
mod row;
mod terminal;

use args::Args;
use document::Document;
use editor::Editor;
use editor::Position;
use row::Row;
use terminal::Terminal;

pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    Editor::default().run();
}
