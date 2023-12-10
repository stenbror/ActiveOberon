
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Main driver module for compiling and linking of projects written in ActiveOberon language


mod scanner;
mod parser;

use console::style;
use build_time::{build_time_local};
use clap::Parser;
use std::fs::File;
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

    //let _args = Cli::parse();

    let res = parse_from_file(String::from("test.mod"));

    match res {
        Ok( _ ) => println!("Success parsing statement!/r/n"),
        Err( s ) => println!("{}\r\n", s)
    }

}
