
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Main driver module for compiling and linking of projects written in ActiveOberon language


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
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("\r\n{}, version {} [Build: {}]",
             style("ActiveOberon Compiler").green(),
             style(VERSION).red(),
             style(build_time_local!("%Y-%m-%d")).green());
    println!("Written by Richard Magnor Stenbro. Licensed under GPL V3 - Linux ARM v8 & X86-64\r\n");

    let _args = Cli::parse();
}