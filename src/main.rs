use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep_qoneqo::search;
use minigrep_qoneqo::search_case_insensitive;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// Configuration for the `minigrep` application.
///
/// Holds the search query, target file path, and whether to perform a case-insensitive search.
pub struct Config {
    /// The search query term.
    pub query: String,
    /// The path to the file to be searched.
    pub file_path: String,
    /// Whether to perform a case-insensitive search.
    pub ignore_case: bool,
}

impl Config {
    /// Builds a new `Config` from an iterator over command-line arguments.
    ///
    /// The first argument is typically the path of the executable and is ignored.
    /// Returns `Err` if either the search query or file path is missing.
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the search logic with the given configuration.
///
/// Reads the target file and performs a case-sensitive or case-insensitive search
/// based on the configuration, printing matching lines to standard output.
///
/// # Errors
///
/// Returns an error if the target file cannot be read.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
