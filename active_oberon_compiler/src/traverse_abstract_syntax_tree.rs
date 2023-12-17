
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
            Node::Empty => {},

            Node::Ident( _ , _ , id ) => {
                /* Handle ident */
            },

            Node::Integer( _ , _ , data ) => {
                /* Handle integer */
            },

            Node::Real( _ , _ , data ) => {
                /* Handle real */
            },

            Node::Character( _ , _ , data ) => {
                /* Handle character */
            },

            Node::String( _ , _ , data ) => {
                /*  Handle string */
            },

            Node::Nil( _ , _ , _ ) => {

            },

            Node::Imag( _ , _ , _ ) => {

            },

            Node::True( _ , _ , _ ) => {

            },

            Node::False( _ , _ , _ ) => {

            },

            Node::Self_( _ , _ , _ ) => {

            },

            Node::Result( _ , _ , _ ) => {

            },

            Node::Address( _ , _ , _ , expr ) => {
                match expr {
                    Some( x ) => {
                        let data = *x;
                        match data {
                            ( _ , node ) => {
                                self.traverse(node)
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            },

            Node::Size( _ , _ , _ , expr ) => {
                match expr {
                    Some( x ) => {
                        let data = *x;
                        match data {
                            ( _ , node ) => {
                                self.traverse(node)
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            },

            Node::Alias( _ , _ , _ , _ , node ) => {
                self.traverse(node);
            },

            Node::New( _ , _ , _ , left , _ , right , _ ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::ParenthesisExpression( _ , _ , _ , node , _ ) => {
                self.traverse(node);
            },

            Node::UnaryExpression( _ , _ , right , designator , flags ) => {
                self.traverse(right);

                match designator {
                    Some( nodes ) => {
                        for el in nodes.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node);
                    },
                    _ => ()
                }
            },

            Node::UnaryPlus( _ , _ , _ , right ) => {
                self.traverse(right);
            },

            Node::UnaryMinus( _ , _ , _ , right ) => {
                self.traverse(right);
            },

            Node::UnaryNot( _ , _ , _ , right ) => {
                self.traverse(right);
            },

            Node::Times( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Slash( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Div( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Mod( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::And( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotTimes( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotSlash( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Backslash( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::TimesTimes( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::PlusTimes( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Plus( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Minus( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Or( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Range( _ , _ , first , _ , second , _ , third ) => {
                match first {
                    Some( x ) => {
                        self.traverse(x)
                    },
                    _ => ()
                }

                match second {
                    Some( x ) => {
                        self.traverse(x)
                    },
                    _ => ()
                }

                match third {
                    Some( x ) => {
                        self.traverse(x)
                    },
                    _ => ()
                }
            },

            Node::Equal( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::NotEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Less( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::LessEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::GreaterEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Greater( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::In( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Is( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotUnequal( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotLess( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotLessEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotGreater( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::DotGreaterEqual( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::QuestionMarks( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::ExplainMarks( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::LessLessQ( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::GreaterGreaterQ( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

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

            Node::ConstDeclaration( _ , _ , _ , nodes ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::TypeDeclaration( _ , _ , _ , nodes ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::VarDeclaration( _ , _ , _ , nodes ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::Const( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::Var( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::VarList( _ , _ , nodes , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone());
                }
            },

            Node::VarName( _ , _ , id , flags , equals ) => {
                self.traverse(id);

                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node);
                    },
                    _ => ()
                }

                match equals {
                    Some( ( symbol , expr_node) ) => {
                        match *symbol {
                            Symbols::Equal( _ , _ ) => {

                            },
                            Symbols::Extern( _ , _ ) => {

                            },
                            _ => ()
                        }

                        self.traverse(expr_node);
                    },
                    _ => ()
                }
            },

            Node::Flags( _ , _ , _ , nodes , _ , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::Flag( _ , _ , id , expr , equals ) => {
                self.traverse(id);

                match expr {
                    Some( ( _ , expr_node , _ ) ) => {
                        self.traverse(expr_node)
                    },
                    _ => ()
                }

                match equals {
                    Some( ( _ , node ) ) => {
                        self.traverse(node)
                    },
                    _ => ()
                }
            },

            Node::Procedure( _ , _ , _ , flags , para_decl , id_def , para , _ , decl , body , _ , id_back ) => {
                match flags {
                    Some ( (flag_node, mark) ) => {
                        match flag_node {
                            Some( flag_node_element) => {
                                self.traverse(flag_node_element)
                            },
                            _ => ()
                        }

                        match mark {
                            Some( mark_element ) => {
                                match *mark_element {
                                    Symbols::Arrow( _ , _ ) => {

                                    },
                                    Symbols::And( _ , _ ) => {

                                    },
                                    Symbols::Not( _ , _ ) => {

                                    },
                                    Symbols::Minus( _ , _ ) => {

                                    },
                                    _ => ()
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }

                self.traverse(id_def);

                match para {
                    Some( el ) => {
                        self.traverse(el)
                    },
                    _ => ()
                }

                match decl {
                    Some( el ) => {
                        self.traverse(el)
                    },
                    _ => ()
                }

                match body {
                    Some( el ) => {
                        self.traverse(el)
                    },
                    _ => ()
                }

                self.traverse(id_back);
            },

            Node::Operator( _ , _ , _ , flags , element1 , id_front , element2 , para , _ , decl , body , _ , id_back ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node);
                    },
                    _ => ()
                }

                match element1 {
                    Some( el1 ) => {
                        match *el1 {
                            Symbols::Minus( _ , _ ) => {

                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }

                self.traverse(id_front);

                match element2 {
                    Some( el2 ) => {
                        match *el2 {
                            Symbols::Minus( _ , _ ) => {

                            },
                            Symbols::Times( _ , _ ) => {

                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }

                self.traverse(para);

                match decl {
                    Some( decl_node ) => {
                        self.traverse(decl_node)
                    },
                    _ => ()
                }

                match body {
                    Some( body_node ) => {
                        self.traverse(body_node)
                    },
                    _ => ()
                }

                self.traverse(id_back);
            },

            Node::FormalParameters( _ , _ , _ , nodes , _ , _ , type_expr ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }

                match type_expr {
                    Some( ( _ , flags2, type_expr_node ) ) => {
                        match flags2 {
                            Some( flags_node2 ) => {
                                self.traverse(flags_node2);
                            },
                            _ => ()
                        }

                        self.traverse(type_expr_node);
                    },
                    _ => ()
                }
            },

            Node::ParameterDeclaration( _ , _ , symbols , nodes , _ , _ , type_expr ) => {
                match symbols {
                    Some( symbol_node ) => {
                        match *symbol_node {
                            Symbols::Var( _ , _ ) => {

                            },
                            Symbols::Const( _ , _ ) => {

                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }

                for el in nodes.iter()  {
                    self.traverse(el.clone())
                }

                self.traverse(type_expr);
            },

            Node::Parameter( _ , _ , id , flags , expr ) => {
                self.traverse(id);

                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node);
                    },
                    _ => ()
                }

                match expr {
                    Some( ( _ , expr_node ) ) => {
                        self.traverse(expr_node)
                    },
                    _ => ()
                }
            },

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

            Node::RecordType( _ , _ , _ , qualident , var_const_decl , proc_oper_decl , _ ) => {
                match qualident {
                    Some( ( _ , qual , _ ) ) => {
                        self.traverse(qual)
                    },
                    _ => ()
                }

                match var_const_decl {
                    Some( ( decl, _ ) ) => {
                        for el in decl.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }

                match proc_oper_decl {
                    Some( ( proc, _ ) ) => {
                        for el in proc.iter() {
                            self.traverse(el.clone())
                        }
                    },
                    _ => ()
                }
            },

            Node::PointerType( _ , _ , _ , flags , _ , type_node ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node)
                    },
                    _ => ()
                }

                self.traverse(type_node);
            },

            Node::ProcedureType( _ , _ , _ , flags , type_node ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node)
                    },
                    _ => ()
                }

                match type_node {
                    Some( type_node_el ) => {
                        self.traverse(type_node_el)
                    },
                    _ => ()
                }
            },

            Node::ObjectTypeEmpty( _ , _ , _ ) => {
                /* Empty Object */
            },

            Node::ObjectType( _ , _ , _ , flags , formal , decl , body , _ , ident ) => {
                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node)
                    },
                    _ => ()
                }

                match formal {
                    Some( ( _ , formal_el , _ ) ) => {
                        self.traverse(formal_el)
                    },
                    _ => ()
                }

                match decl {
                    Some( decl_node ) => {
                        self.traverse(decl_node)
                    },
                    _ => ()
                }

                match body {
                    Some( body_node ) => {
                        self.traverse(body_node)
                    },
                    _ => ()
                }

                match ident {
                    Some( ident_node ) => {
                        self.traverse(ident_node)
                    },
                    _ => ()
                }
            },

            Node::EnumerationType( _ , _ , _ , qualified , nodes , _ , _ ) => {
                match qualified {
                    Some( ( _ , qualident_node, _ ) ) => {
                        self.traverse(qualident_node)
                    },
                    _ => ()
                }

                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::EnumElement( _ , _ , left , right ) => {
                self.traverse(left);

                match right {
                    Some( ( _ , right_node ) ) => {
                        self.traverse(right_node)
                    },
                    _ => ()
                }
            },

            Node::CellType( _ , _ , symbol , flags , port_list , _ , import_list , decl , body , _ , id ) => {
                match *symbol {
                    Symbols::Cell( _ , _ ) => {

                    },
                    Symbols::Cellnet( _ , _ ) => {

                    },
                    _ => ()
                }

                match flags {
                    Some( flags_node ) => {
                        self.traverse(flags_node)
                    },
                    _ => ()
                }

                match port_list {
                    Some( ( _ , port_node, _ ) ) => {
                        self.traverse(port_node)
                    },
                    _ => ()
                }

                match decl {
                    Some( decl_node ) => {
                        self.traverse(decl_node)
                    },
                    _ => ()
                }

                match body {
                    Some( body_node ) => {
                        self.traverse(body_node)
                    },
                    _ => ()
                }

                match id {
                    Some( id_node ) => {
                        self.traverse(id_node)
                    },
                    _ => ()
                }
            },

            Node::PortList( _ , _ , nodes , _ ) => {
                for el in nodes.iter() {
                    self.traverse(el.clone())
                }
            },

            Node::PortDeclaration( _ , _ , nodes , _ , _ , type_node ) => {
                for el in nodes.iter() {
                    match *el.clone() {
                        ( id, Some (flag_node ) ) => {
                            self.traverse(id);
                            self.traverse(flag_node);
                        },
                        ( id , _  ) => {
                            self.traverse(id);
                        }
                        _ => ()
                    }

                }
            },

            Node::PortType( _ , _ , _ , symbol , expr ) => {
                match *symbol {
                    Symbols::In( _ , _ ) => {

                    },
                    Symbols::Out( _ , _ ) => {

                    },
                    _ => ()
                }

                match expr {
                    Some( ( _ , expr_node , _  )) => {
                        self.traverse(expr_node);
                    },
                    _ => ()
                }
            },

            Node::QualifiedIdentifier( _ , _ , left , _ , right ) => {
                self.traverse(left);
                self.traverse(right);
            },

            Node::IdentifierReadWrite( _ , _ , id , _ ) => {
                self.traverse(id);
            },

            Node::IdentifierRead( _ , _ , id , _ ) => {
                self.traverse(id);
            }
        }
    }
}