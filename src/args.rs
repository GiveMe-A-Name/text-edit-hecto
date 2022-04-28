#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]

pub struct Args {
    ///  open file's path
    pub file: Option<String>,
}
