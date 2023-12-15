
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Traverse Abstract Syntax Tree module for compiling and linking of projects written in ActiveOberon language

use crate::parser::Node;
use crate::scanner::Symbols;

pub trait TraverseASTMethods {
    fn new() -> Self;
    fn traverse(&mut self, tree: Box<Node> ) -> ();
}

pub struct TraverseAST {

}

impl TraverseASTMethods for TraverseAST {
    fn new() -> Self {
        TraverseAST {

        }
    }

    fn traverse(&mut self, tree: Box<Node>) -> () {
        match *tree {
            Node::Empty => {}
            Node::Ident(_, _, _) => {}
            Node::Integer(_, _, _) => {}
            Node::Real(_, _, _) => {}
            Node::Character(_, _, _) => {}
            Node::String(_, _, _) => {}
            Node::Nil(_, _, _) => {}
            Node::Imag(_, _, _) => {}
            Node::True(_, _, _) => {}
            Node::False(_, _, _) => {}
            Node::Self_(_, _, _) => {}
            Node::Result(_, _, _) => {}
            Node::Address(_, _, _, _) => {}
            Node::Size(_, _, _, _) => {}
            Node::Alias(_, _, _, _, _) => {}
            Node::New(_, _, _, _, _, _, _) => {}
            Node::ParenthesisExpression(_, _, _, _, _) => {}
            Node::UnaryExpression(_, _, _, _, _) => {}
            Node::UnaryPlus(_, _, _, _) => {}
            Node::UnaryMinus(_, _, _, _) => {}
            Node::UnaryNot(_, _, _, _) => {}
            Node::Times(_, _, _, _, _) => {}
            Node::Slash(_, _, _, _, _) => {}
            Node::Div(_, _, _, _, _) => {}
            Node::Mod(_, _, _, _, _) => {}
            Node::And(_, _, _, _, _) => {}
            Node::DotTimes(_, _, _, _, _) => {}
            Node::DotSlash(_, _, _, _, _) => {}
            Node::Backslash(_, _, _, _, _) => {}
            Node::TimesTimes(_, _, _, _, _) => {}
            Node::PlusTimes(_, _, _, _, _) => {}
            Node::Plus(_, _, _, _, _) => {}
            Node::Minus(_, _, _, _, _) => {}
            Node::Or(_, _, _, _, _) => {}
            Node::Range(_, _, _, _, _, _, _) => {}
            Node::Equal(_, _, _, _, _) => {}
            Node::NotEqual(_, _, _, _, _) => {}
            Node::Less(_, _, _, _, _) => {}
            Node::LessEqual(_, _, _, _, _) => {}
            Node::GreaterEqual(_, _, _, _, _) => {}
            Node::Greater(_, _, _, _, _) => {}
            Node::In(_, _, _, _, _) => {}
            Node::Is(_, _, _, _, _) => {}
            Node::DotEqual(_, _, _, _, _) => {}
            Node::DotUnequal(_, _, _, _, _) => {}
            Node::DotLess(_, _, _, _, _) => {}
            Node::DotLessEqual(_, _, _, _, _) => {}
            Node::DotGreater(_, _, _, _, _) => {}
            Node::DotGreaterEqual(_, _, _, _, _) => {}
            Node::QuestionMarks(_, _, _, _, _) => {}
            Node::ExplainMarks(_, _, _, _, _) => {}
            Node::LessLessQ(_, _, _, _, _) => {}
            Node::GreaterGreaterQ(_, _, _, _, _) => {}
            Node::Array(_, _, _, _, _, _) => {}
            Node::Set(_, _, _, _, _, _) => {}
            Node::ExpressionList(_, _, _, _) => {}
            Node::IndexList(_, _, _, _, _, _, _) => {}
            Node::Call(_, _, _, _, _) => {}
            Node::DotName(_, _, _, _) => {}
            Node::Index(_, _, _, _, _) => {}
            Node::Arrow(_, _, _) => {}
            Node::Transpose(_, _, _) => {}

            Node::StatementSequence( _ , _, nodes , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::StatementBlock( _ , _ , _ , flags , node , _ ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node)
                    },
                    None => ()
                }
                self.traverse(node)
            },

            Node::If( _ , _ , _ , expr , _ , stmt , nodes , node , _ ) => {
                self.traverse(expr);
                self.traverse(stmt);

                match nodes {
                    Some( elsif_nodes) => {
                        for el in elsif_nodes.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                match node {
                    Some( else_node ) => {
                        self.traverse(else_node)
                    },
                    _ => ()
                }
            },

            Node::Elsif( _ , _ , _ , expr , _ , stmt ) => {
                self.traverse(expr);
                self.traverse(stmt);
            },

            Node::Else( _ , _ , _ , stmt ) => {
                self.traverse(stmt);
            },

            Node::With(_, _, _, _, _, _, _, _) => {}
            Node::WithElement(_, _, _, _, _, _) => {}
            Node::Case(_, _, _, _, _, _, _, _) => {}
            Node::CaseElement(_, _, _, _, _, _, _) => {}
            Node::While(_, _, _, _, _, _, _) => {}
            Node::Repeat(_, _, _, _, _, _) => {}
            Node::For(_, _, _, _, _, _, _, _, _, _, _, _) => {}
            Node::Loop(_, _, _, _, _) => {}
            Node::Exit(_, _, _) => {}
            Node::Return(_, _, _, _) => {}
            Node::Await(_, _, _, _) => {}
            Node::Code(_, _, _, _, _) => {}
            Node::Ignore(_, _, _, _) => {}
            Node::BecomesStatement(_, _, _, _, _) => {}
            Node::ExclaimMarkStatement(_, _, _, _, _) => {}
            Node::QuestionmarkStatement(_, _, _, _, _) => {}
            Node::LessLessStatement(_, _, _, _, _) => {}
            Node::GreaterGreaterStatement(_, _, _, _, _) => {}

            Node::Module( _ , _ , _ , template_parameters ,  module_name , in_module , _ , import_list , decl_seq , body , _ , module_ident_end , _ ) => {
                match template_parameters {
                    Some (template) => self.traverse(template),
                    _ => ()
                }

                self.traverse(module_name);

                match in_module {
                    Some( ( _ , in_mod ) ) => self.traverse(in_mod),
                    _ => ()
                }

                match import_list {
                    Some(import_list_nodes) => {
                        for el in import_list_nodes.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                match decl_seq {
                    Some( decl_seq_node ) => self.traverse(decl_seq_node),
                    _ => ()
                }

                match body {
                    Some( body_node ) => self.traverse(body_node),
                    _ => ()
                }

                self.traverse(module_ident_end)
            },

            Node::TemplateParameters(_, _, _, _, _, _) => {}
            Node::TemplateParameter(_, _, _, _) => {}
            Node::ImportList(_, _, _, _, _, _) => {}
            Node::Import(_, _, _, _, _, _) => {}
            Node::DeclarationSequence(_, _, _, _, _, _, _, _) => {}
            Node::ConstDeclaration(_, _, _, _) => {}
            Node::TypeDeclaration(_, _, _, _) => {}
            Node::VarDeclaration(_, _, _, _) => {}
            Node::Const(_, _, _, _, _) => {}
            Node::Var(_, _, _, _, _) => {}
            Node::VarList(_, _, _, _) => {}
            Node::VarName(_, _, _, _, _) => {}
            Node::Flags(_, _, _, _, _, _) => {}
            Node::Flag(_, _, _, _, _) => {}
            Node::Procedure(_, _, _, _, _, _, _, _, _, _, _, _) => {}
            Node::Operator(_, _, _, _, _, _, _, _, _, _, _, _, _) => {}
            Node::FormalParameters(_, _, _, _, _, _, _) => {}
            Node::ParameterDeclaration(_, _, _, _, _, _, _) => {}
            Node::Parameter(_, _, _, _, _) => {}
            Node::Body(_, _, _, _, _, _) => {}
            Node::BodyCode(_, _, _, _) => {}
            Node::TypeDeclarationElement(_, _, _, _, _, _) => {}
            Node::ArrayType(_, _, _, _, _, _) => {}
            Node::MathArrayType(_, _, _, _, _, _) => {}
            Node::MathArraySize(_, _, _, _) => {}
            Node::RecordType(_, _, _, _, _, _, _) => {}
            Node::PointerType(_, _, _, _, _, _) => {}
            Node::ProcedureType(_, _, _, _, _) => {}
            Node::ObjectTypeEmpty(_, _, _) => {}
            Node::ObjectType(_, _, _, _, _, _, _, _, _) => {}
            Node::EnumerationType(_, _, _, _, _, _, _) => {}
            Node::EnumElement(_, _, _, _) => {}
            Node::CellType(_, _, _, _, _, _, _, _, _, _, _) => {}
            Node::PortList(_, _, _, _) => {}
            Node::PortDeclaration(_, _, _, _, _, _) => {}
            Node::PortType(_, _, _, _, _) => {}
            Node::QualifiedIdentifier(_, _, _, _, _) => {}
            Node::IdentifierReadWrite(_, _, _, _) => {}
            Node::IdentifierRead(_, _, _, _) => {}
        }
    }
}