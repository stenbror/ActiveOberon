
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Parser module for syntax analyzing of source files

use console::style;
use crate::scanner::{Scanner, ScannerMethods, Symbols};
use crate::amd64_assembler::{ AssemblerAMD64, AssemblerAMD64Methods };

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
	UnaryExpression( u32, u32, Box<Node>, Option<Box<Vec<Box<Node>>>>, Option<Box<Node>> ),
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
	StatementBlock( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Node>, Box<Symbols> ),
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
	Code( u32, u32, Box<Symbols>, Box<Vec<u8>>, Box<Symbols> ),
	Ignore( u32, u32, Box<Symbols>, Box<Node> ),
	BecomesStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	ExclaimMarkStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	QuestionmarkStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	LessLessStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	GreaterGreaterStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),

	/* Block nodes */
	Module( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Node>, Option<(Box<Symbols>, Box<Node>)>, Box<Symbols>, Option<Box<Vec<Box<Node>>>>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	TemplateParameters( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	TemplateParameter( u32, u32, Box<Symbols>, Box<Node> ),
	ImportList( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	Import( u32, u32, Box<Node>, Option<(Box<Symbols>, Box<Node>)>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<(Box<Symbols>, Box<Node>)> ),
	DeclarationSequence( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	ConstDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	TypeDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	VarDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	Const( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Var( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	VarList( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	VarName( u32, u32, Box<Node>, Option<Box<Node>>, Option<(Box<Symbols>, Box<Node>)> ),
	Flags( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	Flag( u32, u32, Box<Node>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<(Box<Symbols>, Box<Node>)> ),
	Procedure( u32, u32, Box<Symbols>, Option<(Option<Box<Node>>, Option<Box<Symbols>>)>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Box<Node>, Option<Box<Node>>, Box<Symbols>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Box<Node> ),
	Operator( u32, u32, Box<Symbols>, Option<Box<Node>>, Option<Box<Symbols>>, Box<Node>, Option<Box<Symbols>>, Box<Node>, Box<Symbols>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Box<Node> ),
	FormalParameters( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Option<(Box<Symbols>, Option<Box<Node>>, Box<Node>)> ),
	ParameterDeclaration( u32, u32, Option<Box<Symbols>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Box<Node> ),
	Parameter( u32, u32, Box<Node>, Option<Box<Node>>, Option<(Box<Symbols>, Box<Node>)> ),
	Body( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Node>, Option<(Box<Symbols>, Box<Node>)> ),
	BodyCode( u32, u32, Box<Symbols>, Box<Node> ),
	TypeDeclarationElement( u32, u32, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	ArrayType( u32, u32, Box<Symbols>, Option<(Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>)>,  Box<Symbols>, Box<Node> ),
	MathArrayType( u32, u32, Box<Symbols>, Option<(Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>)>,  Box<Symbols>, Box<Node> ),
	MathArraySize( u32, u32, Option<Box<Node>>, Option<Box<Symbols>> ),
	RecordType( u32, u32, Box<Symbols>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<(Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>)>, Option<(Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>)>, Box<Symbols> ),
	PointerType( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Symbols>, Box<Node> ),
	ProcedureType( u32, u32, Box<Symbols>, Option<Box<Node>>, Option<Box<Node>> ),
	ObjectTypeEmpty( u32, u32, Box<Symbols> ),
	ObjectType( u32, u32, Box<Symbols>, Option<Box<Node>>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Option<Box<Node>> ),
	EnumerationType( u32, u32, Box<Symbols>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	EnumElement( u32, u32, Box<Node>, Option<(Box<Symbols>, Box<Node>)> ),
	CellType( u32, u32, Box<Symbols>, Option<Box<Node>>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<Box<Symbols>>, Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Option<Box<Node>> ),
	PortList( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	PortDeclaration( u32, u32, Box<Vec<Box<(Box<Node>, Option<Box<Node>>)>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Box<Node> ),
	PortType( u32, u32, Box<Symbols>, Box<Symbols>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)> ),
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
	fn parse_import(&mut self) -> Result<Box<Node>, Box<String>>;
	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
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
					_ => left.ok_or( Box::new(format!("Missing expression at position: '{}'", self.lexer.get_start_position())) )
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
							_ => return Err(Box::new(format!("Expecting 'of' in 'alias' expression at position: '{}'", self.lexer.get_start_position())))
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
							_ => return Err(Box::new(format!("Expecting '(' in 'new' expression at position: '{}'", self.lexer.get_start_position())))
						}

						let right = self.parse_expression_list()?;

						let symbol3 = self.symbol.clone()?;
						match symbol3 {
							Symbols::RightParen( _ , _ ) => {
								self.advance()
							},
							_ => return Err(Box::new(format!("Expecting ')' in 'new' expression at position: '{}'", self.lexer.get_start_position())))
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
							_ => return Err(Box::new(format!("Expecting ')' in parenthesized expression at position: '{}'", self.lexer.get_start_position())))
						}

						Ok( Box::new(Node::ParenthesisExpression(start_pos, self.lexer.get_start_position(), Box::new(x), right, Box::new(symbol2))))
					},
					Symbols::LeftBracket( _ , _ ) => {
						self.parse_array()
					},
					Symbols::LeftBrace( _ , _ ) => {
						self.parse_set()
					}
					_ => Err(Box::new(format!("Unexpected or missing literal at position: '{}'", self.lexer.get_start_position())))
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
						_ => return Err(Box::new(format!("Missing ')' in call at position: '{}'", self.lexer.get_start_position())))
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
						_ => return Err(Box::new(format!("Expecting name literal after '.' at position: '{}'", self.lexer.get_start_position())))
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
						_ => return Err(Box::new(format!("Missing ']' in index at position: '{}'", self.lexer.get_start_position())))
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
							_ => return Err(Box::new(format!("Expecting '?' in index expression at position: '{}'", self.lexer.get_start_position())))
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
			_ => return Err(Box::new(format!("Expecting '[' in array expression at position: '{}'", self.lexer.get_start_position())))
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
			_ => return Err(Box::new(format!("Expecting ']' in array expression at position: '{}'", self.lexer.get_start_position())))
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
			_ => return Err(Box::new(format!("Expecting start of set expression at position: '{}'", self.lexer.get_start_position())))
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
			_ => return Err(Box::new(format!("Expecting end of set expression at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'THEN' in if statement at position: '{}'", self.lexer.get_start_position())))
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
								_ => return Err(Box::new(format!("Expecting 'THEN' in elsif statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'END' in if statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting Identifier in for statement at position: '{}'", self.lexer.get_start_position())))
				};

				match self.symbol.clone()? {
					Symbols::Colon( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ':' in with statement at position: '{}'", self.lexer.get_start_position())))
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
								Symbols::Bar( _ , _ ) => return Err(Box::new(format!("No '|' at first element in with statement at position: '{}'", self.lexer.get_start_position()))),
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
						_ => return Err(Box::new(format!("Expecting 'DO' in with statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'END' in with statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'OF' in case statement at position: '{}'", self.lexer.get_start_position())))
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
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in case statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'DO' in while statement at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in while statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'UNTIL' in repeat statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting Identifier in for statement at position: '{}'", self.lexer.get_start_position())))
				};

				match self.symbol.clone()? {
					Symbols::Becomes( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ':=' in for statement at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol3 = self.symbol.clone()?;
				self.advance();

				let left = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::To( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'TO' in for statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'DO' in for statement at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol6 = self.symbol.clone()?;
				self.advance();

				let stmt = self.parse_statement_sequence()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in for statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting 'END' in loop statement at position: '{}'", self.lexer.get_start_position())))
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

				let code_bytes = *self.lexer.slice_assembler_code();

				// This need to be controlled by argument, which assembler to use, later!
				let mut assembler = AssemblerAMD64::new(code_bytes, self.lexer.get_start_position());
				let code = assembler.assemble()?;

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in code statement at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol2 = self.symbol.clone()?;
				self.advance();
				Ok( Box::new(Node::Code(start_pos, self.lexer.get_start_position(), Box::new(symbol1), code, Box::new(symbol2))) )
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
					Symbols::ExclaimMark( _ , _ ) => {
						let symbol1 = Box::new( self.symbol.clone()? );
						self.advance();
						let right2 = self.parse_expression()?;
						Ok(Box::new(Node::ExclaimMarkStatement(start_pos, self.lexer.get_start_position(), left, symbol1, right2)))
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

		match self.symbol.clone()? {
			Symbols::Bar( _ , _ ) => {
				symbol1 = Some( Box::new( self.symbol.clone()? ) );
				self.advance();
			},
			_ => {
				match bar_optional {
					true => (),
					_ => return Err(Box::new(format!("Expecting '|' in case statement at position: '{}'", self.lexer.get_start_position())))
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
			_ => return Err(Box::new(format!("Expecting ':' in case statement at position: '{}'", self.lexer.get_start_position())))
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
					_ => Err(Box::new(format!("Expecting 'END' in statement block at position: '{}'", self.lexer.get_start_position())))
				}
			},
			_ => {
				Err(Box::new(format!("Expecting 'BEGIN' in statement block at position: '{}'", self.lexer.get_start_position())))
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
		let start_pos = self.lexer.get_start_position();
		let mut module_name_start = String::new();
		let mut module_name_end = String::new();

		self.advance(); /* Get the first symbol from source code before starting parsing */

		return match self.symbol.clone()? {
			Symbols::Module( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let template : Option<Box<Node>> = match self.symbol.clone()? {
					Symbols::LeftParen( _ , _ ) => Some( self.parse_template_parameters()? ),
					_ => None
				};

				let symbol2 = match self.symbol.clone()? {
					Symbols::Ident( s , _ , t ) => {
						module_name_start = *t;
						println!("  Compiling module: '{}'", style(&module_name_start).green());
						let symbol18 = self.symbol.clone()?;
						self.advance();
						Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol18)) )
					},
					_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'MODULE' at position: '{}'", self.lexer.get_start_position())))
				};

				let in_part = match self.symbol.clone()? {
					Symbols::In( _ , _ ) => {
						let symbol16 = self.symbol.clone()?;
						self.advance();

						match self.symbol.clone()? {
							Symbols::Ident( _ , _ , _ ) => {
								let start_pos2 = self.lexer.get_start_position();
								let symbol15 = self.symbol.clone()?;
								self.advance();

								Some( (Box::new(symbol16), Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol15)))) )
							},
							_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'IN' at position: '{}'", self.lexer.get_start_position())))
						}
					},
					_ => None
				};

				let symbol3 = match self.symbol.clone()? {
					Symbols::SemiColon( _ , _ ) => {
						let symbol17 = self.symbol.clone()?;
						self.advance();
						Box::new( symbol17 )
					},
					_ => return Err(Box::new(format!("Expecting ';' of module after 'MODULE' ident at position: '{}'", self.lexer.get_start_position())))
				};

				let imp = match self.symbol.clone()? {
					Symbols::Import( _ , _ ) => {
						let mut nodes = Box::new(Vec::<Box<Node>>::new());
						loop {
							match self.symbol.clone()? {
								Symbols::Import( _ , _ ) => nodes.push( self.parse_import_list()? ),
								_ => break
							}
						}
						Some( nodes )
					},
					_ => None
				};

				let decl = match self.symbol.clone()? {
					Symbols::Type( _ , _ ) |
					Symbols::Var( _ , _ ) |
					Symbols::Const( _ , _ ) |
					Symbols::Procedure( _ , _ ) |
					Symbols::Operator( _ , _ ) |
					Symbols::SemiColon( _ , _ ) => Some( self.parse_declaration_sequence()? ),
					_ => None
				};

				let body = match self.symbol.clone()? {
					Symbols::Begin( _ , _ ) | Symbols::Code( _ , _ ) => Some( self.parse_body()? ),
					_ => None
				};

				/* Parsing 'END' ident '.' and ignore everything after that! */

				let symbol11 = match self.symbol.clone()? {
					Symbols::End( _ , _) => {
						let symbol19 = self.symbol.clone()?;
						self.advance();
						Box::new( symbol19 )
					},
					_ => return Err(Box::new(format!("Expecting 'END' in module at position: '{}'", self.lexer.get_start_position())))
				};

				let symbol12 = match self.symbol.clone()? {
					Symbols::Ident( s , _ , t ) => {
						module_name_end = *t;
						let symbol21 = self.symbol.clone()?;
						self.advance();
						Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new( symbol21 )) )
 					},
					_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'END' at position: '{}'", self.lexer.get_start_position())))
				};

				let period = match self.symbol.clone()? {
					Symbols::Period( _ , _ ) => {
						let symbol20 = self.symbol.clone()?;
						self.advance();
						Box::new( symbol20 )
					},
					_ => return Err(Box::new(format!("Expecting '.' at end of module at position: '{}'", self.lexer.get_start_position())))
				};

				if module_name_start != module_name_end {
					return Err(Box::new(format!("Expecting 'MODULE' name '{}' to be equal to 'END' name '{}' in module declaration at position: '{}'", module_name_start, module_name_end, self.lexer.get_start_position())))
				}

				Ok( Box::new( Node::Module(start_pos, self.lexer.get_start_position(), Box::new(symbol1), template, symbol2, in_part, symbol3, imp, decl, body, symbol11, symbol12, period) ) )
			},
			_ => Err(Box::new(format!("Expecting 'MODULE' in module declaration at position: '{}'", self.lexer.get_start_position())))
		}
	}

	fn parse_template_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		let symbol1 = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol11)
			},
			_ => return Err(Box::new(format!("Expecting '(' in template list declaration at position: '{}'", self.lexer.get_start_position())))
		};

		nodes.push( self.parse_template_parameter()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new( self.symbol.clone()?) );
					self.advance();
					nodes.push( self.parse_template_parameter()? );
				},
				_ => break
			}
		}

		let symbol2 = match self.symbol.clone()? {
			Symbols::RightParen( _ , _ ) => {
				let symbol12 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol12)
			},
			_ => return Err(Box::new(format!("Expecting ')' in template list declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::TemplateParameters(start_pos, self.lexer.get_start_position(), symbol1, nodes, separators, symbol2)) )
	}

	fn parse_template_parameter(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::Const( _ , _ ) | Symbols::Type( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol11)
			},
			_ => return Err(Box::new(format!("Expecting 'CONST' or 'TYPE' in template declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let idx = match self.symbol.clone()? {
			Symbols::Ident( s , _ , _ ) => {
				let symbol12 = self.symbol.clone()?;
				self.advance();
				Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol12)) )
			},
			_ => return Err(Box::new(format!("Expecting 'ident' literal in template declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::TemplateParameter(start_pos, self.lexer.get_start_position(), symbol1, idx)) )
	}

	fn parse_import_list(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::Import( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol11)
			},
			_ => return Err(Box::new(format!("Expecting 'IMPORT' in import declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		nodes.push( self.parse_import()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
					nodes.push( self.parse_import()? );
				},
				_ => break
			}
		}

		let symbol2 = match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => {
				let symbol12 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol12)
			},
			_ => return Err(Box::new(format!("Expecting ';' in import declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::ImportList(start_pos, self.lexer.get_start_position(), symbol1, nodes, separators, symbol2)) )
	}

	fn parse_import(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::Ident( s , _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol11)) )
			},
			_ => return Err(Box::new(format!("Expecting 'ident' literal in import declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let left = match self.symbol.clone()? {
			Symbols::Becomes( _ , _ ) => {
				let symbol21 = self.symbol.clone()?;
				self.advance();

				match self.symbol.clone()? {
					Symbols::Ident( s, _ , _ ) => {
						let symbol22 = self.symbol.clone()?;
						self.advance();
						let node = Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol22)) );
						Some( (Box::new(symbol21), node) )
					},
					_ => return Err(Box::new(format!("Expecting 'ident' literal in import declaration after ':=' at position: '{}'", self.lexer.get_start_position())))
				}
			},
			_ => None
		};

		let right = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol31 = self.symbol.clone()?;
				self.advance();
				let right2 = self.parse_expression_list()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => {
						let symbol32 = self.symbol.clone()?;
						self.advance();
						Some( ( Box::new(symbol31), right2, Box::new(symbol32) ) )
					},
					_ => return Err(Box::new(format!("Expecting ')' literal in import declaration at position: '{}'", self.lexer.get_start_position())))
				}
			},
			_ => None
		};

		let next = match self.symbol.clone()? {
			Symbols::In( _ , _ ) => {
				let symbol41 = self.symbol.clone()?;
				self.advance();

				match self.symbol.clone()? {
					Symbols::Ident( s, _ , _ ) => {
						let symbol42 = self.symbol.clone()?;
						self.advance();
						let node = Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol42)) );
						Some( (Box::new(symbol41), node) )
					},
					_ => return Err(Box::new(format!("Expecting 'ident' literal in import declaration after 'IN' at position: '{}'", self.lexer.get_start_position())))
				}
			},
			_ => None
		};

		Ok( Box::new(Node::Import(start_pos, self.lexer.get_start_position(), symbol1, left, right, next)) )
	}

	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut const_declarations = Box::new(Vec::<Box<Node>>::new());
		let mut type_declarations = Box::new(Vec::<Box<Node>>::new());
		let mut var_declarations = Box::new(Vec::<Box<Node>>::new());
		let mut procedure_declarations = Box::new(Vec::<Box<Node>>::new());
		let mut operator_declarations = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		loop {
			match self.symbol.clone()? {
				Symbols::Const(_, _) => {
					let start_pos2 = self.lexer.get_start_position();
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut const_declaration_local = Box::new(Vec::<Box<Node>>::new());
					match self.symbol.clone()? {
						Symbols::Ident( _ , _ , _ ) => {
							const_declaration_local.push( self.parse_constant_declaration()? );
							loop {
								match self.symbol.clone()? {
									Symbols::SemiColon( _ , _ ) => {
										separators.push( Box::new(self.symbol.clone()?) );
										self.advance();
										const_declaration_local.push( self.parse_constant_declaration()? )
									},
									_ => break
								}
							}
						},
						_ => ()
					}
					const_declarations.push( Box::new(Node::ConstDeclaration(start_pos2, self.lexer.get_start_position(), Box::new(symbol), const_declaration_local) ) )
				},
				Symbols::Type(_, _) => {
					let start_pos2 = self.lexer.get_start_position();
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut type_declaration_local = Box::new(Vec::<Box<Node>>::new());

					match self.symbol.clone()? {
						Symbols::Ident( _ , _ , _ ) => {
							let node_first = self.parse_type_declaration()?;
							type_declaration_local.push( node_first );

							loop {
								match self.symbol.clone()? {
									Symbols::Ident( _ , _ , _ ) => {
										let node_next = self.parse_type_declaration()?;
										type_declaration_local.push( node_next );
									},
									_ => break
								}
							}
						},
						_ => ()
					}

					type_declarations.push( Box::new(Node::TypeDeclaration(start_pos2, self.lexer.get_start_position(), Box::new(symbol), type_declaration_local) ) )
				},
				Symbols::Var(_, _) => {
					let start_pos2 = self.lexer.get_start_position();
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut var_declaration_local = Box::new(Vec::<Box<Node>>::new());
					match self.symbol.clone()? {
						Symbols::Ident( _ , _ , _ ) => {
							var_declaration_local.push( self.parse_variable_declaration()? );
							loop {
								match self.symbol.clone()? {
									Symbols::SemiColon( _ , _ ) => {
										separators.push( Box::new(self.symbol.clone()?) );
										self.advance();
										var_declaration_local.push( self.parse_variable_declaration()? )
									},
									_ => break
								}
							}
						},
						_ => ()
					}
					var_declarations.push( Box::new(Node::VarDeclaration(start_pos2, self.lexer.get_start_position(), Box::new(symbol), var_declaration_local) ) )
				},
				Symbols::Procedure(_, _) => {
					procedure_declarations.push( self.parse_procedure_declaration()? )
				},
				Symbols::Operator(_, _) => {
					operator_declarations.push( self.parse_operator_declaration()? )
				},
				Symbols::SemiColon(_, _) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
				}
				_ => break
			}
		}

		Ok( Box::new(Node::DeclarationSequence(start_pos, self.lexer.get_start_position(), const_declarations, type_declarations, var_declarations, procedure_declarations, operator_declarations, separators)) )
	}

	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let first = match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => self.parse_identifier_definition()?,
			_ => return Err(Box::new(format!("Expecting 'ident' literal in const declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let symbol = match self.symbol.clone()? {
			Symbols::Equal( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol11)
			},
			_ => return Err(Box::new(format!("Expecting '=' literal in const declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let second = self.parse_expression()?;

		Ok( Box::new(Node::Const(start_pos, self.lexer.get_start_position(), first, symbol, second)) )
	}

	fn parse_variable_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let first = match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => self.parse_variable_name_list()?,
			_ => return Err(Box::new(format!("Expecting 'indent' literal in var declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let symbol = match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Box::new(symbol11)
			},
			_ => return Err(Box::new(format!("Expecting ':' literal in var declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let second = self.parse_type()?;

		Ok( Box::new(Node::Var(start_pos, self.lexer.get_start_position(), first, symbol, second)) )
	}

	fn parse_variable_name_list(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut nodes = Box::new( Vec::<Box<Node>>::new() );
		let mut separators = Box::new( Vec::<Box<Symbols>>::new() );

		match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'indent' literal in var declaration list at position: '{}'", self.lexer.get_start_position())))
		};

		nodes.push( self.parse_variable_name()? );

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();
					nodes.push( self.parse_variable_name()? );
				},
				_ => break
			}
		}

		Ok( Box::new(Node::VarList(start_pos, self.lexer.get_start_position(), nodes, separators)) )
	}

	fn parse_variable_name(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let first = match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => self.parse_identifier_definition()?,
			_ => return Err(Box::new(format!("Expecting 'indent' literal in var declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		return match self.symbol.clone()? {
			Symbols::Becomes( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let right = self.parse_expression()?;
				Ok( Box::new(Node::VarName(start_pos, self.lexer.get_start_position(), first, flags, Some( ( Box::new(symbol1), right ) ))) )
			},
			Symbols::Extern( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let txt = match self.symbol.clone()? {
					Symbols::String( s , _ , _ ) => {
						let symbol2 = self.symbol.clone()?;
						self.advance();
						Box::new(Node::String(s, self.lexer.get_start_position(), Box::new(symbol2)))
					},
					_ => return Err(Box::new(format!("Expecting 'string' literal in var declaration at position: '{}'", self.lexer.get_start_position())))
				};

				Ok( Box::new(Node::VarName(start_pos, self.lexer.get_start_position(), first, flags, Some( (Box::new(symbol1), txt) ))) )
			}
			_ => {
				Ok( Box::new(Node::VarName(start_pos, self.lexer.get_start_position(), first, flags, None)) )
			}
		}
	}

	fn parse_flags(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut nodes = Box::new( Vec::<Box<Node>>::new() );
		let mut separators = Box::new( Vec::<Box<Symbols>>::new() );

		return match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				match self.symbol.clone()? {
					Symbols::RightBrace( _ , _ ) => (),
					_ => {
						nodes.push( self.parse_flag()? );
						loop {
							match self.symbol.clone()? {
								Symbols::Comma( _ , _ ) => {
									separators.push( Box::new(self.symbol.clone()?) );
									self.advance();
									nodes.push( self.parse_flag()? )
								},
								_ => break
							}
						}
					}
				}

				let symbol2 = match self.symbol.clone()? {
					Symbols::RightBrace( _ ,_ ) => {
						let symbol51 = self.symbol.clone()?;
						self.advance();
						Box::new(symbol51)
					},
					_ => return Err(Box::new(format!("Expecting end of flag list declaration at position: '{}'", self.lexer.get_start_position())))
				};

				Ok( Box::new(Node::Flags(start_pos, self.lexer.get_start_position(), Box::new(symbol1), nodes, separators, symbol2)) )
			},
			_ => Err(Box::new(format!("Expecting start of flag list declaration at position: '{}'", self.lexer.get_start_position())))
		}
	}

	fn parse_flag(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		return match self.symbol.clone()? {
			Symbols::Ident( s, _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();
				let flag = Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol1)) );

				match self.symbol.clone()? {
					Symbols::LeftParen( _ , _ ) => {
						let symbol21 = self.symbol.clone()?;
						self.advance();

						let right3 = self.parse_expression()?;

						match self.symbol.clone()? {
							Symbols::RightParen( _ , _ ) => (),
							_ => return Err(Box::new(format!("Expecting ')' in flag declaration at position: '{}'", self.lexer.get_start_position())))
						};
						let symbol22 = self.symbol.clone()?;
						self.advance();

						Ok( Box::new(Node::Flag(start_pos, self.lexer.get_start_position(), flag, Some( ( Box::new(symbol21), right3, Box::new(symbol22)) ) , None) ) )
					},
					Symbols::Equal( _ , _ ) => {
						let symbol31 = self.symbol.clone()?;
						self.advance();
						let right2 = self.parse_expression()?;
						Ok( Box::new(Node::Flag(start_pos, self.lexer.get_start_position(), flag, None, Some( (Box::new(symbol31), right2) ))) )
					},
					_ => {
						Ok( Box::new(Node::Flag(start_pos, self.lexer.get_start_position(), flag, None, None)) )
					}
				}
			},
			_ => Err(Box::new(format!("Expecting 'ident' in flag declaration at position: '{}'", self.lexer.get_start_position())))
		}
	}

	fn parse_procedure_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Procedure( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'PROCEDURE' in procedure declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let first = match self.symbol.clone()? {
			Symbols::Arrow( _ , _ ) | Symbols::And( _ , _ ) | Symbols::Not( _ , _ ) | Symbols::Minus( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Some( ( None, Some( Box::new(symbol11)) ) )
			},
			Symbols::LeftBrace( _ , _ ) => {
				let flags = Some( self.parse_flags()? );

				let symbol12 = match self.symbol.clone()? {
					Symbols::Minus( _ , _ ) => {
						let symbol22 = self.symbol.clone()?;
						self.advance();
						Some( Box::new(symbol22) )
					},
					_ => None
				};

				Some( ( flags, symbol12) )
			},
			_ => None
		};

		let second = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol51 = self.symbol.clone()?;
				self.advance();

				let right2 = self.parse_parameter_declaration()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in procedure declaration at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol52 = self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol51), right2, Box::new(symbol52)) )
			},
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting name of procedure in declaration at position: '{}'", self.lexer.get_start_position())))
		};
		let third = self.parse_identifier_definition()?;

		let forth = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) =>  Some( self.parse_formal_parameters()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ';' in procedure declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2 = self.symbol.clone()?;
		self.advance();

		let decl = match self.symbol.clone()? {
			Symbols::Const( _ , _ ) |
			Symbols::Var( _ , _ ) |
			Symbols::Type( _ , _ ) |
			Symbols::Procedure( _ , _ ) |
			Symbols::Operator( _ , _ ) |
			Symbols::SemiColon( _ , _ ) => Some( self.parse_declaration_sequence()? ),
			_ => None
		};

		let body = match self.symbol.clone()? {
			Symbols::Begin( _ , _ ) | Symbols::Code( _ , _  ) => Some( self.parse_body()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::End( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'END' in procedure declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol3 = self.symbol.clone()?;
		self.advance();

		let fifth = match self.symbol.clone()? {
			Symbols::Ident( s , _ , _ ) => {
				let symbol40 = self.symbol.clone()?;
				self.advance();
				Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol40)) )
			},
			_ => return Err(Box::new(format!("Expecting 'name' literal in procedure declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::Procedure(start_pos, self.lexer.get_start_position(), Box::new(symbol1), first, second, third, forth, Box::new(symbol2), decl, body, Box::new(symbol3), fifth)) )
	}

	fn parse_operator_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Operator( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'OPERATOR' in operator declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		let symbol2 = match self.symbol.clone()? {
			Symbols::Minus( _ , _ ) => {
				let symbol21 = self.symbol.clone()?;
				self.advance();
				Some( Box::new(symbol21) )
			},
			_ => None
		};

		let first = match self.symbol.clone()? {
			Symbols::String( s , _ , _ ) => {
				let symbol30 = self.symbol.clone()?;
				self.advance();
				Box::new( Node::String(s, self.lexer.get_start_position(), Box::new(symbol30)) )
			},
			_ => return Err(Box::new(format!("Expecting 'string' literal in operator declaration at position: '{}'", self.lexer.get_start_position())))
		};

		let symbol3 = match self.symbol.clone()? {
			Symbols::Times( _ , _ ) | Symbols::Minus( _ , _ ) => {
				let symbol21 = self.symbol.clone()?;
				self.advance();
				Some( Box::new(symbol21) )
			},
			_ => None
		};

		let second = self.parse_formal_parameters()?;

		match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ';' in operator declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol4 = self.symbol.clone()?;
		self.advance();

		let decl = match self.symbol.clone()? {
			Symbols::Const( _ , _ ) |
			Symbols::Var( _ , _ ) |
			Symbols::Type( _ , _ ) |
			Symbols::Procedure( _ , _ ) |
			Symbols::Operator( _ , _ ) |
			Symbols::SemiColon( _ , _ ) => Some( self.parse_declaration_sequence()? ),
			_ => None
		};

		let body = match self.symbol.clone()? {
			Symbols::Begin( _ , _ ) | Symbols::Code( _ , _  ) => Some( self.parse_body()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::End( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'END' in operator declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol5 = self.symbol.clone()?;
		self.advance();

		let third = match self.symbol.clone()? {
			Symbols::String( s , _ , _ ) => {
				let symbol40 = self.symbol.clone()?;
				self.advance();
				Box::new( Node::String(s, self.lexer.get_start_position(), Box::new(symbol40)) )
			},
			_ => return Err(Box::new(format!("Expecting 'string' literal in operator declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::Operator(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, symbol2, first, symbol3, second, Box::new(symbol4), decl, body, Box::new(symbol5), third)) )
	}

	fn parse_formal_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting '(' in formal parameters declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		match self.symbol.clone()? {
			Symbols::RightParen( _ , _ ) => (),
			_ => {
				nodes.push( self.parse_parameter_declaration()? );
				loop {
					match self.symbol.clone()? {
						Symbols::SemiColon( _ , _ ) => {
							separators.push( Box::new(self.symbol.clone()?) );
							self.advance();
							nodes.push( self.parse_parameter_declaration()? );
						},
						_ => break
					}
				}
			}
		}

		match self.symbol.clone()? {
			Symbols::RightParen( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ')' in formal parameters declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2 = self.symbol.clone()?;
		self.advance();

		let element = match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => {
				let symbol3 = self.symbol.clone()?;
				self.advance();

				let flags = match self.symbol.clone()? {
					Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
					_ => None
				};

				let right = self.parse_type()?;

				Some( (Box::new(symbol3), flags, right))
			},
			_ => None
		};

		Ok( Box::new(Node::FormalParameters(start_pos, self.lexer.get_start_position(), Box::new(symbol1), nodes, separators, Box::new(symbol2), element)) )
	}

	fn parse_parameter_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();
		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		let symbol1 = match self.symbol.clone()? {
			Symbols::Var( _ , _ ) | Symbols::Const( _ , _ ) => {
				let symbol11 = self.symbol.clone()?;
				self.advance();
				Some( Box::new(symbol11) )
			},
			_ => None
		};

		loop {
			match self.symbol.clone()? {
				Symbols::Ident( s, _ , _ ) => {
					let tmp = self.symbol.clone()?;
					self.advance();
					let node = Box::new(Node::Ident(s, self.lexer.get_start_position(), Box::new(tmp)));

					let flags = match self.symbol.clone()? {
						Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
						_ => None
					};

					let right = match self.symbol.clone()? {
						Symbols::Equal( _ , _ ) => {
							let symbol21 = self.symbol.clone()?;
							self.advance();
							let right_local = self.parse_expression()?;
							Some( (Box::new(symbol21), right_local) )
						},
						_ => None
					};
					nodes.push( Box::new(Node::Parameter(s, self.lexer.get_start_position(), node, flags, right )) )
				},
				_ => return Err(Box::new(format!("Expecting 'indent' literal in parameter declaration at position: '{}'", self.lexer.get_start_position())))
			}

			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance()
				},
				_ => break
			}
		}

		let ( element, node ) = match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => {
				let symbol3 = self.symbol.clone()?;
				self.advance();

				let right = self.parse_type()?;

				(Box::new(symbol3), right)
			},
			_ => return Err(Box::new(format!("Expecting ':' in type declaration at position: '{}'", self.lexer.get_start_position())))
		};

		Ok( Box::new(Node::ParameterDeclaration(start_pos, self.lexer.get_start_position(), symbol1, nodes, separators, element, node)) )
	}

	fn parse_body(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		return match self.symbol.clone()? {
			Symbols::Code( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				// Handle inline assembler later here!

				Ok( Box::new( Node::BodyCode(start_pos, self.lexer.get_start_position(), Box::new(symbol1), Box::new(Node::Empty)) ) )
			},
			Symbols::Begin( _ , _ ) => {
				let symbol1 = self.symbol.clone()?;
				self.advance();

				let flags = match self.symbol.clone()? {
					Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
					_ => None
				};

				let right = self.parse_statement_sequence()?;

				let fin = match self.symbol.clone()? {
					Symbols::Finally( _ , _ ) => {
						let symbol2 = self.symbol.clone()?;
						self.advance();

						let right_local = self.parse_statement_sequence()?;

						Some( ( Box::new(symbol2), right_local ) )
					},
					_ => None
				};

				Ok( Box::new( Node::Body(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, right, fin) ) )
			},
			_ => Err(Box::new(format!("Expecting ';' in type declaration at position: '{}'", self.lexer.get_start_position())))
		}
	}

	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let left = self.parse_identifier_definition()?;

		match self.symbol.clone()? {
			Symbols::Equal( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting '=' in type declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let right = self.parse_type()?;

		match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ';' in type declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2 = self.symbol.clone()?;
		self.advance();


		Ok( Box::new(Node::TypeDeclarationElement(start_pos, self.lexer.get_start_position(), left, Box::new(symbol1), right, Box::new(symbol2))) )
	}

	fn parse_type(&mut self) -> Result<Box<Node>, Box<String>> {
		match self.symbol.clone()? {
			Symbols::Array( _ , _ ) => self.parse_array_type(),
			Symbols::Record( _ , _ ) => self.parse_record_type(),
			Symbols::Pointer( _ , _ ) => self.parse_pointer_type(),
			Symbols::Object( _ , _ ) => self.parse_object_type(),
			Symbols::Procedure( _ , _ ) => self.parse_procedure_type(),
			Symbols::Enum( _ , _ ) => self.parse_enumeration_type(),
			Symbols::Ident( _ , _ , _ ) => self.parse_qualified_identifier(),
			Symbols::Cell( _ , _ ) |
			Symbols::Cellnet( _ , _ ) => self.parse_cell_type(),
			Symbols::Port( _ , _ ) => self.parse_port_type(),
			_ => Err(Box::new(format!("Expecting type at position: '{}'", self.lexer.get_start_position())))
		}
	}

	fn parse_array_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Array( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'ARRAY' in array type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());
		let mut is_math = false;

		match self.symbol.clone()? {
			Symbols::Of( _ , _ ) => (),
			Symbols::Times( s , _ ) |
			Symbols::QuestionMark( s , _ ) => {
				is_math = true;
				let symbol100 = self.symbol.clone()?;
				self.advance();
				nodes.push( Box::new(Node::MathArraySize(s, self.lexer.get_start_position(), None, Some(Box::new(symbol100)))) );
			}
			_ => {
				nodes.push( self.parse_expression()? );
			}
		}

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance();

					let start_pos_2 = self.lexer.get_start_position();

					match self.symbol.clone()? {
						Symbols::Times( _ , _ ) |
						Symbols::QuestionMark( _ , _ ) => {
							is_math = true;
							nodes.push( Box::new(Node::MathArraySize(start_pos_2, self.lexer.get_start_position(), None, Some(Box::new(self.symbol.clone()?)))) );
							self.advance();
						}
						_ => {
							match is_math {
								true => {
									let right = self.parse_expression()?;
									nodes.push( Box::new(Node::MathArraySize(start_pos_2, self.lexer.get_start_position(), Some(right), None)) )
								},
								_ => nodes.push( self.parse_expression()? )
							}
						}
					}
				},
				_ => break
			}
		}

		match self.symbol.clone()? {
			Symbols::Of( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'OF' in array type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2= self.symbol.clone()?;
		self.advance();

		let right = self.parse_type()?;

		let elements = match ( nodes.len(), separators.len() ) {
			( 0, 0 ) => None,
			_ => Some( ( nodes, separators ) )
		};

		match is_math {
			true => Ok( Box::new(Node::MathArrayType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), elements, Box::new(symbol2), right)) ),
			_ => Ok( Box::new(Node::ArrayType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), elements, Box::new(symbol2), right)) )
		}
	}

	fn parse_record_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Record( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'RECORD' in record type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		let base = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol10= self.symbol.clone()?;
				self.advance();

				let right = self.parse_qualified_identifier()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in record type at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol11= self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol10), right, Box::new(symbol11)) )
			},
			_ => None
		};

		let mut nodes_var = Box::new(Vec::<Box<Node>>::new());
		let mut separators_var = Box::new(Vec::<Box<Symbols>>::new());

		match self.symbol.clone()? {
			Symbols::Procedure( _ , _ ) | Symbols::Operator( _ , _ ) | Symbols::End( _ , _ ) => (),
			_ => {
				nodes_var.push( self.parse_variable_declaration()? );
				loop {
					match self.symbol.clone()? {
						Symbols::SemiColon( _ , _ ) => {
							separators_var.push( Box::new(self.symbol.clone()?) );
							self.advance();
							nodes_var.push( self.parse_variable_declaration()? )
						},
						_ => break
					}
				}
			}
		}

		let el_var = match (nodes_var.len(), separators_var.len()) {
			( 0, 0 ) => None,
			_ => Some( ( nodes_var, separators_var ) )
		};

		let mut nodes_op = Box::new(Vec::<Box<Node>>::new());
		let mut separators_op = Box::new(Vec::<Box<Symbols>>::new());

		loop {
			match self.symbol.clone()? {
				Symbols::Procedure( _ , _ ) => {
					nodes_op.push( self.parse_procedure_declaration()? );
					match self.symbol.clone()? {
						Symbols::SemiColon( _ , _ ) => {
							separators_op.push( Box::new(self.symbol.clone()?) );
							self.advance()
						},
						_ => ()
					}
				},
				Symbols::Operator( _ , _ ) => {
					nodes_op.push( self.parse_operator_declaration()? );
					match self.symbol.clone()? {
						Symbols::SemiColon( _ , _ ) => {
							separators_op.push( Box::new(self.symbol.clone()?) );
							self.advance()
						},
						_ => ()
					}
				},
				_ => break
			}
		}

		let el_op = match (nodes_op.len(), separators_op.len()) {
			( 0, 0 ) => None,
			_ => Some( ( nodes_op, separators_op ) )
		};

		match self.symbol.clone()? {
			Symbols::End( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'END' in record type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2= self.symbol.clone()?;
		self.advance();

		Ok( Box::new(Node::RecordType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), base, el_var, el_op, Box::new(symbol2))) )
	}

	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Pointer( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'POINTER' in pointer type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::To( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'TO' in pointer type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2= self.symbol.clone()?;
		self.advance();

		let right = self.parse_type()?;

		Ok( Box::new(Node::PointerType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, Box::new(symbol2), right)) )
	}

	fn parse_procedure_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Procedure( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'PROCEDURE' in procedure type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		let para = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => Some( self.parse_formal_parameters()? ),
			__ => None
		};

		Ok( Box::new(Node::ProcedureType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, para)) )
	}

	fn parse_object_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Object( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'OBJECT' in object type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) |
			Symbols::LeftBrace( _ , _ ) |
			Symbols::Begin( _ , _ ) |
			Symbols::End( _ , _ ) |
			Symbols::SemiColon( _ , _ ) |
			Symbols::Var( _ , _ ) |
			Symbols::Const( _ , _ ) |
			Symbols::Type( _ , _ ) |
			Symbols::Procedure( _ , _ ) |
			Symbols::Operator( _ , _ ) => {

				let flags = match self.symbol.clone()? {
					Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
					_ => None
				};

				let first = match self.symbol.clone()? {
					Symbols::LeftParen( _ , _ ) => {
						let symbol11= self.symbol.clone()?;
						self.advance();

						let right = self.parse_qualified_identifier()?;

						match self.symbol.clone()? {
							Symbols::RightParen( _ , _ ) => (),
							_ => return Err(Box::new(format!("Expecting ')' in object type at position: '{}'", self.lexer.get_start_position())))
						}
						let symbol12= self.symbol.clone()?;
						self.advance();

						Some( (Box::new(symbol11), right, Box::new(symbol12)) )
					},
					_ => None
				};

				let second = match self.symbol.clone()? {
					Symbols::SemiColon( _ , _ ) |
					Symbols::Var( _ , _ ) |
					Symbols::Const( _ , _ ) |
					Symbols::Type( _ , _ ) |
					Symbols::Procedure( _ , _ ) |
					Symbols::Operator( _ , _ ) => {
						Some( self.parse_declaration_sequence()? )
					},
					_ => None
				};

				let body = match self.symbol.clone()? {
					Symbols::Begin( _ , _ ) | Symbols::Code( _ , _ ) => Some( self.parse_statement_block()? ),
					_ => None
				};

				match self.symbol.clone()? {
					Symbols::End( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting 'END' in object type at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol2= self.symbol.clone()?;
				self.advance();

				let ident = match self.symbol.clone()? {
					Symbols::Ident( _ , _ , _ ) => {
						let start_pos2 = self.lexer.get_start_position();
						let symbol3 = self.symbol.clone()?;
						self.advance();
						Some( Box::new(Node::Ident(start_pos2, self.lexer.get_start_position(), Box::new(symbol3))) )
					},
					_ => None
				};

				Ok( Box::new(Node::ObjectType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), flags, first, second, body, Box::new(symbol2), ident )) )
			},
			_ => Ok( Box::new(Node::ObjectTypeEmpty(start_pos, self.lexer.get_start_position(), Box::new(symbol1))) )
		}
	}

	fn parse_enumeration_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Enum( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'ENUM' in enum type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		let first = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol11= self.symbol.clone()?;
				self.advance();

				let right = self.parse_qualified_identifier()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in enum type at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol12= self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol11), right, Box::new(symbol12)) )
			},
			_ => None
		};

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());

		match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => {
				let start_pos3 = self.lexer.get_start_position();
				let left = self.parse_identifier_definition()?;

				let right = match self.symbol.clone()? {
					Symbols::Equal( _ , _ ) => {
						let symbol10 = self.symbol.clone()?;
						self.advance();
						let right_2 = self.parse_expression()?;
						Some( (Box::new(symbol10), right_2) )
					},
					_ => None
				};

				nodes.push( Box::new(Node::EnumElement(start_pos3, self.lexer.get_start_position(), left, right)) )
			},
			_ => return Err(Box::new(format!("Expecting at least one Ident in enum type at position: '{}'", self.lexer.get_start_position())))
		}

		loop {
			match self.symbol.clone()? {
				Symbols::Comma( _ , _ ) => {
					separators.push( Box::new(self.symbol.clone()?) );
					self.advance()
				},
				_ => break
			}

			let start_pos2 = self.lexer.get_start_position();
			match self.symbol.clone()? {
				Symbols::Ident( _ , _ , _ ) => (),
				_ => return Err(Box::new(format!("Expecting Ident in enum type after ',' at position: '{}'", self.lexer.get_start_position())))
			}
			let left = self.parse_identifier_definition()?;

			let right = match self.symbol.clone()? {
				Symbols::Equal( _ , _ ) => {
					let symbol10 = self.symbol.clone()?;
					self.advance();
					let right_2 = self.parse_expression()?;
					Some( (Box::new(symbol10), right_2) )
				},
				_ => None
			};

			nodes.push( Box::new(Node::EnumElement(start_pos2, self.lexer.get_start_position(), left, right)) )
		}

		match self.symbol.clone()? {
			Symbols::End( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'END' in enum type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol2= self.symbol.clone()?;
		self.advance();

		Ok( Box::new(Node::EnumerationType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), first, nodes, separators, Box::new(symbol2))) )
	}

	fn parse_cell_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let symbol1 = match self.symbol.clone()? {
			Symbols::Cell( _ , _ ) => {
				let res1 = self.symbol.clone()?;
				self.advance();
				Box::new(res1)
			},
			Symbols::Cellnet( _ , _ ) => {
				let res1 = self.symbol.clone()?;
				self.advance();
				Box::new(res1)
			},
			_ => return Err(Box::new(format!("Expecting 'CELL' or 'CELLNET' in cell or cellnet type at position: '{}'", self.lexer.get_start_position())))
		};

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		let first = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol11= self.symbol.clone()?;
				self.advance();

				let right = self.parse_port_list()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in cell / cellnet type at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol12= self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol11), right, Box::new(symbol12)) )
			},
			_ => None
		};

		let symbol2 = match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => {
				let symbol11= self.symbol.clone()?;
				self.advance();
				Some( Box::new(symbol11) )
			},
			_ => None
		};

		let import = match self.symbol.clone()? {
			Symbols::Import( _ , _ ) => Some( self.parse_import_list()? ),
			_ => None
		};

		let decl = match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) |
			Symbols::Var( _ ,_ ) |
			Symbols::Const( _ , _ ) |
			Symbols::Type( _ , _ ) |
			Symbols::Procedure( _ , _ ) |
			Symbols::Operator( _ , _ ) => {
				Some( self.parse_declaration_sequence()? )
			},
			_ => None
		};

		let body = match self.symbol.clone()? {
			Symbols::Begin( _ , _ ) | Symbols::Code( _ , _ ) => Some( self.parse_statement_block()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::End( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'END' in cell / cellnet type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol3= self.symbol.clone()?;
		self.advance();

		let id = match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => {
				let start_pos3 = self.lexer.get_start_position();
				let idx = self.symbol.clone()?;
				self.advance();
				Some( Box::new( Node::Ident(start_pos3, self.lexer.get_start_position(), Box::new(idx)) ) )
			},
			_ => None
		};

		Ok( Box::new( Node::CellType(start_pos, self.lexer.get_start_position(), symbol1, flags, first, symbol2, import, decl, body, Box::new(symbol3), id  ) ) )
	}

	fn parse_port_list(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());
		let mut is_first = true;

		loop {

			if !is_first {
				match self.symbol.clone()? {
					Symbols::SemiColon( _ , _ ) => {
						separators.push( Box::new(self.symbol.clone()?) );
						self.advance()
					},
					_ => break
				}
				is_first = false
			}

			nodes.push( self.parse_port_declaration()? )
		}

		Ok( Box::new(Node::PortList(start_pos, self.lexer.get_start_position(), nodes, separators)) )
	}

	fn parse_port_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let mut nodes = Box::new(Vec::<Box<(Box<Node>, Option<Box<Node>>)>>::new());
		let mut separators= Box::new(Vec::<Box<Symbols>>::new());
		let mut is_first = true;

		loop {

			if !is_first {
				match self.symbol.clone()? {
					Symbols::Comma( _ , _ ) => {
						separators.push( Box::new(self.symbol.clone()?) );
						self.advance()
					},
					_ => break
				}
				is_first = false
			}

			let ident = match self.symbol.clone()? {
				Symbols::Ident( _ , _ , _ ) => {
					let id = self.symbol.clone()?;
					self.advance();
					Box::new( Node::Ident(start_pos, self.lexer.get_start_position(), Box::new(id)) )
				},
				_ => return Err(Box::new(format!("Expecting 'Ident' literal in port declaration at position: '{}'", self.lexer.get_start_position())))
			};

			let flags = match self.symbol.clone()? {
				Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
				_ => None
			};

			nodes.push(Box::new( ( ident, flags ) ) )
		}

		match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ':' in port declaration at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let right = self.parse_port_type()?;

		Ok( Box::new(Node::PortDeclaration(start_pos, self.lexer.get_start_position(), nodes, separators, Box::new(symbol1), right)) )
	}

	fn parse_port_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Port( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'PORT' in port type at position: '{}'", self.lexer.get_start_position())))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let direction = match self.symbol.clone()? {
			Symbols::In( _ , _ ) |
			Symbols::Out( _ , _ ) => {
				let dir = self.symbol.clone()?;
				self.advance();
				Box::new(dir)
			},
			_ => return Err(Box::new(format!("Expecting 'IN' or 'OUT' in port type at position: '{}'", self.lexer.get_start_position())))
		};

		let first = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol11= self.symbol.clone()?;
				self.advance();

				let right = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in port type at position: '{}'", self.lexer.get_start_position())))
				}
				let symbol12= self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol11), right, Box::new(symbol12)) )
			},
			_ => None
		};

		Ok( Box::new(Node::PortType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), direction, first )) )
	}

	fn parse_qualified_identifier(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Ident( _ , _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting identifier at position: '{}'", self.lexer.get_start_position())))
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
					_ => return Err(Box::new(format!("Expecting identifier after '.' at position: '{}'", self.lexer.get_start_position())))
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
			_ => Err(Box::new(format!("Expecting identifier at position: '{}'", self.lexer.get_start_position())))
		}
	}
}

// Unittests for parser rules

#[cfg(test)]
mod tests {
	use clap::builder::NonEmptyStringValueParser;
	use crate::parser::{Parser, ParserMethods, Node, ExpressionRules, BlockRules, StatementRules};
	use crate::parser::Node::VarDeclaration;
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

	#[test]
	fn statement_while() {
		let mut parser = Parser::new(Box::new(Scanner::new("WHILE test DO count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(14, 20, Box::new( Symbols::Ident(14, 19, Box::new(String::from("count"))) )) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::While(0, 23,
										 Box::new( Symbols::While(0,5) ),
										 Box::new( Node::Ident(6, 11, Box::new( Symbols::Ident(6, 10, Box::new(String::from("test"))) )) ),
										 Box::new( Symbols::Do(11, 13) ),
										 Box::new(
											 Node::StatementSequence(14, 20, nodes, separators)
										 ),
										 Box::new( Symbols::End(20, 23) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_repeat() {
		let mut parser = Parser::new(Box::new(Scanner::new("REPEAT count UNTIL cat")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(7, 13, Box::new( Symbols::Ident(7, 12, Box::new(String::from("count"))) )) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::Repeat(0, 22,
											Box::new( Symbols::Repeat(0,6) ),
											 Box::new(
												 Node::StatementSequence(7, 13, nodes, separators)
											 ),
											Box::new( Symbols::Until(13, 18) ),
											 Box::new( Node::Ident(19, 22, Box::new( Symbols::Ident(19, 22, Box::new(String::from("cat"))) )) )

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_loop_exit() {
		let mut parser = Parser::new(Box::new(Scanner::new("LOOP EXIT END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Exit(5, 10, Box::new(Symbols::Exit(5, 9)) ) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::Loop(0, 13,
											 Box::new( Symbols::Loop(0,4) ),
											 Box::new(
												 Node::StatementSequence(5, 10, nodes, separators)
											 ),
											 Box::new( Symbols::End(10, 13) )

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_block_with_simple_return() {
		let mut parser = Parser::new(Box::new(Scanner::new("BEGIN RETURN END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Return(6, 13, Box::new(Symbols::Return(6, 12)), None ) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::StatementBlock(0, 16,
										   Box::new( Symbols::Begin(0,5) ),
										   None,
										   Box::new(
											   Node::StatementSequence(6, 13, nodes, separators)
										   ),
										   Box::new( Symbols::End(13, 16) )

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_code() {
		let mut parser = Parser::new(Box::new(Scanner::new("CODE END")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::Code(0, 8,
													 Box::new( Symbols::Code(0,4) ),
													 Box::new(Vec::<u8>::new()),
													 Box::new( Symbols::End(5, 8) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_return_expression() {
		let mut parser = Parser::new(Box::new(Scanner::new("RETURN test")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::Return(0, 11,
										   Box::new( Symbols::Return(0,6) ),
										   Some( Box::new(Node::Ident(7, 11, Box::new(Symbols::Ident(7, 11, Box::new(String::from("test")))))) )

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_await_expression() {
		let mut parser = Parser::new(Box::new(Scanner::new("AWAIT test")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::Await(0, 10,
											 Box::new( Symbols::Await(0,5) ),
											 Box::new(Node::Ident(6, 10, Box::new(Symbols::Ident(6, 10, Box::new(String::from("test"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_ignore_expression() {
		let mut parser = Parser::new(Box::new(Scanner::new("IGNORE test")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::Ignore(0, 11,
											Box::new( Symbols::Ignore(0,6) ),
											Box::new(Node::Ident(7, 11, Box::new(Symbols::Ident(7, 11, Box::new(String::from("test"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_expression_becomes() {
		let mut parser = Parser::new(Box::new(Scanner::new("a := 1")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::BecomesStatement(0, 6,
											 Box::new(Node::Ident(0, 2, Box::new(Symbols::Ident(0, 1, Box::new(String::from("a")))))),
											 Box::new( Symbols::Becomes(2,4) ),
											 Box::new(Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_expression_less_less() {
		let mut parser = Parser::new(Box::new(Scanner::new("a << 1")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::LessLessStatement(0, 6,
													   Box::new(Node::Ident(0, 2, Box::new(Symbols::Ident(0, 1, Box::new(String::from("a")))))),
													   Box::new( Symbols::LessLess(2,4) ),
													   Box::new(Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_expression_greater_greater() {
		let mut parser = Parser::new(Box::new(Scanner::new("a >> 1")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::GreaterGreaterStatement(0, 6,
														Box::new(Node::Ident(0, 2, Box::new(Symbols::Ident(0, 1, Box::new(String::from("a")))))),
														Box::new( Symbols::GreaterGreater(2,4) ),
														Box::new(Node::Integer(5, 6, Box::new(Symbols::Integer(5, 6, Box::new(String::from("1"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_expression_exclaim() {
		let mut parser = Parser::new(Box::new(Scanner::new("a ! 1")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::ExclaimMarkStatement(0, 5,
															  Box::new(Node::Ident(0, 2, Box::new(Symbols::Ident(0, 1, Box::new(String::from("a")))))),
															  Box::new( Symbols::ExclaimMark(2,3) ),
															  Box::new(Node::Integer(4, 5, Box::new(Symbols::Integer(4, 5, Box::new(String::from("1"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_expression_question_mark() {
		let mut parser = Parser::new(Box::new(Scanner::new("a ? 1")));
		parser.advance();
		let res = parser.parse_statement();

		let pattern = Box::new( Node::QuestionmarkStatement(0, 5,
														   Box::new(Node::Ident(0, 2, Box::new(Symbols::Ident(0, 1, Box::new(String::from("a")))))),
														   Box::new( Symbols::QuestionMark(2,3) ),
														   Box::new(Node::Integer(4, 5, Box::new(Symbols::Integer(4, 5, Box::new(String::from("1"))))))

		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_for_without_by() {
		let mut parser = Parser::new(Box::new(Scanner::new("FOR i := 1 TO 100 DO count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(21, 27, Box::new(Symbols::Ident(21, 26, Box::new(String::from("count")))) ) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::For(0, 30,
												 Box::new( Symbols::For(0,3) ),
												 Box::new( Node::Ident(4, 6, Box::new(Symbols::Ident(4, 5, Box::new(String::from("i"))))) ),
												 Box::new( Symbols::Becomes(6, 8) ),
												 Box::new( Node::Integer(9, 11, Box::new(Symbols::Integer(9, 10, Box::new(String::from("1"))))) ),
												 Box::new( Symbols::To(11, 13)),
												 Box::new( Node::Integer(14, 18, Box::new(Symbols::Integer(14, 17, Box::new(String::from("100"))))) ),
												 None,
												 Box::new(Symbols::Do(18, 20)),
												 Box::new(
													  Node::StatementSequence(21, 27, nodes, separators)
												 ),
												Box::new( Symbols::End(27, 30) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_for_with_by() {
		let mut parser = Parser::new(Box::new(Scanner::new("FOR i := 1 TO 100 BY 10 DO count END")));
		parser.advance();
		let res = parser.parse_statement();

		let nodes : Box<Vec<Box<Node>>> =  Box::new( [
			Box::new( Node::Ident(27, 33, Box::new(Symbols::Ident(27, 32, Box::new(String::from("count")))) ) )
		].to_vec() );
		let separators : Box<Vec<Box<Symbols>>> =  Box::new( [].to_vec() );

		let pattern = Box::new( Node::For(0, 36,
										  Box::new( Symbols::For(0,3) ),
										  Box::new( Node::Ident(4, 6, Box::new(Symbols::Ident(4, 5, Box::new(String::from("i"))))) ),
										  Box::new( Symbols::Becomes(6, 8) ),
										  Box::new( Node::Integer(9, 11, Box::new(Symbols::Integer(9, 10, Box::new(String::from("1"))))) ),
										  Box::new( Symbols::To(11, 13)),
										  Box::new( Node::Integer(14, 18, Box::new(Symbols::Integer(14, 17, Box::new(String::from("100"))))) ),
										  Some((Box::new(Symbols::By(18, 20)), Box::new(Node::Integer(21, 24, Box::new(Symbols::Integer(21, 23, Box::new(String::from("10")))))))),
										  Box::new(Symbols::Do(24, 26)),
										  Box::new(
											  Node::StatementSequence(27, 33, nodes, separators)
										  ),
										  Box::new( Symbols::End(33, 36) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_with_single_without_else() {
		let mut parser = Parser::new(Box::new(Scanner::new("WITH a : tall DO test END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 21, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::With(0, 25,
										  Box::new( Symbols::With(0,4) ),
										  Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										  Box::new( Symbols::Colon(7, 8) ),
										  Box::new([
												Box::new( Node::WithElement(9, 22,
																			None,
																			Box::new( Node::Ident(9, 14, Box::new(Symbols::Ident(9, 13, Box::new(String::from("tall"))))) ),
																			Box::new(Symbols::Do(14, 16)),
																			Box::new( Node::StatementSequence(17, 22, Box::new(element_1_nodes), Box::new(element_1_separators)) ))
												)
										  ].to_vec()),
										  None,
										  Box::new( Symbols::End(22, 25) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_with_multiple_without_else() {
		let mut parser = Parser::new(Box::new(Scanner::new("WITH a : tall DO test | dog DO test END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 21, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(31, 36, Box::new(Symbols::Ident(31, 35, Box::new(String::from("test"))))))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::With(0, 39,
										   Box::new( Symbols::With(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Colon(7, 8) ),
										   Box::new([
											   Box::new( Node::WithElement(9, 22,
																		   None,
																		   Box::new( Node::Ident(9, 14, Box::new(Symbols::Ident(9, 13, Box::new(String::from("tall"))))) ),
																		   Box::new(Symbols::Do(14, 16)),
																		   Box::new( Node::StatementSequence(17, 22, Box::new(element_1_nodes), Box::new(element_1_separators)) ))
											   ),
											   Box::new( Node::WithElement(22, 36,
																		   Some(Box::new(Symbols::Bar(22, 23))),
																		   Box::new( Node::Ident(24, 28, Box::new(Symbols::Ident(24, 27, Box::new(String::from("dog"))))) ),
																		   Box::new(Symbols::Do(28, 30)),
																		   Box::new( Node::StatementSequence(31, 36, Box::new(element_2_nodes), Box::new(element_2_separators)) ))
											   )
										   ].to_vec()),
										   None,
										   Box::new( Symbols::End(36, 39) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_with_multiple_with_else() {
		let mut parser = Parser::new(Box::new(Scanner::new("WITH a : tall DO test | dog DO test ELSE cat END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 21, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(31, 36, Box::new(Symbols::Ident(31, 35, Box::new(String::from("test"))))))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_3_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(41, 45, Box::new(Symbols::Ident(41, 44, Box::new(String::from("cat"))))))
		].to_vec();
		let element_3_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::With(0, 48,
										   Box::new( Symbols::With(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Colon(7, 8) ),
										   Box::new([
											   Box::new( Node::WithElement(9, 22,
																		   None,
																		   Box::new( Node::Ident(9, 14, Box::new(Symbols::Ident(9, 13, Box::new(String::from("tall"))))) ),
																		   Box::new(Symbols::Do(14, 16)),
																		   Box::new( Node::StatementSequence(17, 22, Box::new(element_1_nodes), Box::new(element_1_separators)) ))
											   ),
											   Box::new( Node::WithElement(22, 36,
																		   Some(Box::new(Symbols::Bar(22, 23))),
																		   Box::new( Node::Ident(24, 28, Box::new(Symbols::Ident(24, 27, Box::new(String::from("dog"))))) ),
																		   Box::new(Symbols::Do(28, 30)),
																		   Box::new( Node::StatementSequence(31, 36, Box::new(element_2_nodes), Box::new(element_2_separators)) ))
											   )
										   ].to_vec()),
										   Some(
											   Box::new( Node::Else(36, 45, Box::new(Symbols::Else(36, 40)),
																	Box::new(
																		 Node::StatementSequence(41, 45, Box::new(element_3_nodes), Box::new(element_3_separators))
																	)) )
										   ),
										   Box::new( Symbols::End(45, 48) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_case_single_without_else_and_bar() {
		let mut parser = Parser::new(Box::new(Scanner::new("CASE a OF 1 ..100: test END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(19, 24, Box::new(Symbols::Ident(19, 23, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::Case(0, 27,
										   Box::new( Symbols::Case(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Of(7, 9) ),
										   Box::new([
											   Box::new( Node::CaseElement(10, 24,
																		   None,
												   							Box::new(
																				[
																					Box::new(
																						Node::Range(
																							10, 17,
																							Some(Box::new( Node::Integer(10, 12, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))) )),
																							Box::new(Symbols::Upto(12, 14)),
																							Some(Box::new( Node::Integer(14, 17, Box::new(Symbols::Integer(14, 17, Box::new(String::from("100"))))) )),
																							None,
																							None
																						)
																					)
																				].to_vec()
																			),
												   							Box::new( [].to_vec() ),
												   							Box::new(Symbols::Colon(17, 18)),
												   							Box::new(
																				Node::StatementSequence(19, 24, Box::new(element_1_nodes), Box::new(element_1_separators))
																			)
																		     ))
										   ].to_vec()),
										   None,
										   Box::new( Symbols::End(24, 27) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_case_single_without_else() {
		let mut parser = Parser::new(Box::new(Scanner::new("CASE a OF | 1 ..100: test END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(21, 26, Box::new(Symbols::Ident(21, 25, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::Case(0, 29,
										   Box::new( Symbols::Case(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Of(7, 9) ),
										   Box::new([
											   Box::new( Node::CaseElement(10, 26,
																		   Some( Box::new(Symbols::Bar(10, 11)) ),
																		   Box::new(
																			   [
																				   Box::new(
																					   Node::Range(
																						   12, 19,
																						   Some(Box::new( Node::Integer(12, 14, Box::new(Symbols::Integer(12, 13, Box::new(String::from("1"))))) )),
																						   Box::new(Symbols::Upto(14, 16)),
																						   Some(Box::new( Node::Integer(16, 19, Box::new(Symbols::Integer(16, 19, Box::new(String::from("100"))))) )),
																						   None,
																						   None
																					   )
																				   )
																			   ].to_vec()
																		   ),
																		   Box::new( [].to_vec() ),
																		   Box::new(Symbols::Colon(19, 20)),
																		   Box::new(
																			   Node::StatementSequence(21, 26, Box::new(element_1_nodes), Box::new(element_1_separators))
																		   )
											   ))
										   ].to_vec()),
										   None,
										   Box::new( Symbols::End(26, 29) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_case_multiple_without_else() {
		let mut parser = Parser::new(Box::new(Scanner::new("CASE a OF | 1 ..100: test | *: dog END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(21, 26, Box::new(Symbols::Ident(21, 25, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(31, 35, Box::new(Symbols::Ident(31, 34, Box::new(String::from("dog"))))))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [].to_vec();

		let pattern = Box::new( Node::Case(0, 38,
										   Box::new( Symbols::Case(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Of(7, 9) ),
										   Box::new([
											   Box::new( Node::CaseElement(10, 26,
																		   Some( Box::new(Symbols::Bar(10, 11)) ),
																		   Box::new(
																			   [
																				   Box::new(
																					   Node::Range(
																						   12, 19,
																						   Some(Box::new( Node::Integer(12, 14, Box::new(Symbols::Integer(12, 13, Box::new(String::from("1"))))) )),
																						   Box::new(Symbols::Upto(14, 16)),
																						   Some(Box::new( Node::Integer(16, 19, Box::new(Symbols::Integer(16, 19, Box::new(String::from("100"))))) )),
																						   None,
																						   None
																					   )
																				   )
																			   ].to_vec()
																		   ),
																		   Box::new( [].to_vec() ),
																		   Box::new(Symbols::Colon(19, 20)),
																		   Box::new(
																			   Node::StatementSequence(21, 26, Box::new(element_1_nodes), Box::new(element_1_separators))
																		   )
											   )),

											   Box::new( Node::CaseElement(26, 35,
																		   Some( Box::new(Symbols::Bar(26, 27)) ),
																		   Box::new(
																			   [
																				   Box::new(
																					   Node::Range(
																						   28, 29,
																						   None,
																						   Box::new(Symbols::Times(28, 29)),
																						   None,
																						   None,
																						   None
																					   )
																				   )
																			   ].to_vec()
																		   ),
																		   Box::new( [].to_vec() ),
																		   Box::new(Symbols::Colon(29, 30)),
																		   Box::new(
																			   Node::StatementSequence(31, 35, Box::new(element_2_nodes), Box::new(element_2_separators))
																		   )
											   ))

										   ].to_vec()),
										   None,
										   Box::new( Symbols::End(35, 38) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn statement_case_multiple_witht_else_and_statement_sequence_multiple() {
		let mut parser = Parser::new(Box::new(Scanner::new("CASE a OF | 1 ..100: test | *: dog ELSE run; stop END")));
		parser.advance();
		let res = parser.parse_statement();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(21, 26, Box::new(Symbols::Ident(21, 25, Box::new(String::from("test"))))))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(31, 35, Box::new(Symbols::Ident(31, 34, Box::new(String::from("dog"))))))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [].to_vec();

		let element_3_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Ident(40, 43, Box::new(Symbols::Ident(40, 43, Box::new(String::from("run")))))),
			Box::new(Node::Ident(45, 50, Box::new(Symbols::Ident(45, 49, Box::new(String::from("stop"))))))
		].to_vec();
		let element_3_separators : Vec<Box<Symbols>> = [
			Box::new(Symbols::SemiColon(43, 44))
		].to_vec();

		let pattern = Box::new( Node::Case(0, 53,
										   Box::new( Symbols::Case(0,4) ),
										   Box::new( Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a"))))) ),
										   Box::new( Symbols::Of(7, 9) ),
										   Box::new([
											   Box::new( Node::CaseElement(10, 26,
																		   Some( Box::new(Symbols::Bar(10, 11)) ),
																		   Box::new(
																			   [
																				   Box::new(
																					   Node::Range(
																						   12, 19,
																						   Some(Box::new( Node::Integer(12, 14, Box::new(Symbols::Integer(12, 13, Box::new(String::from("1"))))) )),
																						   Box::new(Symbols::Upto(14, 16)),
																						   Some(Box::new( Node::Integer(16, 19, Box::new(Symbols::Integer(16, 19, Box::new(String::from("100"))))) )),
																						   None,
																						   None
																					   )
																				   )
																			   ].to_vec()
																		   ),
																		   Box::new( [].to_vec() ),
																		   Box::new(Symbols::Colon(19, 20)),
																		   Box::new(
																			   Node::StatementSequence(21, 26, Box::new(element_1_nodes), Box::new(element_1_separators))
																		   )
											   )),

											   Box::new( Node::CaseElement(26, 35,
																		   Some( Box::new(Symbols::Bar(26, 27)) ),
																		   Box::new(
																			   [
																				   Box::new(
																					   Node::Range(
																						   28, 29,
																						   None,
																						   Box::new(Symbols::Times(28, 29)),
																						   None,
																						   None,
																						   None
																					   )
																				   )
																			   ].to_vec()
																		   ),
																		   Box::new( [].to_vec() ),
																		   Box::new(Symbols::Colon(29, 30)),
																		   Box::new(
																			   Node::StatementSequence(31, 35, Box::new(element_2_nodes), Box::new(element_2_separators))
																		   )
											   ))

										   ].to_vec()),
										   Some(
											   Box::new( Node::Else(35, 50, Box::new(Symbols::Else(35, 39)), Box::new(
												   Node::StatementSequence(40, 50, Box::new(element_3_nodes), Box::new(element_3_separators)
											   ) )) )
										   ),
										   Box::new( Symbols::End(50, 53) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test; END Test.")));
		let res = parser.parse_module();

		let pattern = Box::new(Node::Module(0, 22,
			Box::new(Symbols::Module(0, 6)),
			None,
			Box::new(Node::Ident(7, 11, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
			None,
			Box::new(Symbols::SemiColon(11, 12)),
			None,
			None,
			None,
			Box::new(Symbols::End(13, 16)),
			Box::new(Node::Ident(17, 21, Box::new(Symbols::Ident(17, 21, Box::new(String::from("Test")))))),
			Box::new(Symbols::Period(21, 22))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_begin() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test; BEGIN test := 1; run := 6 END Test.")));
		let res = parser.parse_module();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::BecomesStatement(19, 28,
				Box::new(Node::Ident(19, 24, Box::new(Symbols::Ident(19, 23, Box::new(String::from("test")))))),
				Box::new(Symbols::Becomes(24, 26)),
				Box::new(Node::Integer(27, 28, Box::new(Symbols::Integer(27, 28, Box::new(String::from("1"))))))
			)),
			Box::new(Node::BecomesStatement(30, 39,
											Box::new(Node::Ident(30, 34, Box::new(Symbols::Ident(30, 33, Box::new(String::from("run")))))),
											Box::new(Symbols::Becomes(34, 36)),
											Box::new(Node::Integer(37, 39, Box::new(Symbols::Integer(37, 38, Box::new(String::from("6"))))))
			))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [ Box::new(Symbols::SemiColon(28, 29)) ].to_vec();

		let pattern = Box::new(Node::Module(0, 48,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 11, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											None,
											Box::new(Symbols::SemiColon(11, 12)),
											None,
											None,
											Some(
												Box::new(
													Node::Body(13, 39,
														Box::new(Symbols::Begin(13, 18)),
														None,
														Box::new(Node::StatementSequence(
															19, 39,
															Box::new(element_1_nodes),
															Box::new(element_1_separators)
														)),
														None
													)
												)
											),
											Box::new(Symbols::End(39, 42)),
											Box::new(Node::Ident(43, 47, Box::new(Symbols::Ident(43, 47, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(47, 48))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_simple_template() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE ( CONST a ) Test; BEGIN test := 1; run := 6 END Test.")));
		let res = parser.parse_module();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::BecomesStatement(31, 40,
											Box::new(Node::Ident(31, 36, Box::new(Symbols::Ident(31, 35, Box::new(String::from("test")))))),
											Box::new(Symbols::Becomes(36, 38)),
											Box::new(Node::Integer(39, 40, Box::new(Symbols::Integer(39, 40, Box::new(String::from("1"))))))
			)),
			Box::new(Node::BecomesStatement(42, 51,
											Box::new(Node::Ident(42, 46, Box::new(Symbols::Ident(42, 45, Box::new(String::from("run")))))),
											Box::new(Symbols::Becomes(46, 48)),
											Box::new(Node::Integer(49, 51, Box::new(Symbols::Integer(49, 50, Box::new(String::from("6"))))))
			))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [ Box::new(Symbols::SemiColon(40, 41)) ].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::TemplateParameter(
				9, 17,
				Box::new(Symbols::Const(9, 14)),
				Box::new(Node::Ident(15, 17, Box::new(Symbols::Ident(15, 16, Box::new(String::from("a"))))))
			))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [  ].to_vec();

		let pattern = Box::new(Node::Module(0, 60,
											Box::new(Symbols::Module(0, 6)),
											Some(
												Box::new(Node::TemplateParameters(7, 19,
												Box::new(Symbols::LeftParen(7, 8)),
												Box::new(element_2_nodes),
												Box::new(element_2_separators),
												Box::new(Symbols::RightParen(17, 18))
												))
											),
											Box::new(Node::Ident(19, 23, Box::new(Symbols::Ident(19, 23, Box::new(String::from("Test")))))),
											None,
											Box::new(Symbols::SemiColon(23, 24)),
											None,
											None,
											Some(
												Box::new(
													Node::Body(25, 51,
															   Box::new(Symbols::Begin(25, 30)),
															   None,
															   Box::new(Node::StatementSequence(
																   31, 51,
																   Box::new(element_1_nodes),
																   Box::new(element_1_separators)
															   )),
															   None
													)
												)
											),
											Box::new(Symbols::End(51, 54)),
											Box::new(Node::Ident(55, 59, Box::new(Symbols::Ident(55, 59, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(59, 60))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_double_template() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE ( CONST a, TYPE b ) Test; END Test.")));
		let res = parser.parse_module();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new(Node::TemplateParameter(
				9, 16,
				Box::new(Symbols::Const(9, 14)),
				Box::new(Node::Ident(15, 16, Box::new(Symbols::Ident(15, 16, Box::new(String::from("a"))))))
			)),
			Box::new(Node::TemplateParameter(
				18, 25,
				Box::new(Symbols::Type(18, 22)),
				Box::new(Node::Ident(23, 25, Box::new(Symbols::Ident(23, 24, Box::new(String::from("b"))))))
			))
		].to_vec();
		let element_1_separators : Vec<Box<Symbols>> = [
			Box::new(Symbols::Comma(16, 17))
		].to_vec();

		let pattern = Box::new(Node::Module(0, 42,
											Box::new(Symbols::Module(0, 6)),
											Some(
												Box::new(Node::TemplateParameters(7, 27,
																				  Box::new(Symbols::LeftParen(7, 8)),
																				  Box::new(element_1_nodes),
																				  Box::new(element_1_separators),
																				  Box::new(Symbols::RightParen(25, 26))
												))
											),
											Box::new(Node::Ident(27, 31, Box::new(Symbols::Ident(27, 31, Box::new(String::from("Test")))))),
											None,
											Box::new(Symbols::SemiColon(31, 32)),
											None,
											None,
											None,
											Box::new(Symbols::End(33, 36)),
											Box::new(Node::Ident(37, 41, Box::new(Symbols::Ident(37, 41, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(41, 42))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; END Test.")));
		let res = parser.parse_module();

		let pattern = Box::new(Node::Module(0, 29,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											None,
											None,
											None,
											Box::new(Symbols::End(20, 23)),
											Box::new(Node::Ident(24, 28, Box::new(Symbols::Ident(24, 28, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(28, 29))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io; END Test.")));
		let res = parser.parse_module();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 29, Box::new(Node::Ident(27, 29, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))), None, None, None))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 31,
				Box::new(Symbols::Import(20, 26)),
				Box::new(element_2_nodes),
				Box::new(element_2_separators),
				Box::new(Symbols::SemiColon(29, 30))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 40,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(31, 34)),
											Box::new(Node::Ident(35, 39, Box::new(Symbols::Ident(35, 39, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(39, 40))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_multiple() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io; IMPORT SYSTEM; END Test.")));
		let res = parser.parse_module();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 29, Box::new(Node::Ident(27, 29, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))), None, None, None))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_3_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(38, 44, Box::new(Node::Ident(38, 44, Box::new(Symbols::Ident(38, 44, Box::new(String::from("SYSTEM")))))), None, None, None))
		].to_vec();
		let element_3_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 31,
									   Box::new(Symbols::Import(20, 26)),
									   Box::new(element_2_nodes),
									   Box::new(element_2_separators),
									   Box::new(Symbols::SemiColon(29, 30))
			) ),
			Box::new( Node::ImportList(31, 46,
									   Box::new(Symbols::Import(31, 37)),
									   Box::new(element_3_nodes),
									   Box::new(element_3_separators),
									   Box::new(Symbols::SemiColon(44, 45))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 55,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(46, 49)),
											Box::new(Node::Ident(50, 54, Box::new(Symbols::Ident(50, 54, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(54, 55))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_simple_with_assignment() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io := InOut; END Test.")));
		let res = parser.parse_module();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 38,
								  Box::new(Node::Ident(27, 30, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))),
								  Some( (Box::new(Symbols::Becomes(30, 32)), Box::new(Node::Ident(33, 38, Box::new(Symbols::Ident(33, 38, Box::new(String::from("InOut"))))))) ),
								  None,
								  None
			))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 40,
									   Box::new(Symbols::Import(20, 26)),
									   Box::new(element_2_nodes),
									   Box::new(element_2_separators),
									   Box::new(Symbols::SemiColon(38, 39))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 49,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(40, 43)),
											Box::new(Node::Ident(44, 48, Box::new(Symbols::Ident(44, 48, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(48, 49))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_multiple_with_assignment() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io := InOut, SYSTEM; END Test.")));
		let res = parser.parse_module();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 38,
								  Box::new(Node::Ident(27, 30, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))),
								  Some( (Box::new(Symbols::Becomes(30, 32)), Box::new(Node::Ident(33, 38, Box::new(Symbols::Ident(33, 38, Box::new(String::from("InOut"))))))) ),
								  None,
								  None
			)),
			Box::new(Node::Import(40, 46,
								  Box::new(Node::Ident(40, 46, Box::new(Symbols::Ident(40, 46, Box::new(String::from("SYSTEM")))))),
								  None,
								  None,
								  None
			))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [
			Box::new(Symbols::Comma(38, 39))
		].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 48,
									   Box::new(Symbols::Import(20, 26)),
									   Box::new(element_2_nodes),
									   Box::new(element_2_separators),
									   Box::new(Symbols::SemiColon(46, 47))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 57,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(48, 51)),
											Box::new(Node::Ident(52, 56, Box::new(Symbols::Ident(52, 56, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(56, 57))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_simple_with_assignment_and_parenthesis() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io := InOut( a, b ); END Test.")));
		let res = parser.parse_module();

		let element_3_nodes = [
			Box::new(Node::Ident(40, 41, Box::new(Symbols::Ident(40, 41, Box::new(String::from("a")))))),
			Box::new(Node::Ident(43, 45, Box::new(Symbols::Ident(43, 44, Box::new(String::from("b"))))))
		].to_vec();
		let element_3_separators = [
			Box::new(Symbols::Comma(41, 42))
		].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 46,
								  Box::new(Node::Ident(27, 30, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))),
								  Some( (Box::new(Symbols::Becomes(30, 32)), Box::new(Node::Ident(33, 38, Box::new(Symbols::Ident(33, 38, Box::new(String::from("InOut"))))))) ),
								  Some( (
									  	Box::new(Symbols::LeftParen(38, 39)),
									  	Box::new( Node::ExpressionList(40, 45, Box::new(element_3_nodes), Box::new(element_3_separators)) ),
									  	Box::new(Symbols::RightParen(45, 46))
									  ) ),
								  None
			))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 48,
									   Box::new(Symbols::Import(20, 26)),
									   Box::new(element_2_nodes),
									   Box::new(element_2_separators),
									   Box::new(Symbols::SemiColon(46, 47))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 57,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(48, 51)),
											Box::new(Node::Ident(52, 56, Box::new(Symbols::Ident(52, 56, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(56, 57))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_module_in_import_simple_with_assignment_and_parenthesis_in_name() {
		let mut parser = Parser::new(Box::new(Scanner::new("MODULE Test IN run; IMPORT io := InOut( a, b ) IN r; END Test.")));
		let res = parser.parse_module();

		let element_3_nodes = [
			Box::new(Node::Ident(40, 41, Box::new(Symbols::Ident(40, 41, Box::new(String::from("a")))))),
			Box::new(Node::Ident(43, 45, Box::new(Symbols::Ident(43, 44, Box::new(String::from("b"))))))
		].to_vec();
		let element_3_separators = [
			Box::new(Symbols::Comma(41, 42))
		].to_vec();

		let element_2_nodes : Vec::<Box<Node>> = [
			Box::new(Node::Import(27, 51,
								  Box::new(Node::Ident(27, 30, Box::new(Symbols::Ident(27, 29, Box::new(String::from("io")))))),
								  Some( (Box::new(Symbols::Becomes(30, 32)), Box::new(Node::Ident(33, 38, Box::new(Symbols::Ident(33, 38, Box::new(String::from("InOut"))))))) ),
								  Some( (
									  Box::new(Symbols::LeftParen(38, 39)),
									  Box::new( Node::ExpressionList(40, 45, Box::new(element_3_nodes), Box::new(element_3_separators)) ),
									  Box::new(Symbols::RightParen(45, 46))
								  ) ),
								  Some( (Box::new(Symbols::In(47, 49)), Box::new(Node::Ident(50, 51, Box::new(Symbols::Ident(50, 51, Box::new(String::from("r"))))))) )
			))
		].to_vec();
		let element_2_separators : Vec<Box<Symbols>> = [ ].to_vec();

		let element_1_nodes : Vec::<Box<Node>> = [
			Box::new( Node::ImportList(20, 53,
									   Box::new(Symbols::Import(20, 26)),
									   Box::new(element_2_nodes),
									   Box::new(element_2_separators),
									   Box::new(Symbols::SemiColon(51, 52))
			) )
		].to_vec();

		let pattern = Box::new(Node::Module(0, 62,
											Box::new(Symbols::Module(0, 6)),
											None,
											Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 11, Box::new(String::from("Test")))))),
											Some( (
												Box::new(Symbols::In(12, 14)),
												Box::new(Node::Ident(15, 18, Box::new(Symbols::Ident(15, 18, Box::new(String::from("run")))))))
											),
											Box::new(Symbols::SemiColon(18, 19)),
											Some( Box::new(element_1_nodes) ),
											None,
											None,
											Box::new(Symbols::End(53, 56)),
											Box::new(Node::Ident(57, 61, Box::new(Symbols::Ident(57, 61, Box::new(String::from("Test")))))),
											Box::new(Symbols::Period(61, 62))
		));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn empty_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("{}")));
		parser.advance();
		let res = parser.parse_flags();

		let nodes = [].to_vec();
		let separators = [].to_vec();

		let pattern = Box::new( Node::Flags(0, 2, Box::new(Symbols::LeftBrace(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightBrace(1, 2))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn empty_flags_single() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ a }")));
		parser.advance();
		let res = parser.parse_flags();

		let nodes = [
			Box::new(Node::Flag(2, 4, Box::new(Node::Ident(2, 4, Box::new(Symbols::Ident(2, 3, Box::new(String::from("a")))))), None, None))
		].to_vec();
		let separators = [].to_vec();

		let pattern = Box::new( Node::Flags(0, 5, Box::new(Symbols::LeftBrace(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightBrace(4, 5))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn empty_flags_multiple() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ a, b }")));
		parser.advance();
		let res = parser.parse_flags();

		let nodes = [
			Box::new(Node::Flag(2, 3, Box::new(Node::Ident(2, 3, Box::new(Symbols::Ident(2, 3, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Flag(5, 7, Box::new(Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("b")))))), None, None))
		].to_vec();
		let separators = [
			Box::new(Symbols::Comma(3, 4))
		].to_vec();

		let pattern = Box::new( Node::Flags(0, 8, Box::new(Symbols::LeftBrace(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightBrace(7, 8))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn empty_flags_multiple_equal() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ a, b = c }")));
		parser.advance();
		let res = parser.parse_flags();

		let nodes = [
			Box::new(Node::Flag(2, 3, Box::new(Node::Ident(2, 3, Box::new(Symbols::Ident(2, 3, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Flag(5, 11, Box::new(Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("b")))))), None, Some( (
				Box::new(Symbols::Equal(7, 8)),
				Box::new(Node::Ident(9, 11, Box::new(Symbols::Ident(9, 10, Box::new(String::from("c"))))))
			) )))
		].to_vec();
		let separators = [
			Box::new(Symbols::Comma(3, 4))
		].to_vec();

		let pattern = Box::new( Node::Flags(0, 12, Box::new(Symbols::LeftBrace(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightBrace(11, 12))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn empty_flags_multiple_parenthesis() {
		let mut parser = Parser::new(Box::new(Scanner::new("{ a, b ( c ) }")));
		parser.advance();
		let res = parser.parse_flags();

		let nodes = [
			Box::new(Node::Flag(2, 3, Box::new(Node::Ident(2, 3, Box::new(Symbols::Ident(2, 3, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Flag(5, 13, Box::new(Node::Ident(5, 7, Box::new(Symbols::Ident(5, 6, Box::new(String::from("b")))))), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(9, 11, Box::new(Symbols::Ident(9, 10, Box::new(String::from("c")))))),
					Box::new(Symbols::RightParen(11, 12))
				)
			), None))
		].to_vec();
		let separators = [
			Box::new(Symbols::Comma(3, 4))
		].to_vec();

		let pattern = Box::new( Node::Flags(0, 14, Box::new(Symbols::LeftBrace(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightBrace(13, 14))));

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("()")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let nodes = [].to_vec();
		let separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 2, Box::new(Symbols::LeftParen(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightParen(1, 2)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_empty_with_type() {
		let mut parser = Parser::new(Box::new(Scanner::new("() : INT64")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let nodes = [].to_vec();
		let separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 10, Box::new(Symbols::LeftParen(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightParen(1, 2)),
													   Some( ( Box::new(Symbols::Colon(3, 4)), None, Box::new(Node::Ident(5, 10, Box::new(Symbols::Ident(5, 10, Box::new(String::from("INT64")))))) ) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_empty_with_type_and_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("() : {} INT64")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let nodes = [].to_vec();
		let separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 13, Box::new(Symbols::LeftParen(0, 1)), Box::new(nodes), Box::new(separators), Box::new(Symbols::RightParen(1, 2)),
													   Some( ( Box::new(Symbols::Colon(3, 4)),
															   Some( Box::new(Node::Flags(
																   5, 8, Box::new(Symbols::LeftBrace(5, 6)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(6, 7))
															   ))),
															   Box::new(Node::Ident(8, 13, Box::new(Symbols::Ident(8, 13, Box::new(String::from("INT64")))))) ) )
		) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_single_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("(a: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(1, 2, Box::new(Node::Ident(1, 2, Box::new(Symbols::Ident(1, 2, Box::new(String::from("a")))))), None, None))
		].to_vec();
		let fd_separators = [].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 9, None, Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(2, 3)), Box::new(Node::Ident(4, 9, Box::new(Symbols::Ident(4, 9, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 10, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(9, 10)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("(a, b: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(1, 2, Box::new(Node::Ident(1, 2, Box::new(Symbols::Ident(1, 2, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Parameter(4, 5, Box::new(Node::Ident(4, 5, Box::new(Symbols::Ident(4, 5, Box::new(String::from("b")))))), None, None))
		].to_vec();
		let fd_separators = [
			Box::new(Symbols::Comma(2, 3))
		].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 12, None, Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(5, 6)), Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 12, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 13, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(12, 13)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple_var() {
		let mut parser = Parser::new(Box::new(Scanner::new("(VAR a, b: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(5, 6, Box::new(Node::Ident(5, 6, Box::new(Symbols::Ident(5, 6, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Parameter(8, 9, Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("b")))))), None, None))
		].to_vec();
		let fd_separators = [
			Box::new(Symbols::Comma(6, 7))
		].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 16, Some(Box::new(Symbols::Var(1, 4))), Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(9, 10)), Box::new(Node::Ident(11, 16, Box::new(Symbols::Ident(11, 16, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 17, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(16, 17)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple_const() {
		let mut parser = Parser::new(Box::new(Scanner::new("(CONST a, b: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(7, 8, Box::new(Node::Ident(7, 8, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Parameter(10, 11, Box::new(Node::Ident(10, 11, Box::new(Symbols::Ident(10, 11, Box::new(String::from("b")))))), None, None))
		].to_vec();
		let fd_separators = [
			Box::new(Symbols::Comma(8, 9))
		].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 18, Some(Box::new(Symbols::Const(1, 6))), Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(11, 12)), Box::new(Node::Ident(13, 18, Box::new(Symbols::Ident(13, 18, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 19, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(18, 19)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple_const_and_value() {
		let mut parser = Parser::new(Box::new(Scanner::new("(CONST a, b = 1: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(7, 8, Box::new(Node::Ident(7, 8, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Parameter(10, 15, Box::new(Node::Ident(10, 12, Box::new(Symbols::Ident(10, 11, Box::new(String::from("b")))))), None,
				Some( (Box::new(Symbols::Equal(12, 13)), Box::new(Node::Integer(14, 15, Box::new(Symbols::Integer(14, 15, Box::new(String::from("1"))))))) )
			))
		].to_vec();
		let fd_separators = [
			Box::new(Symbols::Comma(8, 9))
		].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 22, Some(Box::new(Symbols::Const(1, 6))), Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(15, 16)), Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 22, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 23, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(22, 23)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple_const_and_value_and_flag() {
		let mut parser = Parser::new(Box::new(Scanner::new("(CONST a, b {} = 1: INT64)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(7, 8, Box::new(Node::Ident(7, 8, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))), None, None)),
			Box::new(Node::Parameter(10, 18, Box::new(Node::Ident(10, 12, Box::new(Symbols::Ident(10, 11, Box::new(String::from("b")))))),
									 Some( Box::new( Node::Flags(12, 15, Box::new(Symbols::LeftBrace(12, 13)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(13, 14))) )),
									 Some( (Box::new(Symbols::Equal(15, 16)), Box::new(Node::Integer(17, 18, Box::new(Symbols::Integer(17, 18, Box::new(String::from("1"))))))) )
			))
		].to_vec();
		let fd_separators = [
			Box::new(Symbols::Comma(8, 9))
		].to_vec();

		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 25, Some(Box::new(Symbols::Const(1, 6))), Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(18, 19)), Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64"))))))))
		].to_vec();
		let fp_separators = [].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 26, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(25, 26)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn formal_parameters_multiple_simple_with_types() {
		let mut parser = Parser::new(Box::new(Scanner::new("(a: INT64; b: BOOLEAN)")));
		parser.advance();
		let res = parser.parse_formal_parameters();

		let fd_nodes = [
			Box::new(Node::Parameter(1, 2, Box::new(Node::Ident(1, 2, Box::new(Symbols::Ident(1, 2, Box::new(String::from("a")))))), None, None))
		].to_vec();
		let fd_separators = [].to_vec();


		let fd_nodes2 = [
			Box::new(Node::Parameter(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("b")))))), None, None))
		].to_vec();
		let fd_separators2 = [].to_vec();


		let fp_nodes = [
			Box::new(Node::ParameterDeclaration(1, 9, None, Box::new(fd_nodes), Box::new(fd_separators), Box::new(Symbols::Colon(2, 3)), Box::new(Node::Ident(4, 9, Box::new(Symbols::Ident(4, 9, Box::new(String::from("INT64")))))))),
			Box::new(Node::ParameterDeclaration(11, 21, None, Box::new(fd_nodes2), Box::new(fd_separators2), Box::new(Symbols::Colon(12, 13)), Box::new(Node::Ident(14, 21, Box::new(Symbols::Ident(14, 21, Box::new(String::from("BOOLEAN"))))))))
		].to_vec();
		let fp_separators = [
			Box::new(Symbols::SemiColon(9, 10))
		].to_vec();

		let pattern = Box::new( Node::FormalParameters(0, 22, Box::new(Symbols::LeftParen(0, 1)), Box::new(fp_nodes), Box::new(fp_separators), Box::new(Symbols::RightParen(21, 22)), None) );

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 22,
				Box::new(Symbols::Procedure(0, 9)),
				None,
				None,
				Box::new(Node::Ident(10, 13, Box::new(Symbols::Ident(10, 13, Box::new(String::from("Run")))))),
				None,
				Box::new(Symbols::SemiColon(13, 14)),
				None,
				None,
				Box::new(Symbols::End(15, 18)),
				Box::new(Node::Ident(19, 22, Box::new(Symbols::Ident(19, 22, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_arrow() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE^ Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 23,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (None, Some(Box::new(Symbols::Arrow(9, 10)))) ),
							None,
							Box::new(Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(14, 15)),
							None,
							None,
							Box::new(Symbols::End(16, 19)),
							Box::new(Node::Ident(20, 23, Box::new(Symbols::Ident(20, 23, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_and() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE& Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 23,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (None, Some(Box::new(Symbols::And(9, 10)))) ),
							None,
							Box::new(Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(14, 15)),
							None,
							None,
							Box::new(Symbols::End(16, 19)),
							Box::new(Node::Ident(20, 23, Box::new(Symbols::Ident(20, 23, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_not() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE~ Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 23,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (None, Some(Box::new(Symbols::Not(9, 10)))) ),
							None,
							Box::new(Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(14, 15)),
							None,
							None,
							Box::new(Symbols::End(16, 19)),
							Box::new(Node::Ident(20, 23, Box::new(Symbols::Ident(20, 23, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE- Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 23,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (None, Some(Box::new(Symbols::Minus(9, 10)))) ),
							None,
							Box::new(Node::Ident(11, 14, Box::new(Symbols::Ident(11, 14, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(14, 15)),
							None,
							None,
							Box::new(Symbols::End(16, 19)),
							Box::new(Node::Ident(20, 23, Box::new(Symbols::Ident(20, 23, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{} Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 24,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 12,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), None) ),
							None,
							Box::new(Node::Ident(12, 15, Box::new(Symbols::Ident(12, 15, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(15, 16)),
							None,
							None,
							Box::new(Symbols::End(17, 20)),
							Box::new(Node::Ident(21, 24, Box::new(Symbols::Ident(21, 24, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{}- Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 25,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 11,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), Some(Box::new(Symbols::Minus(11, 12)))) ),
							None,
							Box::new(Node::Ident(13, 16, Box::new(Symbols::Ident(13, 16, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(16, 17)),
							None,
							None,
							Box::new(Symbols::End(18, 21)),
							Box::new(Node::Ident(22, 25, Box::new(Symbols::Ident(22, 25, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags_minus_return_type() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{}- (a: INT32) Run; END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 36,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 11,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), Some(Box::new(Symbols::Minus(11, 12)))) ),
							Some(
								(
									Box::new(Symbols::LeftParen(13, 14)),
									Box::new(Node::ParameterDeclaration(14, 22,
																		None,
																		Box::new([
																			Box::new(Node::Parameter(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("a")))))), None, None))
																		].to_vec()),
																		Box::new([].to_vec()),
																		Box::new(Symbols::Colon(15, 16)),
																		Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 22, Box::new(String::from("INT32"))))))
									)),
									Box::new(Symbols::RightParen(22, 23))
								)
							),
							Box::new(Node::Ident(24, 27, Box::new(Symbols::Ident(24, 27, Box::new(String::from("Run")))))),
							None,
							Box::new(Symbols::SemiColon(27, 28)),
							None,
							None,
							Box::new(Symbols::End(29, 32)),
							Box::new(Node::Ident(33, 36, Box::new(Symbols::Ident(33, 36, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags_minus_return_type_and_formal_parameters() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{}- (a: INT32) Run(); END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 38,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 11,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), Some(Box::new(Symbols::Minus(11, 12)))) ),
							Some(
								(
									Box::new(Symbols::LeftParen(13, 14)),
									Box::new(Node::ParameterDeclaration(14, 22,
																		None,
																		Box::new([
																			Box::new(Node::Parameter(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("a")))))), None, None))
																		].to_vec()),
																		Box::new([].to_vec()),
																		Box::new(Symbols::Colon(15, 16)),
																		Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 22, Box::new(String::from("INT32"))))))
									)),
									Box::new(Symbols::RightParen(22, 23))
								)
							),
							Box::new(Node::Ident(24, 27, Box::new(Symbols::Ident(24, 27, Box::new(String::from("Run")))))),
							Some(
								Box::new(
									Node::FormalParameters(27, 29,
														   Box::new(Symbols::LeftParen(27, 28)),
														   Box::new([].to_vec()),
														   Box::new([].to_vec()),
														   Box::new(Symbols::RightParen(28, 29)),
														   None)
								)
							),
							Box::new(Symbols::SemiColon(29, 30)),
							None,
							None,
							Box::new(Symbols::End(31, 34)),
							Box::new(Node::Ident(35, 38, Box::new(Symbols::Ident(35, 38, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags_minus_return_type_and_formal_parameters_decl() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{}- (a: INT32) Run(); VAR END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 42,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 11,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), Some(Box::new(Symbols::Minus(11, 12)))) ),
							Some(
								(
									Box::new(Symbols::LeftParen(13, 14)),
									Box::new(Node::ParameterDeclaration(14, 22,
																		None,
																		Box::new([
																			Box::new(Node::Parameter(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("a")))))), None, None))
																		].to_vec()),
																		Box::new([].to_vec()),
																		Box::new(Symbols::Colon(15, 16)),
																		Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 22, Box::new(String::from("INT32"))))))
									)),
									Box::new(Symbols::RightParen(22, 23))
								)
							),
							Box::new(Node::Ident(24, 27, Box::new(Symbols::Ident(24, 27, Box::new(String::from("Run")))))),
							Some(
								Box::new(
									Node::FormalParameters(27, 29,
														   Box::new(Symbols::LeftParen(27, 28)),
														   Box::new([].to_vec()),
														   Box::new([].to_vec()),
														   Box::new(Symbols::RightParen(28, 29)),
														   None)
								)
							),
							Box::new(Symbols::SemiColon(29, 30)),
							Some(
								Box::new(
									Node::DeclarationSequence(31, 35,
																Box::new([].to_vec()),
															  	Box::new([].to_vec()),
																Box::new(
																	[
																		Box::new(
																			Node::VarDeclaration(31, 35,
																								 Box::new(Symbols::Var(31, 34)),
																								 Box::new([].to_vec())
																			)
																		)
																	].to_vec()
																),
															  	Box::new([].to_vec()),
															  	Box::new([].to_vec()),
															  	Box::new([].to_vec())
									)
								)
							),
							None,
							Box::new(Symbols::End(35, 38)),
							Box::new(Node::Ident(39, 42, Box::new(Symbols::Ident(39, 42, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_procedure_with_flags_minus_return_type_and_formal_parameters_decl_and_body() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE{}- (a: INT32) Run(); VAR BEGIN test := 1 END Run")));
		parser.advance();
		let res = parser.parse_procedure_declaration();

		let pattern = Box::new(
			Node::Procedure(0, 58,
							Box::new(Symbols::Procedure(0, 9)),
							Some( (Some(Box::new(Node::Flags(9, 11,
															 Box::new(Symbols::LeftBrace(9, 10)),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new(Symbols::RightBrace(10, 11))))), Some(Box::new(Symbols::Minus(11, 12)))) ),
							Some(
								(
									Box::new(Symbols::LeftParen(13, 14)),
									Box::new(Node::ParameterDeclaration(14, 22,
																		None,
																		Box::new([
																			Box::new(Node::Parameter(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("a")))))), None, None))
																		].to_vec()),
																		Box::new([].to_vec()),
																		Box::new(Symbols::Colon(15, 16)),
																		Box::new(Node::Ident(17, 22, Box::new(Symbols::Ident(17, 22, Box::new(String::from("INT32"))))))
									)),
									Box::new(Symbols::RightParen(22, 23))
								)
							),
							Box::new(Node::Ident(24, 27, Box::new(Symbols::Ident(24, 27, Box::new(String::from("Run")))))),
							Some(
								Box::new(
									Node::FormalParameters(27, 29,
														   Box::new(Symbols::LeftParen(27, 28)),
														   Box::new([].to_vec()),
														   Box::new([].to_vec()),
														   Box::new(Symbols::RightParen(28, 29)),
														   None)
								)
							),
							Box::new(Symbols::SemiColon(29, 30)),
							Some(
								Box::new(
									Node::DeclarationSequence(31, 35,
															  Box::new([].to_vec()),
															  Box::new([].to_vec()),
															  Box::new(
																  [
																	  Box::new(
																		  Node::VarDeclaration(31, 35,
																							   Box::new(Symbols::Var(31, 34)),
																							   Box::new([].to_vec())
																		  )
																	  )
																  ].to_vec()
															  ),
															  Box::new([].to_vec()),
															  Box::new([].to_vec()),
															  Box::new([].to_vec())
									)
								)
							),
							Some(
								Box::new(
									Node::Body(35, 51,
												Box::new(Symbols::Begin(35, 40)),
												None,
												Box::new(
													Node::StatementSequence(41, 51,
														Box::new([
															Box::new(Node::BecomesStatement(41, 51,
																	Box::new(Node::Ident(41, 46, Box::new(Symbols::Ident(41, 45, Box::new(String::from("test")))))),
																	Box::new(Symbols::Becomes(46, 48)),
																	Box::new(Node::Integer(49, 51, Box::new(Symbols::Integer(49, 50, Box::new(String::from("1"))))))
															))
														].to_vec()),
														Box::new([].to_vec())
													)
												),
												None
									)
								)
							),
							Box::new(Symbols::End(51, 54)),
							Box::new(Node::Ident(55, 58, Box::new(Symbols::Ident(55, 58, Box::new(String::from("Run"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR \"+\" (a, b: REAL64) : REAL64; END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 45,
						   Box::new(Symbols::Operator(0, 8)),
						   None,
						   None,
						   Box::new(Node::String(9, 13, Box::new(Symbols::String(9, 12, Box::new(String::from("\"+\"")))))),
						   None,
						   Box::new(
								Node::FormalParameters(13, 36,
													   Box::new(Symbols::LeftParen(13, 14)),
													   Box::new([
														   Box::new(Node::ParameterDeclaration(14, 26,
															   None,
															   Box::new([
																   Box::new(
																	   Node::Parameter(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("a")))))), None, None)
																   ),
																   Box::new(
																		Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("b")))))), None, None)
																   ),
															   ].to_vec()),
															   Box::new([
																   Box::new(Symbols::Comma(15, 16))
															   ].to_vec()),
															   Box::new(Symbols::Colon(18, 19)),
															   Box::new(Node::Ident(20, 26, Box::new(Symbols::Ident(20, 26, Box::new(String::from("REAL64"))))))
														   ))
													   ].to_vec()),
													   Box::new([].to_vec()),
													   Box::new(Symbols::RightParen(26, 27)),
													   Some(
														   (
															   Box::new(Symbols::Colon(28, 29)),
															   None,
															   Box::new(Node::Ident(30, 36, Box::new(Symbols::Ident(30, 36, Box::new(String::from("REAL64"))))))
														   )
													   ))),
						   Box::new(Symbols::SemiColon(36, 37)),
						   None,
						   None,
						   Box::new(Symbols::End(38, 41)),
						   Box::new(Node::String(42, 45, Box::new(Symbols::String(42, 45, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {} \"+\" (a, b: REAL64) : REAL64; END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 48,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 12, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   None,
						   Box::new(Node::String(12, 16, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   None,
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   None,
						   None,
						   Box::new(Symbols::End(41, 44)),
						   Box::new(Node::String(45, 48, Box::new(Symbols::String(45, 48, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags_and_minus() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {}-\"+\" (a, b: REAL64) : REAL64; END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 48,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 11, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   Some(Box::new(Symbols::Minus(11, 12))),
						   Box::new(Node::String(12, 16, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   None,
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   None,
						   None,
						   Box::new(Symbols::End(41, 44)),
						   Box::new(Node::String(45, 48, Box::new(Symbols::String(45, 48, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags_and_minus_export_read_write() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {}-\"+\"*(a, b: REAL64) : REAL64; END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 48,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 11, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   Some(Box::new(Symbols::Minus(11, 12))),
						   Box::new(Node::String(12, 15, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   Some(Box::new(Symbols::Times(15, 16))),
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   None,
						   None,
						   Box::new(Symbols::End(41, 44)),
						   Box::new(Node::String(45, 48, Box::new(Symbols::String(45, 48, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags_and_minus_export_read() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {}-\"+\"-(a, b: REAL64) : REAL64; END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 48,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 11, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   Some(Box::new(Symbols::Minus(11, 12))),
						   Box::new(Node::String(12, 15, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   Some(Box::new(Symbols::Minus(15, 16))),
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   None,
						   None,
						   Box::new(Symbols::End(41, 44)),
						   Box::new(Node::String(45, 48, Box::new(Symbols::String(45, 48, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags_and_minus_export_read_decl() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {}-\"+\"-(a, b: REAL64) : REAL64; VAR END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 52,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 11, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   Some(Box::new(Symbols::Minus(11, 12))),
						   Box::new(Node::String(12, 15, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   Some(Box::new(Symbols::Minus(15, 16))),
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   Some(
							   Box::new(
								   Node::DeclarationSequence(41, 45,
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new( [ Box::new(Node::VarDeclaration(41, 45, Box::new(Symbols::Var(41, 44)), Box::new([].to_vec()))) ].to_vec()),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()))
							   )
						   ),
						   None,
						   Box::new(Symbols::End(45, 48)),
						   Box::new(Node::String(49, 52, Box::new(Symbols::String(49, 52, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn simple_operator_with_flags_and_minus_export_read_decl_and_body() {
		let mut parser = Parser::new(Box::new(Scanner::new("OPERATOR {}-\"+\"-(a, b: REAL64) : REAL64; VAR BEGIN test := 1 END \"+\"")));
		parser.advance();
		let res = parser.parse_operator_declaration();

		let pattern = Box::new(
			Node::Operator(0, 68,
						   Box::new(Symbols::Operator(0, 8)),
						   Some(
							   Box::new(Node::Flags(9, 11, Box::new(Symbols::LeftBrace(9, 10)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(10, 11))))
						   ),
						   Some(Box::new(Symbols::Minus(11, 12))),
						   Box::new(Node::String(12, 15, Box::new(Symbols::String(12, 15, Box::new(String::from("\"+\"")))))),
						   Some(Box::new(Symbols::Minus(15, 16))),
						   Box::new(
							   Node::FormalParameters(16, 39,
													  Box::new(Symbols::LeftParen(16, 17)),
													  Box::new([
														  Box::new(Node::ParameterDeclaration(17, 29,
																							  None,
																							  Box::new([
																								  Box::new(
																									  Node::Parameter(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("a")))))), None, None)
																								  ),
																								  Box::new(
																									  Node::Parameter(20, 21, Box::new(Node::Ident(20, 21, Box::new(Symbols::Ident(20, 21, Box::new(String::from("b")))))), None, None)
																								  ),
																							  ].to_vec()),
																							  Box::new([
																								  Box::new(Symbols::Comma(18, 19))
																							  ].to_vec()),
																							  Box::new(Symbols::Colon(21, 22)),
																							  Box::new(Node::Ident(23, 29, Box::new(Symbols::Ident(23, 29, Box::new(String::from("REAL64"))))))
														  ))
													  ].to_vec()),
													  Box::new([].to_vec()),
													  Box::new(Symbols::RightParen(29, 30)),
													  Some(
														  (
															  Box::new(Symbols::Colon(31, 32)),
															  None,
															  Box::new(Node::Ident(33, 39, Box::new(Symbols::Ident(33, 39, Box::new(String::from("REAL64"))))))
														  )
													  ))),
						   Box::new(Symbols::SemiColon(39, 40)),
						   Some(
							   Box::new(
								   Node::DeclarationSequence(41, 45,
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new( [ Box::new(Node::VarDeclaration(41, 45, Box::new(Symbols::Var(41, 44)), Box::new([].to_vec()))) ].to_vec()),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()),
															 Box::new([].to_vec()))
							   )
						   ),
						   Some(
							   Box::new(
								   Node::Body(45, 61,
											  	Box::new(Symbols::Begin(45, 50)),
									   			None,
									   			Box::new(Node::StatementSequence(51, 61,
																Box::new([
																	Box::new(Node::BecomesStatement(51, 61,
																									Box::new(Node::Ident(51, 56, Box::new(Symbols::Ident(51, 55, Box::new(String::from("test")))))),
																									Box::new(Symbols::Becomes(56, 58)),
																									Box::new(Node::Integer(59, 61, Box::new(Symbols::Integer(59, 60, Box::new(String::from("1"))))))))
																].to_vec()),
																Box::new([].to_vec())
												)),
									   			None
								   )
							   )
						   ),
						   Box::new(Symbols::End(61, 64)),
						   Box::new(Node::String(65, 68, Box::new(Symbols::String(65, 68, Box::new(String::from("\"+\""))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn body_with_finally() {
		let mut parser = Parser::new(Box::new(Scanner::new("BEGIN test := 1 FINALLY test := 100")));
		parser.advance();
		let res = parser.parse_body();

		let pattern = Box::new(
			Node::Body(0, 35,
				Box::new(Symbols::Begin(0, 5)),
				None,
				Box::new(Node::StatementSequence(6, 16,
												 Box::new([
													 Box::new(Node::BecomesStatement(6, 16,
																					 Box::new(Node::Ident(6, 11, Box::new(Symbols::Ident(6, 10, Box::new(String::from("test")))))),
																					 Box::new(Symbols::Becomes(11, 13)),
																					 Box::new(Node::Integer(14, 16, Box::new(Symbols::Integer(14, 15, Box::new(String::from("1"))))))))
												 ].to_vec()),
												 Box::new([].to_vec())
				)),
				Some( ( Box::new(Symbols::Finally(16, 23)), Box::new(
					Node::StatementSequence(24, 35,
													 Box::new([
														 Box::new(Node::BecomesStatement(24, 35,
																						 Box::new(Node::Ident(24, 29, Box::new(Symbols::Ident(24, 28, Box::new(String::from("test")))))),
																						 Box::new(Symbols::Becomes(29, 31)),
																						 Box::new(Node::Integer(32, 35, Box::new(Symbols::Integer(32, 35, Box::new(String::from("100"))))))))
													 ].to_vec()),
													 Box::new([].to_vec())
					))
				))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn body_with_finally_and_flag() {
		let mut parser = Parser::new(Box::new(Scanner::new("BEGIN {} test := 1 FINALLY test := 100")));
		parser.advance();
		let res = parser.parse_body();

		let pattern = Box::new(
			Node::Body(0, 38,
					   Box::new(Symbols::Begin(0, 5)),
					   Some( Box::new( Node::Flags(6, 9, Box::new(Symbols::LeftBrace(6, 7)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(7, 8))) ) ),
					   Box::new(Node::StatementSequence(9, 19,
														Box::new([
															Box::new(Node::BecomesStatement(9, 19,
																							Box::new(Node::Ident(9, 14, Box::new(Symbols::Ident(9, 13, Box::new(String::from("test")))))),
																							Box::new(Symbols::Becomes(14, 16)),
																							Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("1"))))))))
														].to_vec()),
														Box::new([].to_vec())
					   )),
					   Some( ( Box::new(Symbols::Finally(19, 26)), Box::new(
						   Node::StatementSequence(27, 38,
												   Box::new([
													   Box::new(Node::BecomesStatement(27, 38,
																					   Box::new(Node::Ident(27, 32, Box::new(Symbols::Ident(27, 31, Box::new(String::from("test")))))),
																					   Box::new(Symbols::Becomes(32, 34)),
																					   Box::new(Node::Integer(35, 38, Box::new(Symbols::Integer(35, 38, Box::new(String::from("100"))))))))
												   ].to_vec()),
												   Box::new([].to_vec())
						   ))
					   ))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn body_with_code() {
		let mut parser = Parser::new(Box::new(Scanner::new("CODE")));
		parser.advance();
		let res = parser.parse_body();

		let pattern = Box::new(
			Node::BodyCode(0, 4,
					   Box::new(Symbols::Code(0, 4)),
					   Box::new(Node::Empty)
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_declaration() {
		let mut parser = Parser::new(Box::new(Scanner::new("test = INT64;")));
		parser.advance();
		let res = parser.parse_type_declaration();

		let pattern = Box::new(
			Node::TypeDeclarationElement(0, 13,
				Box::new(Node::Ident(0, 5, Box::new(Symbols::Ident(0, 4, Box::new(String::from("test")))))),
				Box::new(Symbols::Equal(5, 6)),
				Box::new(Node::Ident(7, 12, Box::new(Symbols::Ident(7, 12, Box::new(String::from("INT64")))))),
				Box::new(Symbols::SemiColon(12, 13))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_array_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("ARRAY OF INT64")));
		parser.advance();
		let res = parser.parse_array_type();

		let pattern = Box::new(
			Node::ArrayType(0, 14,
				Box::new(Symbols::Array(0, 5)),
				None,
				Box::new(Symbols::Of(6, 8)),
				Box::new(Node::Ident(9, 14, Box::new(Symbols::Ident(9, 14, Box::new(String::from("INT64"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_array_single_dim() {
		let mut parser = Parser::new(Box::new(Scanner::new("ARRAY 10 OF INT64")));
		parser.advance();
		let res = parser.parse_array_type();

		let pattern = Box::new(
			Node::ArrayType(0, 17,
							Box::new(Symbols::Array(0, 5)),
							Some(
								(
									Box::new([
										Box::new(Node::Integer(6, 9, Box::new(Symbols::Integer(6, 8, Box::new(String::from("10"))))))
									].to_vec()),
									Box::new([].to_vec())
								)
							),
							Box::new(Symbols::Of(9, 11)),
							Box::new(Node::Ident(12, 17, Box::new(Symbols::Ident(12, 17, Box::new(String::from("INT64"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_array_multi_dim() {
		let mut parser = Parser::new(Box::new(Scanner::new("ARRAY 10, 10 OF INT64")));
		parser.advance();
		let res = parser.parse_array_type();

		let pattern = Box::new(
			Node::ArrayType(0, 21,
							Box::new(Symbols::Array(0, 5)),
							Some(
								(
									Box::new([
										Box::new(Node::Integer(6, 8, Box::new(Symbols::Integer(6, 8, Box::new(String::from("10")))))),
										Box::new(Node::Integer(10, 13, Box::new(Symbols::Integer(10, 12, Box::new(String::from("10"))))))
									].to_vec()),
									Box::new([
										Box::new(Symbols::Comma(8, 9))
									].to_vec())
								)
							),
							Box::new(Symbols::Of(13, 15)),
							Box::new(Node::Ident(16, 21, Box::new(Symbols::Ident(16, 21, Box::new(String::from("INT64"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_math_array_times() {
		let mut parser = Parser::new(Box::new(Scanner::new("ARRAY * OF INT64")));
		parser.advance();
		let res = parser.parse_array_type();

		let pattern = Box::new(
			Node::MathArrayType(0, 16,
							Box::new(Symbols::Array(0, 5)),
							Some(
								(
									Box::new([
										Box::new(Node::MathArraySize(6, 8, None, Some(Box::new(Symbols::Times(6, 7)))))
									].to_vec()),
									Box::new([].to_vec())
								)
							),
							Box::new(Symbols::Of(8, 10)),
							Box::new(Node::Ident(11, 16, Box::new(Symbols::Ident(11, 16, Box::new(String::from("INT64"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_math_array_question_mark() {
		let mut parser = Parser::new(Box::new(Scanner::new("ARRAY ? OF INT64")));
		parser.advance();
		let res = parser.parse_array_type();

		let pattern = Box::new(
			Node::MathArrayType(0, 16,
								Box::new(Symbols::Array(0, 5)),
								Some(
									(
										Box::new([
											Box::new(Node::MathArraySize(6, 8, None, Some(Box::new(Symbols::QuestionMark(6, 7)))))
										].to_vec()),
										Box::new([].to_vec())
									)
								),
								Box::new(Symbols::Of(8, 10)),
								Box::new(Node::Ident(11, 16, Box::new(Symbols::Ident(11, 16, Box::new(String::from("INT64"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 10, Box::new(Symbols::Record(0, 6)), None, None, None, Box::new(Symbols::End(7, 10)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 14, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), None, None, Box::new(Symbols::End(11, 14)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_single() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64 END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 29, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 26,
							Box::new( Node::VarList(11, 18,
								Box::new([
									Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
									Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
									Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
								].to_vec()),
								Box::new([
									Box::new(Symbols::Comma(12, 13)),
									Box::new(Symbols::Comma(15, 16))
								].to_vec())
							)),
							Box::new(Symbols::Colon(18, 19)),
							Box::new(Node::Ident(20, 26, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						)
					].to_vec()),
					Box::new([].to_vec())
				)
			), None, Box::new(Symbols::End(26, 29)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_multiple() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64; d, e, f: REAL64 END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 46, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 25,
									  Box::new( Node::VarList(11, 18,
															  Box::new([
																  Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
																  Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
																  Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(12, 13)),
																  Box::new(Symbols::Comma(15, 16))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(18, 19)),
									  Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						),
						 Box::new(
							 Node::Var(27, 43,
									   Box::new( Node::VarList(27, 34,
															   Box::new([
																   Box::new( Node::VarName(27, 28, Box::new(Node::Ident(27, 28, Box::new(Symbols::Ident(27, 28, Box::new(String::from("d")))))), None, None) ),
																   Box::new( Node::VarName(30, 31, Box::new(Node::Ident(30, 31, Box::new(Symbols::Ident(30, 31, Box::new(String::from("e")))))), None, None) ),
																   Box::new( Node::VarName(33, 34, Box::new(Node::Ident(33, 34, Box::new(Symbols::Ident(33, 34, Box::new(String::from("f")))))), None, None) )
															   ].to_vec()),
															   Box::new([
																   Box::new(Symbols::Comma(28, 29)),
																   Box::new(Symbols::Comma(31, 32))
															   ].to_vec())
									   )),
									   Box::new(Symbols::Colon(34, 35)),
									   Box::new(Node::Ident(36, 43, Box::new(Symbols::Ident(36, 42, Box::new(String::from("REAL64")))))))
						 )
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(25, 26))
					].to_vec())
				)
			), None, Box::new(Symbols::End(43, 46)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_multiple_and_procedure() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64; d, e, f: REAL64 PROCEDURE Test; END Test END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 71, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 25,
									  Box::new( Node::VarList(11, 18,
															  Box::new([
																  Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
																  Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
																  Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(12, 13)),
																  Box::new(Symbols::Comma(15, 16))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(18, 19)),
									  Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						),
						Box::new(
							Node::Var(27, 43,
									  Box::new( Node::VarList(27, 34,
															  Box::new([
																  Box::new( Node::VarName(27, 28, Box::new(Node::Ident(27, 28, Box::new(Symbols::Ident(27, 28, Box::new(String::from("d")))))), None, None) ),
																  Box::new( Node::VarName(30, 31, Box::new(Node::Ident(30, 31, Box::new(Symbols::Ident(30, 31, Box::new(String::from("e")))))), None, None) ),
																  Box::new( Node::VarName(33, 34, Box::new(Node::Ident(33, 34, Box::new(Symbols::Ident(33, 34, Box::new(String::from("f")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(28, 29)),
																  Box::new(Symbols::Comma(31, 32))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(34, 35)),
									  Box::new(Node::Ident(36, 43, Box::new(Symbols::Ident(36, 42, Box::new(String::from("REAL64")))))))
						)
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(25, 26))
					].to_vec())
				)
			), Some(
				(
					Box::new([
						Box::new( Node::Procedure(43, 68,
							Box::new(Symbols::Procedure(43, 52)),
							None,
							None,
							Box::new(Node::Ident(53, 57, Box::new(Symbols::Ident(53, 57, Box::new(String::from("Test")))))),
							None,
							Box::new(Symbols::SemiColon(57, 58)),
							None,
							None,
							Box::new(Symbols::End(59, 62)),
							Box::new(Node::Ident(63, 68, Box::new(Symbols::Ident(63, 67, Box::new(String::from("Test"))))))
						))
					].to_vec()),
					Box::new([].to_vec())
				)
			), Box::new(Symbols::End(68, 71)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_multiple_and_procedure_and_semicolon() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64; d, e, f: REAL64 PROCEDURE Test; END Test; END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 72, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 25,
									  Box::new( Node::VarList(11, 18,
															  Box::new([
																  Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
																  Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
																  Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(12, 13)),
																  Box::new(Symbols::Comma(15, 16))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(18, 19)),
									  Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						),
						Box::new(
							Node::Var(27, 43,
									  Box::new( Node::VarList(27, 34,
															  Box::new([
																  Box::new( Node::VarName(27, 28, Box::new(Node::Ident(27, 28, Box::new(Symbols::Ident(27, 28, Box::new(String::from("d")))))), None, None) ),
																  Box::new( Node::VarName(30, 31, Box::new(Node::Ident(30, 31, Box::new(Symbols::Ident(30, 31, Box::new(String::from("e")))))), None, None) ),
																  Box::new( Node::VarName(33, 34, Box::new(Node::Ident(33, 34, Box::new(Symbols::Ident(33, 34, Box::new(String::from("f")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(28, 29)),
																  Box::new(Symbols::Comma(31, 32))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(34, 35)),
									  Box::new(Node::Ident(36, 43, Box::new(Symbols::Ident(36, 42, Box::new(String::from("REAL64")))))))
						)
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(25, 26))
					].to_vec())
				)
			), Some(
				(
					Box::new([
						Box::new( Node::Procedure(43, 67,
												  Box::new(Symbols::Procedure(43, 52)),
												  None,
												  None,
												  Box::new(Node::Ident(53, 57, Box::new(Symbols::Ident(53, 57, Box::new(String::from("Test")))))),
												  None,
												  Box::new(Symbols::SemiColon(57, 58)),
												  None,
												  None,
												  Box::new(Symbols::End(59, 62)),
												  Box::new(Node::Ident(63, 67, Box::new(Symbols::Ident(63, 67, Box::new(String::from("Test"))))))
						))
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(67, 68))
					].to_vec())
				)
			), Box::new(Symbols::End(69, 72)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_multiple_and_procedure_and_operator() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64; d, e, f: REAL64 PROCEDURE Test; END Test OPERATOR \"+\" (); END \"+\" END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 96, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 25,
									  Box::new( Node::VarList(11, 18,
															  Box::new([
																  Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
																  Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
																  Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(12, 13)),
																  Box::new(Symbols::Comma(15, 16))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(18, 19)),
									  Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						),
						Box::new(
							Node::Var(27, 43,
									  Box::new( Node::VarList(27, 34,
															  Box::new([
																  Box::new( Node::VarName(27, 28, Box::new(Node::Ident(27, 28, Box::new(Symbols::Ident(27, 28, Box::new(String::from("d")))))), None, None) ),
																  Box::new( Node::VarName(30, 31, Box::new(Node::Ident(30, 31, Box::new(Symbols::Ident(30, 31, Box::new(String::from("e")))))), None, None) ),
																  Box::new( Node::VarName(33, 34, Box::new(Node::Ident(33, 34, Box::new(Symbols::Ident(33, 34, Box::new(String::from("f")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(28, 29)),
																  Box::new(Symbols::Comma(31, 32))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(34, 35)),
									  Box::new(Node::Ident(36, 43, Box::new(Symbols::Ident(36, 42, Box::new(String::from("REAL64")))))))
						)
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(25, 26))
					].to_vec())
				)
			), Some(
				(
					Box::new([
						Box::new( Node::Procedure(43, 68,
												  Box::new(Symbols::Procedure(43, 52)),
												  None,
												  None,
												  Box::new(Node::Ident(53, 57, Box::new(Symbols::Ident(53, 57, Box::new(String::from("Test")))))),
												  None,
												  Box::new(Symbols::SemiColon(57, 58)),
												  None,
												  None,
												  Box::new(Symbols::End(59, 62)),
												  Box::new(Node::Ident(63, 68, Box::new(Symbols::Ident(63, 67, Box::new(String::from("Test"))))))
						)),
						Box::new( Node::Operator(68, 93,
								Box::new(Symbols::Operator(68, 76)),
								None,
								None,
								Box::new(Node::String(77, 81, Box::new(Symbols::String(77, 80, Box::new(String::from("\"+\"")))))),
								None,
								Box::new(Node::FormalParameters(81, 83, Box::new(Symbols::LeftParen(81, 82)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightParen(82, 83)), None)),
								Box::new(Symbols::SemiColon(83, 84)),
								None,
								None,
								Box::new(Symbols::End(85, 88)),
								Box::new(Node::String(89, 93, Box::new(Symbols::String(89, 92, Box::new(String::from("\"+\""))))))
						))
					].to_vec()),
					Box::new([].to_vec())
				)
			), Box::new(Symbols::End(93, 96)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_record_inherit_with_variable_multiple_and_procedure_and_semicolon_and_operator_and_semicolon() {
		let mut parser = Parser::new(Box::new(Scanner::new("RECORD (a) a, b, c: INT64; d, e, f: REAL64 PROCEDURE Test; END Test; OPERATOR \"+\" (); END \"+\"; END")));
		parser.advance();
		let res = parser.parse_record_type();

		let pattern = Box::new(
			Node::RecordType(0, 98, Box::new(Symbols::Record(0, 6)), Some(
				(
					Box::new(Symbols::LeftParen(7, 8)),
					Box::new(Node::Ident(8, 9, Box::new(Symbols::Ident(8, 9, Box::new(String::from("a")))))),
					Box::new(Symbols::RightParen(9, 10))
				)
			), Some(
				(
					Box::new([
						Box::new(
							Node::Var(11, 25,
									  Box::new( Node::VarList(11, 18,
															  Box::new([
																  Box::new( Node::VarName(11, 12, Box::new(Node::Ident(11, 12, Box::new(Symbols::Ident(11, 12, Box::new(String::from("a")))))), None, None) ),
																  Box::new( Node::VarName(14, 15, Box::new(Node::Ident(14, 15, Box::new(Symbols::Ident(14, 15, Box::new(String::from("b")))))), None, None) ),
																  Box::new( Node::VarName(17, 18, Box::new(Node::Ident(17, 18, Box::new(Symbols::Ident(17, 18, Box::new(String::from("c")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(12, 13)),
																  Box::new(Symbols::Comma(15, 16))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(18, 19)),
									  Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT64")))))))
						),
						Box::new(
							Node::Var(27, 43,
									  Box::new( Node::VarList(27, 34,
															  Box::new([
																  Box::new( Node::VarName(27, 28, Box::new(Node::Ident(27, 28, Box::new(Symbols::Ident(27, 28, Box::new(String::from("d")))))), None, None) ),
																  Box::new( Node::VarName(30, 31, Box::new(Node::Ident(30, 31, Box::new(Symbols::Ident(30, 31, Box::new(String::from("e")))))), None, None) ),
																  Box::new( Node::VarName(33, 34, Box::new(Node::Ident(33, 34, Box::new(Symbols::Ident(33, 34, Box::new(String::from("f")))))), None, None) )
															  ].to_vec()),
															  Box::new([
																  Box::new(Symbols::Comma(28, 29)),
																  Box::new(Symbols::Comma(31, 32))
															  ].to_vec())
									  )),
									  Box::new(Symbols::Colon(34, 35)),
									  Box::new(Node::Ident(36, 43, Box::new(Symbols::Ident(36, 42, Box::new(String::from("REAL64")))))))
						)
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(25, 26))
					].to_vec())
				)
			), Some(
				(
					Box::new([
						Box::new( Node::Procedure(43, 67,
												  Box::new(Symbols::Procedure(43, 52)),
												  None,
												  None,
												  Box::new(Node::Ident(53, 57, Box::new(Symbols::Ident(53, 57, Box::new(String::from("Test")))))),
												  None,
												  Box::new(Symbols::SemiColon(57, 58)),
												  None,
												  None,
												  Box::new(Symbols::End(59, 62)),
												  Box::new(Node::Ident(63, 67, Box::new(Symbols::Ident(63, 67, Box::new(String::from("Test"))))))
						)),
						Box::new( Node::Operator(69, 93,
												 Box::new(Symbols::Operator(69, 77)),
												 None,
												 None,
												 Box::new(Node::String(78, 82, Box::new(Symbols::String(78, 81, Box::new(String::from("\"+\"")))))),
												 None,
												 Box::new(Node::FormalParameters(82, 84, Box::new(Symbols::LeftParen(82, 83)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightParen(83, 84)), None)),
												 Box::new(Symbols::SemiColon(84, 85)),
												 None,
												 None,
												 Box::new(Symbols::End(86, 89)),
												 Box::new(Node::String(90, 93, Box::new(Symbols::String(90, 93, Box::new(String::from("\"+\""))))))
						))
					].to_vec()),
					Box::new([
						Box::new(Symbols::SemiColon(67, 68)),
						Box::new(Symbols::SemiColon(93, 94))
					].to_vec())
				)
			), Box::new(Symbols::End(95, 98)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_pointer_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("POINTER TO FLOAT64")));
		parser.advance();
		let res = parser.parse_pointer_type();

		let pattern = Box::new(
			Node::PointerType(0, 18, Box::new(Symbols::Pointer(0, 7)), None, Box::new(Symbols::To(8, 10)), Box::new(Node::Ident(11, 18, Box::new(Symbols::Ident(11, 18, Box::new(String::from("FLOAT64")))))))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_pointer_simple_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("POINTER {} TO FLOAT64")));
		parser.advance();
		let res = parser.parse_pointer_type();

		let pattern = Box::new(
			Node::PointerType(0, 21, Box::new(Symbols::Pointer(0, 7)), Some(
				Box::new(Node::Flags(8, 11, Box::new(Symbols::LeftBrace(8, 9)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(9, 10))))
			), Box::new(Symbols::To(11, 13)), Box::new(Node::Ident(14, 21, Box::new(Symbols::Ident(14, 21, Box::new(String::from("FLOAT64")))))))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_procedure_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE")));
		parser.advance();
		let res = parser.parse_procedure_type();

		let pattern = Box::new(
			Node::ProcedureType(0, 9, Box::new(Symbols::Procedure(0, 9)), None, None)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_procedure_simple_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE {}")));
		parser.advance();
		let res = parser.parse_procedure_type();

		let pattern = Box::new(
			Node::ProcedureType(0, 12, Box::new(Symbols::Procedure(0, 9)), Some(
				Box::new(Node::Flags(10, 12, Box::new(Symbols::LeftBrace(10, 11)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(11, 12))))
			), None)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_procedure_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("PROCEDURE {} ()")));
		parser.advance();
		let res = parser.parse_procedure_type();

		let pattern = Box::new(
			Node::ProcedureType(0, 15, Box::new(Symbols::Procedure(0, 9)), Some(
				Box::new(Node::Flags(10, 13, Box::new(Symbols::LeftBrace(10, 11)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(11, 12))))
			), Some(
				Box::new(Node::FormalParameters(13, 15, Box::new(Symbols::LeftParen(13, 14)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightParen(14, 15)), None))
			))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_object_simple_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("OBJECT")));
		parser.advance();
		let res = parser.parse_object_type();

		let pattern = Box::new(
			Node::ObjectTypeEmpty(0, 6, Box::new(Symbols::Object(0, 6)))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_object_simple() {
		let mut parser = Parser::new(Box::new(Scanner::new("OBJECT END")));
		parser.advance();
		let res = parser.parse_object_type();

		let pattern = Box::new(
			Node::ObjectType(0, 10,
							 Box::new(Symbols::Object(0, 6)),
							 None,
							 None,
							 None,
							 None,
							 Box::new(Symbols::End(7, 10)),
							 None
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_object_simple_with_name() {
		let mut parser = Parser::new(Box::new(Scanner::new("OBJECT END name")));
		parser.advance();
		let res = parser.parse_object_type();

		let pattern = Box::new(
			Node::ObjectType(0, 15,
							 Box::new(Symbols::Object(0, 6)),
							 None,
							 None,
							 None,
							 None,
							 Box::new(Symbols::End(7, 10)),
							 Some(Box::new(Node::Ident(11, 15, Box::new(Symbols::Ident(11, 15, Box::new(String::from("name")))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_object_with_flags() {
		let mut parser = Parser::new(Box::new(Scanner::new("OBJECT {} END")));
		parser.advance();
		let res = parser.parse_object_type();

		let pattern = Box::new(
			Node::ObjectType(0, 13,
							 Box::new(Symbols::Object(0, 6)),
							 Some(
								 Box::new(Node::Flags(7, 10, Box::new(Symbols::LeftBrace(7, 8)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(8, 9))))
							 ),
							 None,
							 None,
							 None,
							 Box::new(Symbols::End(10, 13)),
							 None
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_object_with_flags_inherits() {
		let mut parser = Parser::new(Box::new(Scanner::new("OBJECT {} ( a ) END")));
		parser.advance();
		let res = parser.parse_object_type();

		let pattern = Box::new(
			Node::ObjectType(0, 19,
							 Box::new(Symbols::Object(0, 6)),
							 Some(
								 Box::new(Node::Flags(7, 10, Box::new(Symbols::LeftBrace(7, 8)), Box::new([].to_vec()), Box::new([].to_vec()), Box::new(Symbols::RightBrace(8, 9))))
							 ),
							 Some(
								 (
									Box::new(Symbols::LeftParen(10, 11)),
									Box::new(Node::Ident(12, 14, Box::new(Symbols::Ident(12, 13, Box::new(String::from("a")))))),
								 	Box::new(Symbols::RightParen(14, 15))
								 )
							 ),
							 None,
							 None,
							 Box::new(Symbols::End(16, 19)),
							 None
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_enum_simple_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("ENUM empty END")));
		parser.advance();
		let res = parser.parse_enumeration_type();

		let pattern = Box::new(
			Node::EnumerationType(0, 14,
				Box::new(Symbols::Enum(0, 4)),
				None,
				Box::new([
					Box::new(Node::EnumElement(5, 11, Box::new(Node::Ident(5, 11, Box::new(Symbols::Ident(5, 10, Box::new(String::from("empty")))))), None))
				].to_vec()),
				Box::new([].to_vec()),
				Box::new(Symbols::End(11, 14))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_enum_normal_empty() {
		let mut parser = Parser::new(Box::new(Scanner::new("ENUM ( a ) empty END")));
		parser.advance();
		let res = parser.parse_enumeration_type();

		let pattern = Box::new(
			Node::EnumerationType(0, 20,
								  Box::new(Symbols::Enum(0, 4)),
								  Some(
									  (
										  Box::new(Symbols::LeftParen(5, 6)),
										  Box::new(Node::Ident(7, 9, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))),
										  Box::new(Symbols::RightParen(9, 10))
									  )
								  ),
								  Box::new([
									  Box::new(Node::EnumElement(11, 17, Box::new(Node::Ident(11, 17, Box::new(Symbols::Ident(11, 16, Box::new(String::from("empty")))))), None))
								  ].to_vec()),
								  Box::new([].to_vec()),
								  Box::new(Symbols::End(17, 20))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_enum_normal_empty_with_assignment() {
		let mut parser = Parser::new(Box::new(Scanner::new("ENUM ( a ) empty = 1 END")));
		parser.advance();
		let res = parser.parse_enumeration_type();

		let pattern = Box::new(
			Node::EnumerationType(0, 24,
								  Box::new(Symbols::Enum(0, 4)),
								  Some(
									  (
										  Box::new(Symbols::LeftParen(5, 6)),
										  Box::new(Node::Ident(7, 9, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))),
										  Box::new(Symbols::RightParen(9, 10))
									  )
								  ),
								  Box::new([
									  Box::new(Node::EnumElement(11, 21, Box::new(Node::Ident(11, 17, Box::new(Symbols::Ident(11, 16, Box::new(String::from("empty")))))),
									  Some(
										  (
												Box::new(Symbols::Equal(17, 18)),
											  	Box::new(Node::Integer(19, 21, Box::new(Symbols::Integer(19, 20, Box::new(String::from("1"))))))
										  )
									  )))
								  ].to_vec()),
								  Box::new([].to_vec()),
								  Box::new(Symbols::End(21, 24))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_enum_normal_empty_multiple_element() {
		let mut parser = Parser::new(Box::new(Scanner::new("ENUM ( a ) empty = 1, full END")));
		parser.advance();
		let res = parser.parse_enumeration_type();

		let pattern = Box::new(
			Node::EnumerationType(0, 30,
								  Box::new(Symbols::Enum(0, 4)),
								  Some(
									  (
										  Box::new(Symbols::LeftParen(5, 6)),
										  Box::new(Node::Ident(7, 9, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))),
										  Box::new(Symbols::RightParen(9, 10))
									  )
								  ),
								  Box::new([
									  Box::new(Node::EnumElement(11, 20, Box::new(Node::Ident(11, 17, Box::new(Symbols::Ident(11, 16, Box::new(String::from("empty")))))),
																 Some(
																	 (
																		 Box::new(Symbols::Equal(17, 18)),
																		 Box::new(Node::Integer(19, 20, Box::new(Symbols::Integer(19, 20, Box::new(String::from("1"))))))
																	 )
																 ))),
									  Box::new(Node::EnumElement(22, 27, Box::new(Node::Ident(22, 27, Box::new(Symbols::Ident(22, 26, Box::new(String::from("full")))))), None))
								  ].to_vec()),
								  Box::new([
									  Box::new(Symbols::Comma(20, 21))
								  ].to_vec()),
								  Box::new(Symbols::End(27, 30))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn type_enum_normal_empty_multiple_element_and_value() {
		let mut parser = Parser::new(Box::new(Scanner::new("ENUM ( a ) empty = 1, full = 2 END")));
		parser.advance();
		let res = parser.parse_enumeration_type();

		let pattern = Box::new(
			Node::EnumerationType(0, 34,
								  Box::new(Symbols::Enum(0, 4)),
								  Some(
									  (
										  Box::new(Symbols::LeftParen(5, 6)),
										  Box::new(Node::Ident(7, 9, Box::new(Symbols::Ident(7, 8, Box::new(String::from("a")))))),
										  Box::new(Symbols::RightParen(9, 10))
									  )
								  ),
								  Box::new([
									  Box::new(Node::EnumElement(11, 20, Box::new(Node::Ident(11, 17, Box::new(Symbols::Ident(11, 16, Box::new(String::from("empty")))))),
																 Some(
																	 (
																		 Box::new(Symbols::Equal(17, 18)),
																		 Box::new(Node::Integer(19, 20, Box::new(Symbols::Integer(19, 20, Box::new(String::from("1"))))))
																	 )
																 ))),
									  Box::new(Node::EnumElement(22, 31, Box::new(Node::Ident(22, 27, Box::new(Symbols::Ident(22, 26, Box::new(String::from("full")))))),
																 Some(
																	 (
																		 Box::new(Symbols::Equal(27, 28)),
																		 Box::new(Node::Integer(29, 31, Box::new(Symbols::Integer(29, 30, Box::new(String::from("2"))))))
																	 )
																 )))
								  ].to_vec()),
								  Box::new([
									  Box::new(Symbols::Comma(20, 21))
								  ].to_vec()),
								  Box::new(Symbols::End(31, 34))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn const_declaration() {
		let mut parser = Parser::new(Box::new(Scanner::new("test = 1")));
		parser.advance();
		let res = parser.parse_constant_declaration();

		let pattern = Box::new(
			Node::Const(0, 8,
						Box::new(Node::Ident(0, 5, Box::new(Symbols::Ident(0, 4, Box::new(String::from("test")))))),
						Box::new(Symbols::Equal(5, 6)),
						Box::new(Node::Integer(7, 8, Box::new(Symbols::Integer(7, 8, Box::new(String::from("1")))))))
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn var_declaration_extern() {
		let mut parser = Parser::new(Box::new(Scanner::new("IO EXTERN 'InOut' : INT32")));
		parser.advance();
		let res = parser.parse_variable_declaration();

		let pattern = Box::new(
			Node::Var(0, 25,
						Box::new(Node::VarList(0, 18,
							Box::new([
								Box::new( Node::VarName(0, 18,
														Box::new(Node::Ident(0, 3, Box::new(Symbols::Ident(0, 2, Box::new(String::from("IO")))))),
														None,
														Some(
															(
																Box::new(Symbols::Extern(3, 9)),
																Box::new(Node::String(10, 18, Box::new(Symbols::String(10, 17, Box::new(String::from("'InOut'"))))))
															)
														)) )
							].to_vec()),
							Box::new([].to_vec())
						)),
						Box::new(Symbols::Colon(18, 19)),
						Box::new(Node::Ident(20, 25, Box::new(Symbols::Ident(20, 25, Box::new(String::from("INT32"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn var_declaration_assignment() {
		let mut parser = Parser::new(Box::new(Scanner::new("IO := InOut : INT32")));
		parser.advance();
		let res = parser.parse_variable_declaration();

		let pattern = Box::new(
			Node::Var(0, 19,
				Box::new(Node::VarList(0, 12,
					Box::new([
						Box::new(Node::VarName(0, 12,
							Box::new(Node::Ident(0, 3, Box::new(Symbols::Ident(0, 2, Box::new(String::from("IO")))))),
							None,
							Some(
								(
									Box::new(Symbols::Becomes(3, 5)),
									Box::new(Node::Ident(6, 12, Box::new(Symbols::Ident(6, 11, Box::new(String::from("InOut"))))))
								)
							)
						))
					].to_vec()),
					Box::new([].to_vec())
				)),
				Box::new(Symbols::Colon(12, 13)),
				Box::new(Node::Ident(14, 19, Box::new(Symbols::Ident(14, 19, Box::new(String::from("INT32"))))))
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 18,
					Box::new([
						Box::new(Node::ConstDeclaration(0, 18,
							Box::new(Symbols::Const(0, 5)),
							Box::new([
								Box::new(Node::Const(6, 11,
									Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
									Box::new(Symbols::Equal(8, 9)),
									Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
								)),
								Box::new(Node::Const(13, 18,
													 Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
													 Box::new(Symbols::Equal(15, 16)),
													 Box::new(Node::Integer(17, 18, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
								))
							].to_vec())
						))
					].to_vec()),
				  	Box::new([].to_vec()),
				  	Box::new([].to_vec()),
				  	Box::new([].to_vec()),
				  	Box::new([].to_vec()),
				  	Box::new([
						Box::new(Symbols::SemiColon(11, 12))
					].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_twice() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 CONST")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 24,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  )),
										  Box::new(Node::ConstDeclaration(19, 24,
																		  Box::new(Symbols::Const(19, 24)),
																		  Box::new([].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_var() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 40,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 40,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 40,
																			 		Box::new(Node::VarList(23, 31,
																										   Box::new([
																											   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																											   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																											   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																										   ].to_vec()),
																										   Box::new([
																											   Box::new(Symbols::Comma(24, 25)),
																											   Box::new(Symbols::Comma(27, 28))
																										   ].to_vec()))),
																				 	Box::new(Symbols::Colon(31, 32)),
																				 	Box::new(Node::Ident(33, 40, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_var_var() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 44,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 44,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_var_var_type() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR TYPE")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 49,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::TypeDeclaration(45, 49, Box::new(Symbols::Type(45, 49)), Box::new([].to_vec())))
									  ].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 45,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_var_var_type_with_values() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR TYPE a = FLOAT32;")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 62,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::TypeDeclaration(45, 62, Box::new(Symbols::Type(45, 49)), Box::new([
											  Box::new(Node::TypeDeclarationElement(50, 62,
											  		Box::new(Node::Ident(50, 52, Box::new(Symbols::Ident(50, 51, Box::new(String::from("a")))))),
												  	Box::new(Symbols::Equal(52, 53)),
												  	Box::new(Node::Ident(54, 61, Box::new(Symbols::Ident(54, 61, Box::new(String::from("FLOAT32")))))),
												  	Box::new(Symbols::SemiColon(61, 62))
											  ))
										  ].to_vec())))
									  ].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 45,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_const_var_var_type_with_values_twice() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR TYPE a = FLOAT32; g = BOOLEAN;")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 75,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::TypeDeclaration(45, 75, Box::new(Symbols::Type(45, 49)), Box::new([
											  Box::new(Node::TypeDeclarationElement(50, 63,
																					Box::new(Node::Ident(50, 52, Box::new(Symbols::Ident(50, 51, Box::new(String::from("a")))))),
																					Box::new(Symbols::Equal(52, 53)),
																					Box::new(Node::Ident(54, 61, Box::new(Symbols::Ident(54, 61, Box::new(String::from("FLOAT32")))))),
																					Box::new(Symbols::SemiColon(61, 62))
											  )),
											  Box::new(Node::TypeDeclarationElement(63, 75,
																					Box::new(Node::Ident(63, 65, Box::new(Symbols::Ident(63, 64, Box::new(String::from("g")))))),
																					Box::new(Symbols::Equal(65, 66)),
																					Box::new(Node::Ident(67, 74, Box::new(Symbols::Ident(67, 74, Box::new(String::from("BOOLEAN")))))),
																					Box::new(Symbols::SemiColon(74, 75))
											  ))
										  ].to_vec())))
									  ].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 45,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_procedure_with_semicolon() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR TYPE a = FLOAT32; g = BOOLEAN; PROCEDURE Run; END Run;")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 99,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::TypeDeclaration(45, 76, Box::new(Symbols::Type(45, 49)), Box::new([
											  Box::new(Node::TypeDeclarationElement(50, 63,
																					Box::new(Node::Ident(50, 52, Box::new(Symbols::Ident(50, 51, Box::new(String::from("a")))))),
																					Box::new(Symbols::Equal(52, 53)),
																					Box::new(Node::Ident(54, 61, Box::new(Symbols::Ident(54, 61, Box::new(String::from("FLOAT32")))))),
																					Box::new(Symbols::SemiColon(61, 62))
											  )),
											  Box::new(Node::TypeDeclarationElement(63, 76,
																					Box::new(Node::Ident(63, 65, Box::new(Symbols::Ident(63, 64, Box::new(String::from("g")))))),
																					Box::new(Symbols::Equal(65, 66)),
																					Box::new(Node::Ident(67, 74, Box::new(Symbols::Ident(67, 74, Box::new(String::from("BOOLEAN")))))),
																					Box::new(Symbols::SemiColon(74, 75))
											  ))
										  ].to_vec())))
									  ].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 45,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([
										  Box::new(
											  Node::Procedure(76, 98,
											  	Box::new(Symbols::Procedure(76, 85)),
												  None, None,
												  Box::new(Node::Ident(86, 89, Box::new(Symbols::Ident(86, 89, Box::new(String::from("Run")))))),
												  None,
												  Box::new(Symbols::SemiColon(89, 90)),
												  None, None,
												  Box::new(Symbols::End(91, 94)),
												  Box::new(Node::Ident(95, 98, Box::new(Symbols::Ident(95, 98, Box::new(String::from("Run"))))))
											  )
										  )
									  ].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12)),
										  Box::new(Symbols::SemiColon(98, 99))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_operator_with_semicolon() {
		let mut parser = Parser::new(Box::new(Scanner::new("CONST a = 1; b = 2 VAR a, b, c : FLOAT64 VAR TYPE a = FLOAT32; g = BOOLEAN; PROCEDURE Run; END Run; OPERATOR \"+\" (); END \"+\") ")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 124,
									  Box::new([
										  Box::new(Node::ConstDeclaration(0, 19,
																		  Box::new(Symbols::Const(0, 5)),
																		  Box::new([
																			  Box::new(Node::Const(6, 11,
																								   Box::new(Node::Ident(6, 8, Box::new(Symbols::Ident(6, 7, Box::new(String::from("a")))))),
																								   Box::new(Symbols::Equal(8, 9)),
																								   Box::new(Node::Integer(10, 11, Box::new(Symbols::Integer(10, 11, Box::new(String::from("1"))))))
																			  )),
																			  Box::new(Node::Const(13, 19,
																								   Box::new(Node::Ident(13, 15, Box::new(Symbols::Ident(13, 14, Box::new(String::from("b")))))),
																								   Box::new(Symbols::Equal(15, 16)),
																								   Box::new(Node::Integer(17, 19, Box::new(Symbols::Integer(17, 18, Box::new(String::from("2"))))))
																			  ))
																		  ].to_vec())
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::TypeDeclaration(45, 76, Box::new(Symbols::Type(45, 49)), Box::new([
											  Box::new(Node::TypeDeclarationElement(50, 63,
																					Box::new(Node::Ident(50, 52, Box::new(Symbols::Ident(50, 51, Box::new(String::from("a")))))),
																					Box::new(Symbols::Equal(52, 53)),
																					Box::new(Node::Ident(54, 61, Box::new(Symbols::Ident(54, 61, Box::new(String::from("FLOAT32")))))),
																					Box::new(Symbols::SemiColon(61, 62))
											  )),
											  Box::new(Node::TypeDeclarationElement(63, 76,
																					Box::new(Node::Ident(63, 65, Box::new(Symbols::Ident(63, 64, Box::new(String::from("g")))))),
																					Box::new(Symbols::Equal(65, 66)),
																					Box::new(Node::Ident(67, 74, Box::new(Symbols::Ident(67, 74, Box::new(String::from("BOOLEAN")))))),
																					Box::new(Symbols::SemiColon(74, 75))
											  ))
										  ].to_vec())))
									  ].to_vec()),
									  Box::new([
										  Box::new( Node::VarDeclaration(19, 41,
																		 Box::new(Symbols::Var(19, 22)),
																		 Box::new([
																			 Box::new(Node::Var(23, 41,
																								Box::new(Node::VarList(23, 31,
																													   Box::new([
																														   Box::new(Node::VarName(23, 24, Box::new(Node::Ident(23, 24, Box::new(Symbols::Ident(23, 24, Box::new(String::from("a")))))), None, None)),
																														   Box::new(Node::VarName(26, 27, Box::new(Node::Ident(26, 27, Box::new(Symbols::Ident(26, 27, Box::new(String::from("b")))))), None, None)),
																														   Box::new(Node::VarName(29, 31, Box::new(Node::Ident(29, 31, Box::new(Symbols::Ident(29, 30, Box::new(String::from("c")))))), None, None))
																													   ].to_vec()),
																													   Box::new([
																														   Box::new(Symbols::Comma(24, 25)),
																														   Box::new(Symbols::Comma(27, 28))
																													   ].to_vec()))),
																								Box::new(Symbols::Colon(31, 32)),
																								Box::new(Node::Ident(33, 41, Box::new(Symbols::Ident(33, 40, Box::new(String::from("FLOAT64"))))))
																			 ))
																		 ].to_vec())) ),
										  Box::new( Node::VarDeclaration(41, 45,
																		 Box::new(Symbols::Var(41, 44)),
																		 Box::new([].to_vec())) )
									  ].to_vec()),
									  Box::new([
										  Box::new(
											  Node::Procedure(76, 98,
															  Box::new(Symbols::Procedure(76, 85)),
															  None, None,
															  Box::new(Node::Ident(86, 89, Box::new(Symbols::Ident(86, 89, Box::new(String::from("Run")))))),
															  None,
															  Box::new(Symbols::SemiColon(89, 90)),
															  None, None,
															  Box::new(Symbols::End(91, 94)),
															  Box::new(Node::Ident(95, 98, Box::new(Symbols::Ident(95, 98, Box::new(String::from("Run"))))))
											  )
										  )
									  ].to_vec()),
									  Box::new([
										  Box::new(Node::Operator(100, 124,
										  		Box::new(Symbols::Operator(100, 108)),
											  None, None,
											  Box::new(Node::String(109, 113, Box::new(Symbols::String(109, 112, Box::new(String::from("\"+\"")))))),
											  None,
											  Box::new(Node::FormalParameters(113, 115,
											  		Box::new(Symbols::LeftParen(113, 114)),
												  	Box::new([].to_vec()),
												  	Box::new([].to_vec()),
												  	Box::new(Symbols::RightParen(114, 115)),
													None
											  )),
											  Box::new(Symbols::SemiColon(115, 116)),
											  None, None,
											  Box::new(Symbols::End(117, 120)),
											  Box::new(Node::String(121, 124, Box::new(Symbols::String(121, 124, Box::new(String::from("\"+\""))))))
										  ))
									  ].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(11, 12)),
										  Box::new(Symbols::SemiColon(98, 99))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}

	#[test]
	fn declaration_sequence_semicolon() {
		let mut parser = Parser::new(Box::new(Scanner::new(";")));
		parser.advance();
		let res = parser.parse_declaration_sequence();

		let pattern = Box::new(
			Node::DeclarationSequence(0, 1,
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([].to_vec()),
									  Box::new([
										  Box::new(Symbols::SemiColon(0, 1))
									  ].to_vec())
			)
		);

		match res {
			Ok(x) => {
				assert_eq!(pattern, x)
			}, _ => assert!(false)
		}
	}


}