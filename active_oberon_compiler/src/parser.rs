
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
	Plus( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Minus( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Or( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Range( u32, u32, Option<Box<Node>>, Box<Symbols>, Option<Box<Node>>, Option<Box<Symbols>>, Option<Box<Node>> ),
	Equal( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	NotEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Less( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	LessEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	GreaterEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Greater( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	In( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Is( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotUnequal( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotLess( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotLessEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotGreater( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	DotGreaterEqual( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	QuestionMarks( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	ExplainMarks( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	LessLessQ( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	GreaterGreaterQ( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Array( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	Set( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
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
		let start_pos = self.lexer.get_start_position();
		let left = self.parse_range_expression()?;

		match self.symbol.clone()? {
			Symbols::Equal( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::Equal(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::NotEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::NotEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::Less( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::Less(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::LessEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::LessEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::Greater( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::Greater(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::GreaterEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::GreaterEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::In( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::In(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::Is( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::Is(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::DotEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::DotUnEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotUnequal(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::DotLess( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotLess(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::DotLessEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotLessEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::DotGreaterEqual( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotGreaterEqual(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},

			Symbols::DotGreater( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::DotGreater(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::QuestionMarks( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::QuestionMarks(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::ExclaimMarks( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::ExplainMarks(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::LessLessQ( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::LessLessQ(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			Symbols::GreaterGreaterQ( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_term()?;
				Ok( Box::new( Node::GreaterGreaterQ(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) ))
			},
			_ => Ok(left)
		}
	}

	fn parse_range_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Times( _ , _ ) => {
				let symbol2 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new( Node::Range(start_pos, self.lexer.get_start_position(), None, Box::new(symbol2), None, None, None) ) )
			},
			_ => {
				let mut left : Option<Box<Node>> = None;
				let mut upto  = Box::new(Symbols::Empty);
				let mut right : Option<Box<Node>> = None;
				let mut by : Option<Box<Symbols>> = None;
				let mut next : Option<Box<Node>> = None;

				match self.symbol.clone()? {
					Symbols::Upto( _ , _ ) => (),
					_ => left = Some(self.parse_simple_expression()?)
				}

				match self.symbol.clone()? {
					Symbols::Upto( _ , _ ) => {
						upto = Box::new(self.symbol.clone()?);
						self.advance();

						/* We need to handle all symbols that can follow '..' when right side is empty */
						match self.symbol.clone()? {
							Symbols::Equal( _ , _ ) |
							Symbols::NotEqual( _ , _ ) |
							Symbols::Less( _ , _ ) |
							Symbols::LessEqual( _ , _ ) |
							Symbols::Greater( _ , _ ) |
							Symbols::GreaterEqual( _ , _ ) |
							Symbols::In( _ , _ ) |
							Symbols::Is( _ , _ ) |
							Symbols::DotEqual( _ , _ ) |
							Symbols::DotUnEqual( _ , _ ) |
							Symbols::DotLess( _ , _ ) |
							Symbols::DotLessEqual( _ , _ ) |
							Symbols::DotGreaterEqual( _ , _ ) |
							Symbols::DotGreater( _ , _ ) |
							Symbols::QuestionMarks( _ , _ ) |
							Symbols::ExclaimMarks( _ , _ ) |
							Symbols::LessLessQ( _ , _ ) |
							Symbols::GreaterGreaterQ( _ , _ ) |

							Symbols::EndOfFile( _ ) |
							Symbols::By( _ , _ ) => (),
							_ => right = Some( self.parse_simple_expression()? )
						}

						match self.symbol.clone()? {
							Symbols::By( _ , _ ) => {
								let byc = self.symbol.clone()?;
								by = Some(Box::new( byc ));
								self.advance();
								next = Some( self.parse_simple_expression()? )
							},
							_ => ()
						}

						Ok( Box::new( Node::Range(start_pos, self.lexer.get_start_position(), left, upto, right, by, next ) ) )
					},
					_ => left.ok_or( Box::new(format!("Missing expression at position: '{}'", start_pos)) )
				}
			}
		}
	}

	fn parse_simple_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut left = self.parse_term()?;

		loop {
			match self.symbol.clone()? {
				Symbols::Plus( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_term()?;
					left = Box::new( Node::Plus(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::Minus( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_term()?;
					left = Box::new( Node::Minus(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				Symbols::Or( _ , _ ) => {
					let symbol2 = self.symbol.clone()?;
					self.advance();
					let right = self.parse_term()?;
					left = Box::new( Node::Or(start_pos, self.lexer.get_start_position(), left, Box::new(symbol2), right) )
				},
				_ => break
			}
		}

		Ok(left)
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
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::LeftBracket( _ , _ ) => {
				let _symb1 = self.symbol.clone()?;
				self.advance();
				_symb1
			},
			_ => return Err(Box::new(format!("Expecting '[' in array expression at position: '{}'", start_pos)))
		};

		let mut elements : Vec<Box<Node>> = Vec::new();
		let mut separators : Vec<Box<Symbols>> = Vec::new();

		elements.push( self.parse_expression()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
					elements.push( self.parse_expression()? )
				},
				_ => break
			}
		}

		let symbol2 = match self.symbol.clone()? {
			Symbols::RightBracket( _ , _ ) => {
				let _symb2 = self.symbol.clone()?;
				self.advance();
				_symb2
			},
			_ => return Err(Box::new(format!("Expecting ']' in array expression at position: '{}'", start_pos)))
		};

		Ok( Box::new( Node::Array(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Box::new(elements), Box::new(separators), Box::new(symbol2)) ) )
	}

	fn parse_set(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => {
				let _symb1 = self.symbol.clone()?;
				self.advance();
				_symb1
			},
			_ => return Err(Box::new(format!("Expecting start of set expression at position: '{}'", start_pos)))
		};

		let mut elements : Vec<Box<Node>> = Vec::new();
		let mut separators : Vec<Box<Symbols>> = Vec::new();

		elements.push( self.parse_range_expression()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
					elements.push( self.parse_range_expression()? )
				},
				_ => break
			}
		}

		let symbol2 = match self.symbol.clone()? {
			Symbols::RightBrace( _ , _ ) => {
				let _symb2 = self.symbol.clone()?;
				self.advance();
				_symb2
			},
			_ => return Err(Box::new(format!("Expecting end of set expression at position: '{}'", start_pos)))
		};

		Ok( Box::new( Node::Set(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Box::new(elements), Box::new(separators), Box::new(symbol2)) ) )
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

	#[test]
	fn term_expression_times_div() {
		let mut parser = Parser::new(Box::new(Scanner::new("a * b / c")));
		parser.advance();
		let res = parser.parse_term();

		let pattern = Box::new( Node::Slash(0, 9,
											Box::new( Node::Times(0, 6,
												Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1, Box::new( String::from("a"))) )) ),
												Box::new( Symbols::Times(2, 3) ),
												Box::new( Node::Ident(4, 6, Box::new( Symbols::Ident(4, 5, Box::new( String::from("b"))) )) )
											) ),
											Box::new( Symbols::Slash(6, 7) ),
											Box::new( Node::Ident(8, 9, Box::new( Symbols::Ident(8, 9,Box::new(String::from("c"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_simple_expression();

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
	fn simple_expression_plus() {
		let mut parser = Parser::new(Box::new(Scanner::new("a + b")));
		parser.advance();
		let res = parser.parse_simple_expression();

		let pattern = Box::new( Node::Plus(0, 5,
										  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										  Box::new( Symbols::Plus(2, 3) ),
										  Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_expression_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("a - b")));
		parser.advance();
		let res = parser.parse_simple_expression();

		let pattern = Box::new( Node::Minus(0, 5,
										   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										   Box::new( Symbols::Minus(2, 3) ),
										   Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_expression_or() {
		let mut parser = Parser::new(Box::new(Scanner::new("a OR b")));
		parser.advance();
		let res = parser.parse_simple_expression();

		let pattern = Box::new( Node::Or(0, 6,
										   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										   Box::new( Symbols::Or(2, 4) ),
										   Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_expression_plus_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("a + b - c")));
		parser.advance();
		let res = parser.parse_simple_expression();

		let pattern = Box::new( Node::Minus(0, 9,
											Box::new( Node::Plus(0, 6,
																  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1, Box::new( String::from("a"))) )) ),
																  Box::new( Symbols::Plus(2, 3) ),
																  Box::new( Node::Ident(4, 6, Box::new( Symbols::Ident(4, 5, Box::new( String::from("b"))) )) )
											) ),
											Box::new( Symbols::Minus(6, 7) ),
											Box::new( Node::Ident(8, 9, Box::new( Symbols::Ident(8, 9,Box::new(String::from("c"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_ident() {
		let mut parser = Parser::new(Box::new(Scanner::new("variable1")));
		parser.advance();
		let res = parser.parse_expression();

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
	fn expression_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a = b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Equal(0, 5,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::Equal(2, 3) ),
										 Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_not_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a # b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::NotEqual(0, 5,
											Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											Box::new( Symbols::NotEqual(2, 3) ),
											Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_less() {
		let mut parser = Parser::new(Box::new(Scanner::new("a < b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Less(0, 5,
											   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											   Box::new( Symbols::Less(2, 3) ),
											   Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_greater() {
		let mut parser = Parser::new(Box::new(Scanner::new("a > b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Greater(0, 5,
											   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											   Box::new( Symbols::Greater(2, 3) ),
											   Box::new( Node::Ident(4, 5, Box::new( Symbols::Ident(4, 5,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_less_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a <= b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::LessEqual(0, 6,
											   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
											   Box::new( Symbols::LessEqual(2, 4) ),
											   Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_greater_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a >= b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::GreaterEqual(0, 6,
												Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
												Box::new( Symbols::GreaterEqual(2, 4) ),
												Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_in() {
		let mut parser = Parser::new(Box::new(Scanner::new("a IN b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::In(0, 6,
												   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
												   Box::new( Symbols::In(2, 4) ),
												   Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_is() {
		let mut parser = Parser::new(Box::new(Scanner::new("a IS b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Is(0, 6,
										  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										  Box::new( Symbols::Is(2, 4) ),
										  Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .= b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotEqual(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::DotEqual(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_unequal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .# b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotUnequal(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::DotUnEqual(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_less() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .< b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotLess(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::DotLess(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_greater() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .> b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotGreater(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::DotGreater(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_question_marks() {
		let mut parser = Parser::new(Box::new(Scanner::new("a ?? b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::QuestionMarks(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::QuestionMarks(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_explain_marks() {
		let mut parser = Parser::new(Box::new(Scanner::new("a !! b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::ExplainMarks(0, 6,
										 Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
										 Box::new( Symbols::ExclaimMarks(2, 4) ),
										 Box::new( Node::Ident(5, 6, Box::new( Symbols::Ident(5, 6,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_less_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .<= b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotLessEqual(0, 7,
												   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
												   Box::new( Symbols::DotLessEqual(2, 5) ),
												   Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_dot_greater_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("a .>= b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::DotGreaterEqual(0, 7,
												   Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
												   Box::new( Symbols::DotGreaterEqual(2, 5) ),
												   Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_less_less_q() {
		let mut parser = Parser::new(Box::new(Scanner::new("a <<? b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::LessLessQ(0, 7,
													  Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
													  Box::new( Symbols::LessLessQ(2, 5) ),
													  Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_greater_greater_q() {
		let mut parser = Parser::new(Box::new(Scanner::new("a >>? b")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::GreaterGreaterQ(0, 7,
												Box::new( Node::Ident(0, 2, Box::new( Symbols::Ident(0, 1,Box::new(String::from("a"))) )) ),
												Box::new( Symbols::GreaterGreaterQ(2, 5) ),
												Box::new( Node::Ident(6, 7, Box::new( Symbols::Ident(6, 7,Box::new(String::from("b"))) )) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_range_from_to_by() {
		let mut parser = Parser::new(Box::new(Scanner::new("1 .. 10 BY 2")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Range(0, 12,
											Some(Box::new(Node::Integer(0, 2, Box::new(Symbols::Integer(0, 1, Box::new(String::from("1"))))))),
											Box::new( Symbols::Upto(2, 4)),
											Some(Box::new(Node::Integer(5, 8, Box::new(Symbols::Integer(5, 7, Box::new(String::from("10"))))))),
											Some( Box::new(Symbols::By(8, 10)) ),
											Some(Box::new(Node::Integer(11, 12, Box::new(Symbols::Integer(11, 12, Box::new(String::from("2")))))))  ) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_range_from_empty_to_by() {
		let mut parser = Parser::new(Box::new(Scanner::new(".. 10 BY 2")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Range(0, 10,
											None,
											Box::new( Symbols::Upto(0, 2)),
											Some(Box::new(Node::Integer(3, 6, Box::new(Symbols::Integer(3, 5, Box::new(String::from("10"))))))),
											Some( Box::new(Symbols::By(6, 8)) ),
											Some(Box::new(Node::Integer(9, 10, Box::new(Symbols::Integer(9, 10, Box::new(String::from("2")))))))  ) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_range_from_empty_to_empty_by() {
		let mut parser = Parser::new(Box::new(Scanner::new(".. BY 2")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Range(0, 7,
											None,
											Box::new( Symbols::Upto(0, 2)),
											None,
											Some( Box::new(Symbols::By(3, 5)) ),
											Some(Box::new(Node::Integer(6, 7, Box::new(Symbols::Integer(6, 7, Box::new(String::from("2")))))))  ) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_range_from_empty_to_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("..")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Range(0, 2,
											None,
											Box::new( Symbols::Upto(0, 2)),
											None,
											None,
											None  ) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_range_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("*")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::Range(0, 1,
											None,
											Box::new( Symbols::Times(0, 1)),
											None,
											None,
											None  ) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

}