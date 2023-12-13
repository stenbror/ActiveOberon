
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Main driver module for compiling and linking of projects written in ActiveOberon language


mod scanner;
mod parser;
mod symbol_table;
mod compiler;
mod traverse_abstract_syntax_tree;

use console::style;
use build_time::{build_time_local};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crate::compiler::{Compiler, CompilerMethods};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Rename output binary file
    #[arg(short, long, value_name = "FILE")]
    out_file: Option<PathBuf>,

    /// Build for release, no debug information
    #[arg(short, long)]
    release: Option<bool>,

    /// Generate code for ARM v8 CPU
    #[arg(long)]
    arm_v8: Option<bool>,

    /// Generate code for X86-64 CPU
    #[arg(long)]
    x86_64: Option<bool>,

    /// Generate code for Risc V 64 bits
    #[arg(long)]
    risc_v: Option<bool>,

    #[arg(short, long)]
    linux: Option<bool>,

    #[arg(short, long)]
    windows: Option<bool>,

    #[arg(short, long)]
    mac_os: Option<bool>,

    #[arg(short, long)]
    dynamic_library: Option<bool>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Build project out of given main module file
    Build {

    },
    /// Compile and not link current module file only
    Compile {
        module_file: String
    },
    /// Check module source file for valid syntax
    Lint {

    },
    /// Build and Execute all tests in project
    Test {

    }
}


fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("\r\n{}, version {} [Build: {}]",
             style("ActiveOberon Compiler").green(),
             style(VERSION).red(),
             style(build_time_local!("%Y-%m-%d")).green());
    println!("Written by Richard Magnor Stenbro. Licensed under GPL V3 - Linux ARM v8 & X86-64 & Risc V - Rust based compiler\r\n");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {}  => {

        },
        Commands::Compile { module_file} => {
            let mut compiler = Compiler::new();

            let _ = compiler.compile_module(module_file);
        },
        Commands::Lint {}  => {

        },
        Commands::Test {}  => {

        },
        _ => { }
    }
}
