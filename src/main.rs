use std::io::{self, Write};

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let pattern = args.pattern;
    let path = args.path;
    let content = std::fs::read_to_string(&path)
        .with_context(|_| format!("could not read file `{:?}`", path))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(handle, "{}", line);
        }
    }
    Ok(())
}
