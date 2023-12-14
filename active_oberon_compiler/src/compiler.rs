
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Compiler module for compiling and linking of projects written in ActiveOberon language


use std::fs::{File, read_to_string};
use std::io::Read;
use console::style;
use crate::parser::{Parser as ActiveOberonParser, ParserMethods, BlockRules, Node};
use crate::scanner::{Scanner as ActiveOberonScanner, ScannerMethods };
use crate::traverse_abstract_syntax_tree::{TraverseAST, TraverseASTMethods};


pub trait CompilerMethods {
    fn new() -> Self;
    fn compile_module(&mut self, file_name: &String) -> bool;
    /// Present Syntax Error messages correctly with position and source line
    fn present_error_message(&mut self, msg: &String, file_name: &String);
    fn parse_from_file(&mut self, file_name: String) -> Result<Box<Node>, Box<String>>;
}

pub struct Compiler {

}

impl CompilerMethods for Compiler {
    fn new() -> Self {
        Compiler {

        }
    }

    fn compile_module(&mut self, file_name: &String) -> bool {
        let res = self.parse_from_file(String::from(file_name));

        match res {
            Ok( root ) => {
                println!("Success parsing statement!/r/n");

                let mut tree_walker = TraverseAST::new();

                tree_walker.traverse(root);

                true
            },
            Err( s ) => {
                self.present_error_message(&s, file_name);
                false
            }
        }
    }

    /// Present Syntax Error messages correctly with position and source line
    fn present_error_message(&mut self, msg: &String, file_name: &String) {
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

    fn parse_from_file(&mut self, file_name: String) -> Result<Box<Node>, Box<String>> {

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
}
