#![allow(unused)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the line that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}", args);

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?;
    let mut reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());
    for line in lines_iter {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
