
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Parser module for syntax analyzing of source files

use crate::scanner::{Scanner, ScannerMethods, Symbols};

#[derive(Clone, PartialEq, Debug)]
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
	Address( u32, u32, Box<Symbols>, Option<Box<(Box<Symbols>, Box<Node>)>> ),
	Size( u32, u32, Box<Symbols>, Option<Box<(Box<Symbols>, Box<Node>)>> ),
	Alias( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node> ),
	New( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	ParenthesisExpression( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols> ),
	UnaryExpression( u32, u32, Box<Node>, Option<Box<Vec<Box<Node>>>>, Option<Box<Vec<Box<Node>>>> ),
	UnaryPlus( u32, u32, Box<Symbols>, Box<Node> ),
	UnaryMinus( u32, u32, Box<Symbols>, Box<Node> ),
	UnaryNot( u32, u32, Box<Symbols>, Box<Node> ),
	Times( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Slash( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Div( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Mod( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	And( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotTimes( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotSlash( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Backslash( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	TimesTimes( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	PlusTimes( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
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
	fn parse_designator_operator(&mut self) -> Result<Box<Vec<Box<Node>>>, Box<std::string::String>>;
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
	fn parse_flags(&mut self) -> Result<Box<Vec<Box<Node>>>, Box<std::string::String>>;
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
		let start_pos = self.lexer.get_start_position();
		let mut left = self.parse_factor()?;

		loop {
			match self.symbol.clone()? {
				Symbols::Times( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::Times(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::Slash( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::Slash(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::Div( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::Div(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::Mod( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::Mod(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::And( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::And(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::DotTimes( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::DotTimes(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::DotSlash( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::DotSlash(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::BackSlash( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::Backslash(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::TimesTimes( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::TimesTimes(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::PlusTimes( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_factor()?;
					left = Box::new( Node::PlusTimes(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				_ => break
			}
		}

		Ok(left)
	}

	fn parse_factor(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Plus( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_factor()?;

				Ok( Box::new( Node::UnaryPlus(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right) ) )
			},
			Symbols::Minus( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_factor()?;

				Ok( Box::new( Node::UnaryMinus(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right) ) )
			},
			Symbols::Not( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_factor()?;

				Ok( Box::new( Node::UnaryNot(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right) ) )
			},
			_ => self.parse_unary_expression()
		}
	}

	fn parse_unary_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let left = self.parse_primary_expression()?;

		let right = match &self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) | Symbols::Period( _ , _ ) | Symbols::LeftBracket( _ , _ ) |
			Symbols::Arrow( _ , _ ) | Symbols::Transpose( _ , _ ) => Some(self.parse_designator_operator()?),
			_ => {
				None
			}
		};

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some(self.parse_flags()?),
			_ => None
		};

		match (right.clone(), flags.clone()) {
			( None, None ) => Ok(left),
			( _ , _ ) => {
				Ok( Box::new( Node::UnaryExpression(start_pos, self.lexer.get_start_position(), left, right, flags) ) )
			}
		}
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
					Symbols::Address( _ , _ ) => {
						self.advance();
						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::Of( _ , _ ) => {
								self.advance();
								let right = self.parse_factor()?;

								Ok( Box::new(Node::Address(start_pos, self.lexer.get_start_position(), Box::new(x), Some(Box::new( (Box::new(symbol2), right) )))))
							},
							_ => Ok( Box::new(Node::Address(start_pos, self.lexer.get_start_position(), Box::new(x), None)))
						}
					},
					Symbols::Size( _ , _ ) => {
						self.advance();
						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::Of( _ , _ ) => {
								self.advance();
								let right = self.parse_factor()?;

								Ok( Box::new(Node::Size(start_pos, self.lexer.get_start_position(), Box::new(x), Some(Box::new( (Box::new(symbol2), right) )))))
							},
							_ => Ok( Box::new(Node::Size(start_pos, self.lexer.get_start_position(), Box::new(x), None)))
						}
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
						let left = self.parse_qualified_identifier()?;

						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::LeftParen( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting '(' in 'new' expression at position: '{}'", start_pos)))
						}

						let right = self.parse_expression_list()?;

						let symbol3 = self.symbol.clone()?;
						match symbol3 {
							Symbols::RightParen( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting ')' in 'new' expression at position: '{}'", start_pos)))
						}

						Ok( Box::new(Node::New(start_pos, self.lexer.get_start_position(), Box::new(x), left, Box::new(symbol2), right, Box::new(symbol3))))
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

	fn parse_designator_operator(&mut self) -> Result<Box<Vec<Box<Node>>>, Box<String>> {
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

	fn parse_flags(&mut self) -> Result<Box<Vec<Box<Node>>>, Box<String>> {
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

	// Unittests for string insert here!

	#[test]
	fn primary_expression_nil() {
		let mut parser = Parser::new(Box::new(Scanner::new("NIL")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Nil(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*t, Symbols::Nil(0, 3))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_imag() {
		let mut parser = Parser::new(Box::new(Scanner::new("IMAG")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Imag(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*t, Symbols::Imag(0, 4))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_true() {
		let mut parser = Parser::new(Box::new(Scanner::new("TRUE")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::True(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*t, Symbols::True(0, 4))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_false() {
		let mut parser = Parser::new(Box::new(Scanner::new("FALSE")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::False(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
						assert_eq!(*t, Symbols::False(0, 5))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_self() {
		let mut parser = Parser::new(Box::new(Scanner::new("SELF")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Self_(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*t, Symbols::Self_(0, 4))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_result() {
		let mut parser = Parser::new(Box::new(Scanner::new("RESULT")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Result(s, e,t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*t, Symbols::Result(0, 6))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_address() {
		let mut parser = Parser::new(Box::new(Scanner::new("ADDRESS")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Address(s, e,t, None ) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
						assert_eq!(*t, Symbols::Address(0, 7))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	// Unittests for address of variable inserted here!

	#[test]
	fn primary_expression_size() {
		let mut parser = Parser::new(Box::new(Scanner::new("SIZE")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::Size(s, e,t, None ) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*t, Symbols::Size(0, 4))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	// Unittests for alias, new and parenthesis expression inserted here!

	#[test]
	fn unary_expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_unary_expression();

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

	// Unittests for designator and flags inserted here!

	#[test]
	fn factor_expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_factor();

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
	fn factor_expression_plus() {
		let mut parser = Parser::new(Box::new(Scanner::new("+counter")));
		parser.advance();
		let res = parser.parse_factor();

		let pattern = Box::new(Node::UnaryPlus(0, 8, Box::new(Symbols::Plus(0, 1)), Box::new(Node::Ident(1, 8, Box::new(Symbols::Ident(1, 8, Box::new(String::from("counter"))))))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn factor_expression_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("-counter")));
		parser.advance();
		let res = parser.parse_factor();

		let pattern = Box::new(Node::UnaryMinus(0, 8, Box::new(Symbols::Minus(0, 1)), Box::new(Node::Ident(1, 8, Box::new(Symbols::Ident(1, 8, Box::new(String::from("counter"))))))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn factor_expression_not() {
		let mut parser = Parser::new(Box::new(Scanner::new("~counter")));
		parser.advance();
		let res = parser.parse_factor();

		let pattern = Box::new(Node::UnaryNot(0, 8, Box::new(Symbols::Not(0, 1)), Box::new(Node::Ident(1, 8, Box::new(Symbols::Ident(1, 8, Box::new(String::from("counter"))))))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_term();

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
	fn term_expression_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("a * b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Times(0, 5,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::Times(2, 3) ),
											Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_slash() {
		let mut parser = Parser::new(Box::new(Scanner::new("a / b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Slash(0, 5,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::Slash(2, 3) ),
											Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_div() {
		let mut parser = Parser::new(Box::new(Scanner::new("a DIV b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Div(0, 7,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::Div(2, 5) ),
											Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_mod() {
		let mut parser = Parser::new(Box::new(Scanner::new("a MOD b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Mod(0, 7,
										  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										  Box::new( Symbols::Mod(2, 5) ),
										  Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_and() {
		let mut parser = Parser::new(Box::new(Scanner::new("a & b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::And(0, 5,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::And(2, 3) ),
											Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_dot_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .* b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::DotTimes(0, 6,
										  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										  Box::new( Symbols::DotTimes(2, 4) ),
										  Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_dot_slash() {
		let mut parser = Parser::new(Box::new(Scanner::new("a ./ b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::DotSlash(0, 6,
											   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											   Box::new( Symbols::DotSlash(2, 4) ),
											   Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_backslash() {
		let mut parser = Parser::new(Box::new(Scanner::new("a \\ b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Backslash(0, 5,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::BackSlash(2, 3) ),
											Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_times_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("a ** b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::TimesTimes(0, 6,
											   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											   Box::new( Symbols::TimesTimes(2, 4) ),
											   Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn term_expression_plus_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("a +* b")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::PlusTimes(0, 6,
													Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
													Box::new( Symbols::PlusTimes(2, 4) ),
													Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

}