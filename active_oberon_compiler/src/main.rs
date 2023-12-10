
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Main driver module for compiling and linking of projects written in ActiveOberon language


mod scanner;
mod parser;

use console::style;
use build_time::{build_time_local};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use std::fs::{File, read_to_string};
use std::io::prelude::*;

use crate::parser::{Parser as ActiveOberonParser, ParserMethods, BlockRules, Node};
use crate::scanner::{Scanner as ActiveOberonScanner, ScannerMethods };


/// Parse a given source file into a node tree or error if parsing fails!
fn parse_from_file(file_name: String) -> Result<Box<Node>, Box<String>> {

    let mut file = File::open(file_name.as_str());

    return match &mut file {
        Ok( f ) => {
            let mut contents = String::new();
            let size = f.read_to_string(&mut contents);

            match &size {
                Ok(s) => {
                    match s {
                        0 => Err(Box::new(format!("File '{}' is empty!", style(file_name).red()))),
                        _ => {
                            let mut parser = Box::new( ActiveOberonParser::new( Box::new( ActiveOberonScanner::new( Box::leak(contents.into_boxed_str() ) ) ) ) );
                            let res =  parser.parse_module()?;
                            Ok( res )
                        }
                    }
                },
                _ => Err(Box::new(format!("File '{}' is empty or i am unable to find size!", style(file_name).red())))
            }
        },
        _ => Err(Box::new(format!("Unable to find or open '{}' file.", style(file_name).red())))
    }
}

/// Present Syntax Error messages correctly with position and source line
fn present_error_message(msg: &String, file_name: &String) {
    let parts = msg.split("position: '").collect::<Vec<&str>>();

    if parts.len() < 2 {
        println!("\r\n{} in file: {}", style("SyntaxError").red(), file_name);
        println!("{}", msg);
        return
    }

    let t : String = parts[1].chars().filter(|char| char.is_digit(10)).collect();
    let pos : usize = t.parse::<u32>().unwrap() as usize;

    let mut line : usize = 0;
    let mut col : usize = 0;
    let mut cur_pos : usize = 0;
    let mut syntax_line = String::new();

    let lines : Vec<String> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for source_line in lines {
        let start_of_line = cur_pos;
        cur_pos += source_line.len();
        line += 1;
        if pos > cur_pos {
            continue
        }
        col = (pos - start_of_line) + 1;
        syntax_line = source_line.clone();
        break
    }

    println!("\r\n{} in file: {}", style("SyntaxError").red(), file_name);
    println!("Line: {}, Col: {} => {}\r\n", style(line).yellow(), style(col).yellow(), msg);
    println!("{}", syntax_line);
    println!("{}{}", (0..col).map(|_| " ").collect::<String>(), style("^").red())




}


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
    #[arg(short, long)]
    arm_v8: Option<bool>,

    /// Generate code for X86-64 CPU
    #[arg(short, long)]
    x86_64: Option<bool>,

    /// Generate code for Risc V 64 bits
    #[arg(short, long)]
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
    println!("Written by Richard Magnor Stenbro. Licensed under GPL V3 - Linux ARM v8 & X86-64\r\n");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {}  => {

        },
        Commands::Compile { module_file} => {
            let res = parse_from_file(String::from(module_file));

            match res {
                Ok( _ ) => println!("Success parsing statement!/r/n"),
                Err( s ) => present_error_message(&s, module_file)
            }
        },
        Commands::Lint {}  => {

        },
        Commands::Test {}  => {

        },
        _ => { }
    }
}
