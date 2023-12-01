
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Parser module for syntax analyzing of source files

use crate::scanner::{Scanner, ScannerMethods, Symbols};

#[derive()]
pub enum Node {
	Empty,
	Ident( u32, u32, Box<Symbols> ),
	Integer( u32, u32, Box<Symbols> ),
	Real( u32, u32, Box<Symbols> ),
	Character( u32, u32, Box<Symbols> ),
	String( u32, u32, Box<Symbols> ),
	Nil( u32, u32, Box<Symbols> ),
	Imag( u32, u32, Box<Symbols> ),
	True( u32, u32, Box<Symbols> ),
	False( u32, u32, Box<Symbols> ),
	Self_( u32, u32, Box<Symbols> ),
	Result( u32, u32, Box<Symbols> ),
	Address( u32, u32, Box<Symbols>, Box<Node> ),
	Size( u32, u32, Box<Symbols>, Box<Node> ),
	Alias( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node> ),
	New( u32, u32, Box<Symbols>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	ParenthesisExpression( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols> ),
}

pub trait ParserMethods {
	fn new(scanner: Box<Scanner>) -> Parser;
	fn advance(&mut self) -> ();
}

pub trait ExpressionRules {
	fn parse_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_range_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_simple_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_term(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_factor(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_unary_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_primary_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_designator_operator(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_expression_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_index_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_array(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_set(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

pub trait StatementRules {
	fn parse_statement(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_case(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_element_sequence(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

pub trait BlockRules {
	fn parse_module(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_template_parameters(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_template_parameter(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_import_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_constant_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_name_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_name(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_flags(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_flag(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_procedure_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_operator_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_formal_parameters(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_parameter_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_body(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_array_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_math_array_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_math_array_size(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_record_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_procedure_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_object_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_enumeration_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_cell_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_qualified_identifier(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_identifier_definition(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

/// Parser component for ActiveOberon language grammar
pub struct Parser {
	lexer: Box<Scanner>,		/* Lexical analyzer for sourcecode, returning symbols to parser rules */
	symbol: Result<Symbols, Box<String>>	/* Current symbol being handled in parser rule */
}

impl ParserMethods for Parser {
	fn new(scanner: Box<Scanner>) -> Parser {
		Parser {
			lexer: scanner,
			symbol: Ok(Symbols::Empty)
		}
	}
	fn advance(&mut self) -> () {
		self.symbol = self.lexer.get_symbol()
	}
}

/// Implements all expression rules in grammar of ActiveOberon
impl ExpressionRules for Parser {
	fn parse_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_range_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_simple_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_term(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_factor(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_unary_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_primary_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		return match self.symbol.clone() {
			Ok(x) => {
				match x {
					Symbols::Ident( _ , _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Ident(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Integer( _ , _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Integer(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Real( _ , _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Real(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Character( _ , _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Character(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::String( _ , _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::String(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Nil( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Nil(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Imag( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Imag(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::True( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::True(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::False( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::False(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Self_( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Self_(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Result( _ , _ ) => {
						self.advance();
						Ok( Box::new(Node::Result(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::Alias( _ , _ ) => {
						self.advance();
						let left = self.parse_qualified_identifier()?;

						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::Of( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting 'of' in 'alias' expression at position: '{}'", start_pos)))
						}

						let right = self.parse_factor()?;
						Ok( Box::new(Node::Alias(start_pos, self.lexer.get_start_position(), Box::new(x), left, Box::new(symbol2), right)))
					},
					Symbols::New( _ , _ ) => {
						self.advance();

						Ok( Box::new(Node::Result(start_pos, self.lexer.get_start_position(), Box::new(x))))
					},
					Symbols::LeftParen( _ , _ ) => {
						self.advance();

						let right = self.parse_expression()?;

						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::RightParen( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting ')' in parenthesized expression at position: '{}'", start_pos)))
						}

						Ok( Box::new(Node::ParenthesisExpression(start_pos, self.lexer.get_start_position(), Box::new(x), right, Box::new(symbol2))))
					},
					Symbols::LeftBracket( _ , _ ) => {
						self.parse_array()
					},
					Symbols::LeftBrace( _ , _ ) => {
						self.parse_set()
					}
					_ => Err(Box::new(format!("Unexpected or missing literal at position: '{}'", start_pos)))
				}
			},
			Err(e) => {
				Err(e.clone())
			}
		}
	}

	fn parse_designator_operator(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_expression_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_index_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_array(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_set(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

/// Implements all statement rules in grammar of ActiveOberon
impl StatementRules for Parser {
	fn parse_statement(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_case(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_element_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

/// Implements all block rules in grammar of ActiveOberon
impl BlockRules for Parser {
	fn parse_module(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_template_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_template_parameter(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_import_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_constant_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_name_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_name(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_flags(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_flag(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_procedure_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_operator_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_formal_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_parameter_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_body(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_array_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_math_array_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_math_array_size(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_record_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_procedure_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_object_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_enumeration_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_cell_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_qualified_identifier(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_identifier_definition(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

// Unittests for parser rules

#[cfg(test)]
mod tests {
	use crate::parser::{Parser, ParserMethods, Node, ExpressionRules};
	use crate::scanner::{Scanner, ScannerMethods, Symbols};

	#[test]
	fn primary_expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Ident(s, e, t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*t, Symbols::Ident(0, 9, Box::new(String::from("variable1"))))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_integer() {
		let mut parser = Parser::new(Box::new(Scanner::new("123456")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Integer(s, e, t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*t, Symbols::Integer(0, 6, Box::new(String::from("123456"))))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_real() {
		let mut parser = Parser::new(Box::new(Scanner::new("1.034E-56")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Real(s, e, t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*t, Symbols::Real(0, 9, Box::new(String::from("1.034E-56"))))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_character() {
		let mut parser = Parser::new(Box::new(Scanner::new("0FF7EX")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Character(s, e, t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*t, Symbols::Character(0, 6, Box::new(String::from("0FF7EX"))))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}


}