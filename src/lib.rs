mod process;
mod cli;

pub use cli::{Opts, SubCommand, OutputFormat};
pub use process::process_csv;
pub use process::process_genpass;
