mod editor;
mod terminal;
use editor::Editor;
use terminal::Terminal;

pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    Editor::default().run();
}
