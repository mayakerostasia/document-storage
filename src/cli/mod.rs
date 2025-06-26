use clap::{Parser, Subcommand};
pub use upload::Upload;
pub use search::Search;
pub use services::ServiceType;

mod upload;
mod search;
mod services;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = "Quickwit Pipe Uploader";
const LONG_ABOUT: &str = r#"
This is a CLI for Piping JSON data to quickwit. 
It allows you to take output from another command and uploads it to quickwit.
"#;

#[derive(Parser, Debug)]
#[command(version = VERSION, about = ABOUT, long_about = LONG_ABOUT)]
pub struct QuickwitUploader {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Upload(Upload),
    Search(Search),
}

