
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

            Node::With( _ , _ , _ , id , _ , nodes , node , _ ) => {
                self.traverse(id);

                for el in nodes.iter() {
                    self.traverse(el.clone())
                }

                match node {
                    Some( else_node ) => {
                        self.traverse(else_node)
                    },
                    _ => ()
                }
            },

            Node::WithElement( _ , _ , _ , element , _ , stmt ) => {
                self.traverse(element);
                self.traverse(stmt);
            },

            Node::Case( _ , _ , _ , expr , _ , nodes , node , _ ) => {
                self.traverse(expr);

                for el in nodes.iter() {
                    self.traverse(el.clone())
                }

                match node {
                    Some( else_node ) => {
                        self.traverse(else_node)
                    },
                    _ => ()
                }
            },

            Node::CaseElement( _ , _ , _ , nodes , _ , _ , stmt ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }

                self.traverse(stmt);
            },

            Node::While( _ , _ , _ , expr , _ , stmt , _ ) => {
                self.traverse(expr);
                self.traverse(stmt);
            },

            Node::Repeat( _ , _ , _ , stmt , _ , expr ) => {
                self.traverse(stmt);
                self.traverse(expr);
            },

            Node::For( _ , _ , _ , id , _ , from , _ , to , by , _ , stmt , _ ) => {
                self.traverse(id);
                self.traverse(from);
                self.traverse(to);

                match by {
                    Some( ( _ , by_node ) ) => {
                        self.traverse(by_node)
                    },
                    _ => ()
                }

                self.traverse(stmt);
            },

            Node::Loop( _ , _ , _ , stmt , _ ) => {
                self.traverse(stmt);
            },

            Node::Exit( _ , _ , _ ) => {
                /* No need for elements in */
            },

            Node::Return( _ , _ , _ , expr ) => {
                match expr {
                    Some(expr_node) => {
                        self.traverse(expr_node)
                    },
                    _ => ()

                }
            },

            Node::Await( _ , _ , _ , expr ) => {
                self.traverse(expr);
            },

            Node::Code( _ , _ , _ , code , _ ) => {
                /* Need to handle code later! */
            },

            Node::Ignore( _ , _ , _ , expr ) => {
                self.traverse(expr);
            },

            Node::BecomesStatement( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::ExclaimMarkStatement( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::QuestionmarkStatement( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::LessLessStatement( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::GreaterGreaterStatement( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

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

            Node::TemplateParameters( _ , _ , _ , nodes , _ , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::TemplateParameter( _ , _ , symbol , node ) => {
                match *symbol {
                    Symbols::Const( _ , _  ) => {

                    },
                    Symbols::Type( _ , _ ) => {

                    },
                    _ => ()
                }
            },

            Node::ImportList( _ , _ , _ , nodes , _ , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::Import( _ , _ , id_from , id_to , expr_list , el_ion ) => {
                self.traverse(id_from);

                match id_to {
                    Some ( ( _ , id_to_node ) ) => {
                        self.traverse(id_to_node)
                    },
                    _ => ()
                }

                match expr_list {
                    Some( ( _ , expr_list_node , _ ) ) => {
                        self.traverse(expr_list_node)
                    },
                    _ => ()
                }

                match el_ion {
                    Some( ( _ , el_ion_node) ) => {
                        self.traverse(el_ion_node)
                    },
                    _ => ()
                }
            },

            Node::DeclarationSequence( _ , _ , const_decl , type_decl , var_decl , proc_decl , oper_decl , _ ) => {
                for el in const_decl.iter() {
                    self.traverse(el.clone())
                }

                for el in type_decl.iter() {
                    self.traverse(el.clone())
                }

                for el in var_decl.iter() {
                    self.traverse(el.clone())
                }

                for el in proc_decl.iter() {
                    self.traverse(el.clone())
                }

                for el in oper_decl.iter() {
                    self.traverse(el.clone())
                }
            },

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

            Node::Body( _ , _ , _ , flags , stmt , fin ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node);
                    },
                    _ => ()
                }

                self.traverse(stmt);

                match fin {
                    Some( ( _ , fin_node ) ) => {
                        self.traverse( fin_node )
                    },
                    _ => ()
                }
            },

            Node::BodyCode( _ , _ , _ , _ ) => {
                /* Handle later! */
            },

            Node::TypeDeclarationElement( _ , _ , left , _ , right , _ ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::ArrayType( _ , _ , _ , nodes, _ , type_node ) => {
                match nodes {
                    Some( ( nodes_element, _ ) ) => {
                        for el in nodes_element.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                self.traverse(type_node);
            },

            Node::MathArrayType( _ , _ , _ , nodes , _ , type_node ) => {
                match nodes {
                    Some( ( nodes_element, _ ) ) => {
                        for el in nodes_element.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                self.traverse(type_node);
            },

            Node::MathArraySize( _ , _ , node , symbol ) => {
                match symbol {
                    Some ( x) => {
                        match *x {
                            Symbols::Times( _ , _ ) => {

                            },
                            Symbols::QuestionMark( _ , _ ) => {

                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }

                match node {
                    Some( node_element) => {
                        self.traverse(node_element)
                    },
                    _ => ()
                }
            },

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