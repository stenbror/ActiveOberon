mod scanner;
mod parser;

use console::style;
use build_time::{build_time_local};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    println!("\r\n{}, version 0.0.1 [Build: {}]",
             style("ActiveOberon Compiler").green(),
             style(build_time_local!("%Y-%m-%d")).green());
    println!("Written by Richard Magnor Stenbro. Licensed under GPL V3 - Linux ARM v8 & X86-64\r\n");

    let _args = Cli::parse();
}
