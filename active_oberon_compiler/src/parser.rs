
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Parser module for syntax analyzing of source files

use crate::scanner::{Scanner, ScannerMethods, Symbols};

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
	Empty,

	/* Expression nodes */
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
	Alias( u32, u32, Box<Symbols>, Box<Symbols>, Box<Node> ),
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
	ExpressionList( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	IndexList( u32, u32, Option<Box<Node>>, Option<Box<Symbols>>, Option<Box<Symbols>>, Option<Box<Symbols>>, Option<Box<Node>> ),
	Call( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Symbols> ),
	DotName( u32, u32, Box<Symbols>, Box<Node> ),
	Index( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Symbols> ),
	Arrow( u32, u32, Box<Symbols> ),
	Transpose( u32, u32, Box<Symbols> ),

	/* Statement nodes */
	StatementSequence( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	StatementBlock( u32, u32, Box<Symbols>, Option<Box<Vec<Box<Node>>>>, Box<Node>, Box<Symbols> ),
	If( u32, u32, Box<Symbols> , Box<Node>, Box<Symbols>, Box<Node>, Option<Box<Vec<Box<Node>>>>, Option<Box<Node>>, Box<Symbols> ),
	Elsif( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node> ),
	Else( u32, u32, Box<Symbols>, Box<Node> ),
	With( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Vec<Box<Node>>>, Option<Box<Node>>, Box<Symbols> ),
	WithElement( u32, u32, Option<Box<Symbols>>, Box<Node>, Box<Symbols>, Box<Node> ),
	Case( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Vec<Box<Node>>>, Option<Box<Node>>, Box<Symbols> ),
	CaseElement( u32, u32, Option<Box<Symbols>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Box<Node> ),
	While( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	Repeat( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node> ),
	For( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Option<(Box<Symbols>, Box<Node>)>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	Loop( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols> ),
	Exit( u32, u32, Box<Symbols> ),
	Return( u32, u32, Box<Symbols>, Option<Box<Node>> ),
	Await( u32, u32, Box<Symbols>, Box<Node> ),
	Code( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Symbols> ),
	Ignore( u32, u32, Box<Symbols>, Box<Node> ),
	BecomesStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	NotStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	QuestionmarkStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	LessLessStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	GreaterGreaterStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),

	/* Block nodes */
	QualifiedIdentifier( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	IdentifierReadWrite( u32, u32, Box<Node>, Box<Symbols> ),
	IdentifierRead( u32, u32, Box<Node>, Box<Symbols> ),
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
	fn parse_case(&mut self, bar_optional: bool) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_statement_sequence(&mut self) -> Result<Box<Node>, Box<String>>;
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

						let symbol2 = self.symbol.clone()?;
						match symbol2 {
							Symbols::Of( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting 'of' in 'alias' expression at position: '{}'", start_pos)))
						}

						let right = self.parse_factor()?;
						Ok( Box::new(Node::Alias(start_pos, self.lexer.get_start_position(), Box::new(x), Box::new(symbol2), right)))
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
		let mut start_pos = self.lexer.get_start_position();
		let mut elements = Vec::<Box<Node>>::new();

		loop {
			start_pos = self.lexer.get_start_position();

			match self.symbol.clone()? {
				Symbols::LeftParen( _ , _ ) => {
					let symbol1 = self.symbol.clone()?;
					self.advance();

					let right = match self.symbol.clone()? {
						Symbols::RightParen( _ , _ ) => None,
						_ => Some( self.parse_expression_list()? )
					};

					match self.symbol.clone()? {
						Symbols::RightParen( _ , _ ) => {
							let symbol2 = self.symbol.clone()?;
							self.advance();

							elements.push( Box::new( Node::Call(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right, Box::new(symbol2)) ) )
						},
						_ => return Err(Box::new(format!("Missing ')' in call at position: '{}'", start_pos)))
					}
				},
				Symbols::Period( _ , _ ) => {
					let symbol1 = self.symbol.clone()?;
					self.advance();
					match self.symbol.clone()? {
						Symbols::Ident( _ , _ , _ ) => {
							let start_pos2 = self.lexer.get_start_position();
							let symbol2 = self.symbol.clone()?;
							self.advance();
							elements.push( Box::new( Node::DotName(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol2)))) ) )
						},
						_ => return Err(Box::new(format!("Expecting name literal after '.' at position: '{}'", start_pos)))
					}
				},
				Symbols::LeftBracket( _ , _ ) => {
					let symbol1 = self.symbol.clone()?;
					self.advance();

					let right = match self.symbol.clone()? {
						Symbols::RightBracket( _ , _ ) => None,
						_ => Some( self.parse_index_list()? )
					};

					match self.symbol.clone()? {
						Symbols::RightBracket( _ , _ ) => {
							let symbol2 = self.symbol.clone()?;
							self.advance();

							elements.push( Box::new( Node::Index(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right, Box::new(symbol2)) ) )
						},
						_ => return Err(Box::new(format!("Missing ']' in index at position: '{}'", start_pos)))
					}
				},
				Symbols::Arrow( _ , _ ) => {
					let symbol1 = self.symbol.clone()?;
					self.advance();
					elements.push( Box::new( Node::Arrow(start_pos, self.lexer.get_start_position(), Box::new(symbol1)) ) )
				},
				Symbols::Transpose( _ , _ ) => {
					let symbol1 = self.symbol.clone()?;
					self.advance();
					elements.push( Box::new( Node::Transpose(start_pos, self.lexer.get_start_position(), Box::new(symbol1)) ) )
				},
				_ => break
			}
		}

		Ok( Box::new( elements ) )
	}

	fn parse_expression_list(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut elements : Vec<Box<Node>> = Vec::new();
		let mut separators : Vec<Box<Symbols>> = Vec::new();

		elements.push( self.parse_expression()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					match self.lexer.peek_symbol()? {
						Symbols::QuestionMark( _ , _ ) => {
							break
						},
						_ => {
							separators.push( Box::new(self.symbol.clone()?) );
							self.advance()
						}
					}

					elements.push( self.parse_expression()? );
				},
				_ => break
			}
		}

		Ok( Box::new( Node::ExpressionList(start_pos, self.lexer.get_start_position(), Box::new(elements), Box::new(separators)) ) )
	}

	fn parse_index_list(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut left : Option<Box<Node>> = None;
		let mut symbol1 : Option<Box<Symbols>> = None;
		let mut symbol2 : Option<Box<Symbols>> = None;
		let mut symbol3 : Option<Box<Symbols>> = None;
		let mut right : Option<Box<Node>> = None;

		match self.symbol.clone()? {
			Symbols::QuestionMark( _ , _ ) => {
				symbol2 = Some( Box::new( self.symbol.clone()? ) );
				self.advance();
				match self.symbol.clone()? {
					Symbols::Comma( _ , _ ) => {
						symbol3 = Some( Box::new( self.symbol.clone()? ) );
						self.advance();
						right = Some( self.parse_expression_list()? )
					},
					_ => ()
				}
			},
			_ => {
				left = Some( self.parse_expression_list()? );

				match self.symbol.clone()? {
					Symbols::Comma( _ , _ ) => {
						symbol1 = Some( Box::new( self.symbol.clone()? ) );
						self.advance();

						match self.symbol.clone()? {
							Symbols::QuestionMark( _ , _ ) => {
								symbol2 = Some( Box::new( self.symbol.clone()? ) );
								self.advance();
								match self.symbol.clone()? {
									Symbols::Comma( _ , _ ) => {
										symbol3 = Some( Box::new( self.symbol.clone()? ) );
										self.advance();
										right = Some( self.parse_expression_list()? )
									},
									_ => ()
								}
							},
							_ => return Err(Box::new(format!("Expecting '?' in index expression at position: '{}'", start_pos)))
						}
					},
					_ => ()
				}
			}
		}

		Ok( Box::new( Node::IndexList(start_pos, self.lexer.get_start_position(), left, symbol1, symbol2, symbol3, right) ) )
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
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::If( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::Then( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'THEN' in if statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_statement_sequence().clone()?;

				let mut elsif_nodes = Box::new(Vec::<Box<Node>>::new());
				loop {
					let start_pos2 = self.lexer.get_start_position();
					match self.symbol.clone()? {
						Symbols::Elsif( _ , _ ) => {
							let symbol4 = self.symbol.clone()?;
							self.advance();

							let left3 = self.parse_expression()?;

							match self.symbol.clone()? {
								Symbols::Then( _ , _ ) => (),
								_ => return Err(Box::new(format!("Expecting 'THEN' in elsif statement at position: '{}'", start_pos)))
							}
							let symbol5 = self.symbol.clone()?;
							self.advance();

							let right3 = self.parse_statement_sequence()?;

							elsif_nodes.push( Box::new(Node::Elsif(start_pos2, self.lexer.get_start_position(), Box::new(symbol4), left3, Box::new(symbol5), right3 )) )
						},
						_ => break
					}
				}

				let else_node = match self.symbol.clone()? {
					Symbols::Else( _ , _) => {
						let start_pos3 = self.lexer.get_start_position();

						let symbol4 = self.symbol.clone()?;
						self.advance();

						let right2 = self.parse_statement_sequence()?;
						Some( Box::new(Node::Else(start_pos3, self.lexer.get_start_position(), Box::new(symbol4), right2)))
					},
					_ => None
				};

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in if statement at position: '{}'", start_pos)))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				let nodes = match elsif_nodes.len() {
					0 => None,
					_ => Some( elsif_nodes.clone() )
				};

				Ok( Box::new(Node::If(start_pos, self.lexer.get_start_position(), Box::new(symbol1), left, Box::new(symbol2), right.clone(), nodes, else_node.clone(), Box::new(symbol3))) )
			},
			Symbols::With( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let ident = match self.symbol.clone()? {
					Symbols::Ident( _ , _ , _ ) => {
						let start_pos2 = self.lexer.get_start_position();
						let symbol2 = self.symbol.clone()?;
						self.advance();
						Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol2)))
					},
					_ => return Err(Box::new(format!("Expecting Identifier in for statement at position: '{}'", start_pos)))
				};

				match self.symbol.clone()? {
					Symbols::Colon( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ':' in with statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let mut is_first = true;
				let mut nodes = Box::new( Vec::<Box<Node>>::new() );

				loop {
					let mut symbol_local : Option<Box<Symbols>> = None;
					let start_pos5 = self.lexer.get_start_position();
					match is_first {
						true => {
							is_first = false;
							match self.symbol.clone()? {
								Symbols::Bar( _ , _ ) => return Err(Box::new(format!("No '|' at first element in with statement at position: '{}'", start_pos))),
								_ => ()
							}
						},
						_ => {
							match self.symbol.clone()? {
								Symbols::Bar( _ , _ ) => {
									symbol_local = Some(Box::new(self.symbol.clone()?));
									self.advance()
								},
								_ => break
							}
						}
					};
					let left = self.parse_qualified_identifier()?;

					match self.symbol.clone()? {
						Symbols::Do( _ , _ ) => (),
						_ => return Err(Box::new(format!("Expecting 'DO' in with statement at position: '{}'", start_pos)))
					}
					let symbol_local2 = self.symbol.clone()?;
					self.advance();

					let right = self.parse_statement_sequence()?;

					nodes.push( Box::new(Node::WithElement(start_pos5, self.lexer.get_start_position(), symbol_local, left, Box::new(symbol_local2), right)) )
				}

				let else_node = match self.symbol.clone()? {
					Symbols::Else( _ , _) => {
						let start_pos3 = self.lexer.get_start_position();

						let symbol4 = self.symbol.clone()?;
						self.advance();

						let right2 = self.parse_statement_sequence()?;
						Some( Box::new(Node::Else(start_pos3, self.lexer.get_start_position(), Box::new(symbol4), right2)))
					},
					_ => None
				};

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in with statement at position: '{}'", start_pos)))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new( Node::With(start_pos, self.lexer.get_start_position(), Box::new(symbol1), ident, Box::new(symbol2),   nodes, else_node, Box::new(symbol3) ) ) )
			},
			Symbols::Case( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::Of( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'OF' in case statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let mut nodes = Box::new( Vec::<Box<Node>>::new() );
				nodes.push( self.parse_case(true)? );

				loop {
					match self.symbol.clone()? {
						Symbols::Bar( _ , _ ) => {
							nodes.push( self.parse_case(false)? )
						},
						_ => break
					}
				}

				let else_node = match self.symbol.clone()? {
					Symbols::Else( _ , _) => {
						let start_pos3 = self.lexer.get_start_position();

						let symbol4 = self.symbol.clone()?;
						self.advance();

						let right2 = self.parse_statement_sequence()?;
						Some( Box::new(Node::Else(start_pos3, self.lexer.get_start_position(), Box::new(symbol4), right2)))
					},
					_ => None
				};

				match self.symbol.clone()? {
					Symbols::Of( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in case statement at position: '{}'", start_pos)))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new( Node::Case(start_pos, self.lexer.get_start_position(), Box::new(symbol1), left, Box::new(symbol2), nodes, else_node, Box::new(symbol3)) ) )
			},
			Symbols::While( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::Do( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'DO' in while statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in while statement at position: '{}'", start_pos)))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new(Node::While(start_pos, self.lexer.get_start_position(), Box::new(symbol1), left, Box::new(symbol2), right, Box::new(symbol3))) )
			},
			Symbols::Repeat( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::Until( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'UNTIL' in repeat statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_expression()?;

				Ok( Box::new(Node::Repeat(start_pos, self.lexer.get_start_position(), Box::new(symbol1), left, Box::new(symbol2), right)) )
			},
			Symbols::For( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let ident = match self.symbol.clone()? {
					Symbols::Ident( _ , _ , _ ) => {
						let start_pos2 = self.lexer.get_start_position();
						let symbol2 = self.symbol.clone()?;
						self.advance();
						Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol2)))
					},
					_ => return Err(Box::new(format!("Expecting Identifier in for statement at position: '{}'", start_pos)))
				};

				match self.symbol.clone()? {
					Symbols::Becomes( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ':=' in for statement at position: '{}'", start_pos)))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::To( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'TO' in for statement at position: '{}'", start_pos)))
				}
				let symbol4 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_expression()?;

				let by_expr = match self.symbol.clone()? {
					Symbols::By( _ , _ ) => {
						let symbol5 = self.symbol.clone()?;
						self.advance();

						let next = self.parse_expression()?;

						Some( (Box::new(symbol5), next) )
					},
					_ => None
				};

				match self.symbol.clone()? {
					Symbols::Do( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'DO' in for statement at position: '{}'", start_pos)))
				}
				let symbol6 = self.symbol.clone()?;
				self.advance();

				let stmt = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in for statement at position: '{}'", start_pos)))
				}
				let symbol7 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new(Node::For(start_pos, self.lexer.get_start_position(), Box::new(symbol1), ident, Box::new(symbol3), left, Box::new(symbol4), right, by_expr, Box::new(symbol6), stmt, Box::new(symbol7))) )
			},
			Symbols::Loop( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in loop statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				Ok( Box::new(Node::Loop(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right, Box::new(symbol2))) )
			},
			Symbols::Exit( _, _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				Ok( Box::new(Node::Exit(start_pos, self.lexer.get_start_position(), Box::new(symbol1) )))
			},
			Symbols::Return( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				match self.symbol.clone()? {
					Symbols::SemiColon( _ , _ ) |
					Symbols::End( _ , _ ) => {
						Ok( Box::new(Node::Return(start_pos, self.lexer.get_start_position(), Box::new(symbol1), None)) )
					}, _ => {
						let right = self.parse_expression()?;
						Ok( Box::new(Node::Return(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Some(right))) )
					}
				}
			},
			Symbols::Await( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_expression()?;

				Ok( Box::new(Node::Await(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right)) )
			},
			Symbols::Begin( _ , _ ) => self.parse_statement_block(),
			Symbols::Code( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				// Insert inline assembler parser here ....

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in code statement at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();
				Ok( Box::new(Node::Code(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Box::new(Vec::<Box<Node>>::new()), Box::new(symbol2))) )
			},
			Symbols::Ignore( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_expression()?;

				Ok( Box::new(Node::Ignore(start_pos, self.lexer.get_start_position(), Box::new(symbol1), right)) )
			},
			_ => {
				let left = self.parse_expression()?;
				match self.symbol.clone()? {
					Symbols::Becomes( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::BecomesStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
					},
					Symbols::Not( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::NotStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
					},
					Symbols::QuestionMark( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::QuestionmarkStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
					},
					Symbols::GreaterGreater( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::GreaterGreaterStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
					},
					Symbols::LessLess( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::LessLessStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
					},
					_ => {
						Ok(left)
					}
				}
			}
		}
	}

	fn parse_case(&mut self, bar_optional: bool) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut symbol1 = None;

		if !bar_optional {
			match self.symbol.clone()? {
				Symbols::Bar( _ , _ ) => {
					symbol1 = Some( Box::new( self.symbol.clone()? ) );
					self.advance();
				},
				_ => {
					return Err(Box::new(format!("Expecting '|' in case statement at position: '{}'", start_pos)))
				}
			}
		}

		let mut elements = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

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

		let mut symbols2 = Symbols::Empty;

		match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => {
				symbols2 = self.symbol.clone()?;
				self.advance()
			},
			_ => return Err(Box::new(format!("Expecting ':' in case statement at position: '{}'", start_pos)))
		}

		let right = self.parse_statement_sequence()?;

		Ok( Box::new( Node::CaseElement(start_pos, self.lexer.get_start_position(), symbol1, elements, separators, Box::new(symbols2), right ) ) )
	}

	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Begin( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let flags = match self.symbol.clone()? {
					Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
					_ => None
				};

				let right = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => {
						let symbol2 = self.symbol.clone()?;
						self.advance();

						Ok( Box::new( Node::StatementBlock(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, right, Box::new(symbol2)) ) )
					},
					_ => Err(Box::new(format!("Expecting 'END' in statement block at position: '{}'", start_pos)))
				}
			},
			_ => {
				Err(Box::new(format!("Expecting 'BEGIN' in statement block at position: '{}'", start_pos)))
			}
		}
	}

	fn parse_statement_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut nodes = Box::new( Vec::<Box<Node>>::new() );
		let mut separators = Box::new( Vec::<Box<Symbols>>::new() );

		nodes.push( self.parse_statement()? );

		loop {
			match self.symbol.clone()? {
				Symbols::SemiColon( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
					nodes.push( self.parse_statement()? )
				},
				_ => break
			}
		}

		Ok( Box::new( Node::StatementSequence(start_pos, self.lexer.get_start_position(), nodes, separators) ) )
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
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting identifier at position: '{}'", start_pos)))
		}
		let symbol = self.symbol.clone()?;
		self.advance();
		let ident = Box::new(Node::Ident(start_pos, self.lexer.get_start_position(), Box::new(symbol)));

		return match self.symbol.clone()? {
			Symbols::Period( _ , _ ) => {
				let symbol_period = self.symbol.clone()?;
				self.advance();

				let start_pos2 = self.lexer.get_start_position();

				match self.symbol.clone()? {
					Symbols::Ident( _ , _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting identifier after '.' at position: '{}'", start_pos)))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();
				let ident2 = Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol2)));

				Ok( Box::new(Node::QualifiedIdentifier(start_pos, self.lexer.get_start_position(), ident, Box::new(symbol_period), ident2)) )
			},
			_ => Ok( ident )
		}
	}

	fn parse_identifier_definition(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		return match self.symbol.clone()? {
			Symbols::Ident( _, _ , _ ) => {
				let symbol = self.symbol.clone()?;
				self.advance();
				let ident = Box::new(Node::Ident(start_pos, self.lexer.get_start_position(), Box::new(symbol)));

				match self.symbol.clone() ? {
					Symbols::Times( _ , _ ) => { /* Export read / write */
						let symbol_read_write = self.symbol.clone()?;
						self.advance();

						Ok( Box::new(Node::IdentifierReadWrite(start_pos, self.lexer.get_start_position(), ident, Box::new(symbol_read_write))) )
					},
					Symbols::Minus( _ , _ ) => { /* Export read only */
						let symbol_read = self.symbol.clone()?;
						self.advance();

						Ok( Box::new(Node::IdentifierRead(start_pos, self.lexer.get_start_position(), ident, Box::new(symbol_read))) )
					},
					_ => {	/* No export out of module */
						Ok( ident )
					}
				}
			},
			_ => Err(Box::new(format!("Expecting identifier at position: '{}'", start_pos)))
		}
	}
}

// Unittests for parser rules

#[cfg(test)]
mod tests {
	use clap::builder::NonEmptyStringValueParser;
	use crate::parser::{Parser, ParserMethods, Node, ExpressionRules, BlockRules, StatementRules};
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

	#[test]
	fn primary_expression_string() {
		let mut parser = Parser::new(Box::new(Scanner::new("'This is a test!'")));
		parser.advance();
		let res = parser.parse_primary_expression();

		match res {
			Ok(x) => {
				match *x {
					Node::String(s, e, t) => {
						assert_eq!(s, 0);
						assert_eq!(e, 17);
						assert_eq!(*t, Symbols::String(0, 17, Box::new(String::from("'This is a test!'"))))
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

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

	#[test]
	fn primary_expression_size_of() {
		let mut parser = Parser::new(Box::new(Scanner::new("SIZE OF a")));
		parser.advance();
		let res = parser.parse_primary_expression();

		let opt = (Box::new(Symbols::Of(5, 7)), Box::new( Node::Ident( 8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a"))))) ));
		let pattern = Box::new( Node::Size(0, 9, Box::new(Symbols::Size(0, 4)), Some(Box::new(opt))) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_address_of() {
		let mut parser = Parser::new(Box::new(Scanner::new("ADDRESS OF a")));
		parser.advance();
		let res = parser.parse_primary_expression();

		let opt = (Box::new(Symbols::Of(8, 10)), Box::new( Node::Ident( 11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a"))))) ));
		let pattern = Box::new( Node::Address(0, 12, Box::new(Symbols::Address(0, 7)), Some(Box::new(opt))) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn primary_expression_alias_of() {
		let mut parser = Parser::new(Box::new(Scanner::new("ALIAS OF a")));
		parser.advance();
		let res = parser.parse_primary_expression();

		let pattern = Box::new( Node::Alias(
			0,
			10,
			Box::new(Symbols::Alias(0, 5)),
			Box::new( Symbols::Of(6, 8)),
			Box::new( Node::Ident(9, 10, Box::new( Symbols::Ident(9, 10, Box::new(String::from("a"))) )) )
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	// Unittests for new  expression inserted here!

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

	#[test]
	fn primary_expression_parenthesized() {
		let mut parser = Parser::new(Box::new(Scanner::new("( a + b )")));
		parser.advance();
		let res = parser.parse_expression();

		let pattern = Box::new( Node::ParenthesisExpression(
			0,
			9,
			Box::new(Symbols::LeftParen(0, 1)),
			Box::new( Node::Plus(2, 8,
								 Box::new( Node::Ident(2, 4, Box::new(Symbols::Ident(2, 3, Box::new(String::from("a"))))) ),
								 Box::new(Symbols::Plus(4, 5)),
								 Box::new( Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("b"))))) )) ),
			Box::new(Symbols::RightParen(8, 9))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	// Unittests for flags inserted here!

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

	#[test]
	fn expression_array_one() {
		let mut parser = Parser::new(Box::new(Scanner::new("[ 1 ]")));
		parser.advance();
		let res = parser.parse_expression();

		let elements= [ Box::new( Node::Integer(2, 4, Box::new( Symbols::Integer( 2, 3, Box::new( String::from("1" )))))) ].to_vec();

		let pattern = Box::new( Node::Array(0, 5,
												Box::new( Symbols::LeftBracket(0, 1) ),
												Box::new( elements ) ,
												Box::new( Vec::new() ),
												Box::new( Symbols::RightBracket(4, 5) )
												 ));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_set_one() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ 1 }")));
		parser.advance();
		let res = parser.parse_expression();

		let elements= [ Box::new( Node::Integer(2, 4, Box::new( Symbols::Integer( 2, 3, Box::new( String::from("1" )))))) ].to_vec();

		let pattern = Box::new( Node::Set(0, 5,
											Box::new( Symbols::LeftBrace(0, 1) ),
											Box::new( elements ) ,
											Box::new( Vec::new() ),
											Box::new( Symbols::RightBrace(4, 5) )
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_array_two() {
		let mut parser = Parser::new(Box::new(Scanner::new("[ 1, 2 ]")));
		parser.advance();
		let res = parser.parse_expression();

		let elements= [
			Box::new( Node::Integer(2, 3, Box::new( Symbols::Integer( 2, 3, Box::new( String::from("1" )))))),
			Box::new( Node::Integer(5, 7, Box::new( Symbols::Integer( 5, 6, Box::new( String::from("2" ))))))
		].to_vec();
		let separators = [ Box::new( Symbols::Comma(3, 4) ) ].to_vec();

		let pattern = Box::new( Node::Array(0, 8,
											Box::new( Symbols::LeftBracket(0, 1) ),
											Box::new( elements ) ,
											Box::new( separators ),
											Box::new( Symbols::RightBracket(7, 8) )
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn expression_set_two() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ 1, 2 }")));
		parser.advance();
		let res = parser.parse_expression();

		let elements= [
			Box::new( Node::Integer(2, 3, Box::new( Symbols::Integer( 2, 3, Box::new( String::from("1" )))))),
			Box::new( Node::Integer(5, 7, Box::new( Symbols::Integer( 5, 6, Box::new( String::from("2" ))))))
		].to_vec();
		let separators = [ Box::new( Symbols::Comma(3, 4) ) ].to_vec();

		let pattern = Box::new( Node::Set(0, 8,
											Box::new( Symbols::LeftBrace(0, 1) ),
											Box::new( elements ) ,
											Box::new( separators ),
											Box::new( Symbols::RightBrace(7, 8) )
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call() {
		let mut parser = Parser::new(Box::new(Scanner::new("test()")));
		parser.advance();
		let res = parser.parse_expression();

		let elements = [
			Box::new( Node::Call( 4, 6, Box::new( Symbols::LeftParen(4, 5)), None, Box::new(Symbols::RightParen(5, 6)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 6,
											 Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
							Some(Box::new(elements)),
							None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call_with_one_param() {
		let mut parser = Parser::new(Box::new(Scanner::new("test(1)")));
		parser.advance();
		let res = parser.parse_expression();

		let exp_list_element = [
			Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) )
		].to_vec();

		let exp_list = Box::new( Node::ExpressionList(5, 6, Box::new(exp_list_element), Box::new( Vec::<Box<Symbols>>::new() ) ) );

		let elements = [
			Box::new( Node::Call( 4, 7, Box::new( Symbols::LeftParen(4, 5)), Some(exp_list), Box::new(Symbols::RightParen(6, 7)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 7,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call_with_two_param() {
		let mut parser = Parser::new(Box::new(Scanner::new("test(1, 2)")));
		parser.advance();
		let res = parser.parse_expression();

		let exp_list_element = [
			Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) ),
			Box::new( Node::Integer(8, 9, Box::new(Symbols::Integer(8, 9, Box::new(String::from("2"))))) )
		].to_vec();

		let separators = [ Box::new( Symbols::Comma(6, 7) ) ].to_vec();

		let exp_list = Box::new( Node::ExpressionList(5, 9, Box::new(exp_list_element), Box::new( separators ) ) );

		let elements = [
			Box::new( Node::Call( 4, 10, Box::new( Symbols::LeftParen(4, 5)), Some(exp_list), Box::new(Symbols::RightParen(9, 10)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 10,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call_with_two_param_dot_name() {
		let mut parser = Parser::new(Box::new(Scanner::new("test(1, 2).run")));
		parser.advance();
		let res = parser.parse_expression();

		let exp_list_element = [
			Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) ),
			Box::new( Node::Integer(8, 9, Box::new(Symbols::Integer(8, 9, Box::new(String::from("2"))))) )
		].to_vec();

		let separators = [ Box::new( Symbols::Comma(6, 7) ) ].to_vec();

		let exp_list = Box::new( Node::ExpressionList(5, 9, Box::new(exp_list_element), Box::new( separators ) ) );

		let elements = [
			Box::new( Node::Call( 4, 10, Box::new( Symbols::LeftParen(4, 5)), Some(exp_list), Box::new(Symbols::RightParen(9, 10)) ) ),
			Box::new( Node::DotName(10, 14, Box::new( Symbols::Period(10, 11) ), Box::new( Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("run"))))) )) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 14,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call_with_two_param_dot_name_arrow() {
		let mut parser = Parser::new(Box::new(Scanner::new("test(1, 2).run^")));
		parser.advance();
		let res = parser.parse_expression();

		let exp_list_element = [
			Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) ),
			Box::new( Node::Integer(8, 9, Box::new(Symbols::Integer(8, 9, Box::new(String::from("2"))))) )
		].to_vec();

		let separators = [ Box::new( Symbols::Comma(6, 7) ) ].to_vec();

		let exp_list = Box::new( Node::ExpressionList(5, 9, Box::new(exp_list_element), Box::new( separators ) ) );

		let elements = [
			Box::new( Node::Call( 4, 10, Box::new( Symbols::LeftParen(4, 5)), Some(exp_list), Box::new(Symbols::RightParen(9, 10)) ) ),
			Box::new( Node::DotName(10, 14, Box::new( Symbols::Period(10, 11) ), Box::new( Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("run"))))) )) ),
			Box::new( Node::Arrow(14, 15, Box::new(Symbols::Arrow(14, 15))) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 15,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_call_with_two_param_dot_name_transpose() {
		let mut parser = Parser::new(Box::new(Scanner::new("test(1, 2).run`")));
		parser.advance();
		let res = parser.parse_expression();

		let exp_list_element = [
			Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) ),
			Box::new( Node::Integer(8, 9, Box::new(Symbols::Integer(8, 9, Box::new(String::from("2"))))) )
		].to_vec();

		let separators = [ Box::new( Symbols::Comma(6, 7) ) ].to_vec();

		let exp_list = Box::new( Node::ExpressionList(5, 9, Box::new(exp_list_element), Box::new( separators ) ) );

		let elements = [
			Box::new( Node::Call( 4, 10, Box::new( Symbols::LeftParen(4, 5)), Some(exp_list), Box::new(Symbols::RightParen(9, 10)) ) ),
			Box::new( Node::DotName(10, 14, Box::new( Symbols::Period(10, 11) ), Box::new( Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("run"))))) )) ),
			Box::new( Node::Transpose(14, 15, Box::new(Symbols::Transpose(14, 15))) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 15,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_index_one_param() {
		let mut parser = Parser::new(Box::new(Scanner::new("test[1]")));
		parser.advance();
		let res = parser.parse_expression();

		let el = [ Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))) ) ].to_vec();
		let left_list = Box::new( Node::ExpressionList(5, 6, Box::new(el), Box::new(Vec::<Box<Symbols>>::new())));

		let exp_list = Box::new( Node::IndexList(5, 6, Some(left_list), None, None, None, None )  );

		let elements = [
			Box::new( Node::Index( 4, 7, Box::new( Symbols::LeftBracket(4, 5)), Some(exp_list), Box::new(Symbols::RightBracket(6, 7)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 7,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	// COMMA ISSUES IN EXPRLIST AND INDEX !!! INSERT TEST HERE!

	#[test]
	fn designator_index_one_param_questionmark() {
		let mut parser = Parser::new(Box::new(Scanner::new("test[?]")));
		parser.advance();
		let res = parser.parse_expression();
		
		let exp_list = Box::new( Node::IndexList(5, 6, None, None, Some(Box::new(Symbols::QuestionMark(5, 6))), None, None )  );

		let elements = [
			Box::new( Node::Index( 4, 7, Box::new( Symbols::LeftBracket(4, 5)), Some(exp_list), Box::new(Symbols::RightBracket(6, 7)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 7,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_index_one_param_questionmark_and_more() {
		let mut parser = Parser::new(Box::new(Scanner::new("test[?, 1]")));
		parser.advance();
		let res = parser.parse_expression();

		let el = [ Box::new( Node::Integer(8, 9, Box::new(Symbols::Integer(8, 9, Box::new(String::from("1"))))) ) ].to_vec();
		let right_list = Box::new( Node::ExpressionList(8, 9, Box::new(el), Box::new(Vec::<Box<Symbols>>::new())));


		let exp_list = Box::new( Node::IndexList(5, 9, None, None, Some(Box::new(Symbols::QuestionMark(5, 6))), Some(Box::new(Symbols::Comma(6, 7))), Some(right_list) )  );

		let elements = [
			Box::new( Node::Index( 4, 10, Box::new( Symbols::LeftBracket(4, 5)), Some(exp_list), Box::new(Symbols::RightBracket(9, 10)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 10,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_index_one_param_questionmark_and_more_front_and_back() {
		let mut parser = Parser::new(Box::new(Scanner::new("test[2, ?, 1]")));
		parser.advance();
		let res = parser.parse_expression();

		let el = [ Box::new( Node::Integer(11, 12, Box::new(Symbols::Integer(11, 12, Box::new(String::from("1"))))) ) ].to_vec();
		let right_list = Box::new( Node::ExpressionList(11, 12, Box::new(el), Box::new(Vec::<Box<Symbols>>::new())));

		let el2 = [ Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("2"))))) ) ].to_vec();
		let left_list = Box::new( Node::ExpressionList(5, 8, Box::new(el2), Box::new(Vec::<Box<Symbols>>::new())));

		let exp_list = Box::new( Node::IndexList(5, 12, Some(left_list), Some(Box::new(Symbols::Comma(6, 7))), Some(Box::new(Symbols::QuestionMark(8, 9))), Some(Box::new(Symbols::Comma(9, 10))), Some(right_list.clone()) )  );

		let elements = [
			Box::new( Node::Index( 4, 13, Box::new( Symbols::LeftBracket(4, 5)), Some(exp_list), Box::new(Symbols::RightBracket(12, 13)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 13,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn designator_index_one_param_questionmark_and_more_front() {
		let mut parser = Parser::new(Box::new(Scanner::new("test[2, ?]")));
		parser.advance();
		let res = parser.parse_expression();

		let el2 = [ Box::new( Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("2"))))) ) ].to_vec();
		let left_list = Box::new( Node::ExpressionList(5, 8, Box::new(el2), Box::new(Vec::<Box<Symbols>>::new())));

		let exp_list = Box::new( Node::IndexList(5, 9, Some(left_list), Some(Box::new(Symbols::Comma(6, 7))), Some(Box::new(Symbols::QuestionMark(8, 9))), None, None )  );

		let elements = [
			Box::new( Node::Index( 4, 10, Box::new( Symbols::LeftBracket(4, 5)), Some(exp_list), Box::new(Symbols::RightBracket(9, 10)) ) )
		].to_vec();

		let pattern = Box::new( Node::UnaryExpression(0, 10,
													  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) ) )),
													  Some(Box::new(elements)),
													  None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn qualified_identifier_single() {
		let mut parser = Parser::new(Box::new(Scanner::new("test")));
		parser.advance();
		let res = parser.parse_qualified_identifier();

		let pattern = Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn qualified_identifier() {
		let mut parser = Parser::new(Box::new(Scanner::new("test.tull")));
		parser.advance();
		let res = parser.parse_qualified_identifier();

		//let pattern = Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) );
		let pattern = Box::new( Node::QualifiedIdentifier(0, 9,
														  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) ),
														  Box::new( Symbols::Period(4, 5) ),
														  Box::new( Node::Ident(5, 9, Box::new( Symbols::Ident(5, 9, Box::new(String::from("tull"))) )) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn ident_definition_non_export() {
		let mut parser = Parser::new(Box::new(Scanner::new("test")));
		parser.advance();
		let res = parser.parse_identifier_definition();

		let pattern = Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn ident_definition_read_write_export() {
		let mut parser = Parser::new(Box::new(Scanner::new("test*")));
		parser.advance();
		let res = parser.parse_identifier_definition();

		let pattern = Box::new( Node::IdentifierReadWrite(0, 5,
														  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) ),
														Box::new( Symbols::Times(4, 5) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn ident_definition_read_export() {
		let mut parser = Parser::new(Box::new(Scanner::new("test-")));
		parser.advance();
		let res = parser.parse_identifier_definition();

		let pattern = Box::new( Node::IdentifierRead(0, 5,
														  Box::new( Node::Ident(0, 4, Box::new( Symbols::Ident(0, 4, Box::new(String::from("test"))) )) ),
														  Box::new( Symbols::Minus(4, 5) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_if_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("IF test THEN count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
				Box::new( Node::Ident(13, 19, Box::new( Symbols::Ident(13, 18, Box::new(String::from("count"))) )) )
			].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::If(0, 22,
						Box::new( Symbols::If(0,2) ),
						Box::new( Node::Ident(3, 8, Box::new( Symbols::Ident(3, 7, Box::new(String::from("test"))) )) ),
						Box::new( Symbols::Then(8, 12) ),
						Box::new(
							Node::StatementSequence(13, 19, nodes, separators)
						),
						None,
						None,
						Box::new( Symbols::End(19, 22) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_if_else_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("IF test THEN count ELSE count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(13, 19, Box::new( Symbols::Ident(13, 18, Box::new(String::from("count"))) )) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );
		let nodes2 : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(24, 30, Box::new( Symbols::Ident(24, 29, Box::new(String::from("count"))) )) )
		].to_vec() );
		let separators2 : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let else_part= Box::new( Node::Else(19, 30, Box::new( Symbols::Else(19, 23) ),
											Box::new( Node::StatementSequence(24, 30, nodes2, separators2) )
		) );

		let pattern = Box::new( Node::If(0, 33,
										 Box::new( Symbols::If(0,2) ),
										 Box::new( Node::Ident(3, 8, Box::new( Symbols::Ident(3, 7, Box::new(String::from("test"))) )) ),
										 Box::new( Symbols::Then(8, 12) ),
										 Box::new(
											 Node::StatementSequence(13, 19, nodes, separators)
										 ),
										 None,
										 Some( else_part ),
										 Box::new( Symbols::End(30, 33) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_if_elsif_else_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("IF test THEN count ELSIF order THEN cat ELSE count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(13, 19, Box::new( Symbols::Ident(13, 18, Box::new(String::from("count"))) )) )
		].to_vec() );

		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );
		let nodes2 : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(45, 51, Box::new( Symbols::Ident(45, 50, Box::new(String::from("count"))) )) )
		].to_vec() );

		let separators2 : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let else_part= Box::new( Node::Else(40, 51, Box::new( Symbols::Else(40, 44) ),
											Box::new( Node::StatementSequence(45, 51, nodes2, separators2) )
		) );

		let separators3 : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );
		let nodes3 : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(36, 40, Box::new( Symbols::Ident(36, 39, Box::new(String::from("cat"))) )) )
		].to_vec() );



		let elsif_nodes = Box::new( [
			Box::new( Node::Elsif(19, 40,
								  Box::new(Symbols::Elsif(19, 24)),
								  Box::new( Node::Ident(25, 31, Box::new( Symbols::Ident(25, 30, Box::new(String::from("order"))) )) ),
								  Box::new(Symbols::Then(31, 35)),
								  Box::new( Node::StatementSequence(36, 40, nodes3, separators3) )) )
		].to_vec() );

		let pattern = Box::new( Node::If(0, 54,
										 Box::new( Symbols::If(0,2) ),
										 Box::new( Node::Ident(3, 8, Box::new( Symbols::Ident(3, 7, Box::new(String::from("test"))) )) ),
										 Box::new( Symbols::Then(8, 12) ),
										 Box::new(
											 Node::StatementSequence(13, 19, nodes, separators)
										 ),
										 Some(elsif_nodes),
										 Some( else_part ),
										 Box::new( Symbols::End(51, 54) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}




}