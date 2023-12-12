
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Traverse Abstract Syntax Tree module for compiling and linking of projects written in ActiveOberon language

use crate::parser::Node;

pub trait TraverseASTMethods {
    fn new() -> Self;
    fn traverse(&mut self, tree: Box<Node> ) -> ();
}

pub struct TraverseAST {

}

impl TraverseASTMethods for TraverseAST {
    fn new() -> Self {
        todo!()
    }

    fn traverse(&mut self, tree: Box<Node>) -> () {
        match &tree {
            Empty => {

            }
        }
    }
}