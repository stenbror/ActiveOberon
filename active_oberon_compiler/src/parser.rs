
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
	Code( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Symbols> ),
	Ignore( u32, u32, Box<Symbols>, Box<Node> ),
	BecomesStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	ExclaimMarkStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	QuestionmarkStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	LessLessStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	GreaterGreaterStatement( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),

	/* Block nodes */
	Module( u32, u32, Box<Symbols>, Option<Box<Node>>, Box<Node>, Option<(Box<Symbols>, Box<Node>)>, Box<Symbols>, Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>, Box<Symbols>, Box<Node>, Box<Symbols> ),
	TemplateParameters( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	TemplateParameter( u32, u32, Box<Symbols>, Box<Node> ),
	ImportList( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	Import( u32, u32, Box<Node>, Option<(Box<Symbols>, Box<Node>)>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<(Box<Symbols>, Box<Node>)> ),
	DeclarationSequence( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	ConstDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	TypeDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	VarDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	ProcedureDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	OperatorDeclaration( u32, u32, Box<Symbols>,  Box<Vec<Box<Node>>> ),
	Const( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	Var( u32, u32, Box<Node>, Box<Symbols>, Box<Node> ),
	VarList( u32, u32, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>> ),
	VarName( u32, u32, Box<Node>, Box<Vec<Box<Node>>>, Option<(Box<Symbols>, Box<Node>)> ),
	ExternVarName( u32, u32, Box<Symbols>, Box<Node> ),
	Flags( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols> ),
	Flag( u32, u32, Box<Node>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Option<(Box<Symbols>, Box<Node>)> ),
	Procedure( u32, u32, Box<Symbols>, Option<(Box<Symbols>, Box<Node>)>, Option<(Box<Symbols>, Box<Node>, Box<Symbols>)>, Box<Node>, Option<Box<Node>>, Box<Symbols>, Box<Node>, Option<Box<Node>>, Box<Symbols>, Box<Node> ),
	Operator( u32, u32, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Box<Symbols>, Box<Node>, Option<Box<Node>>, Box<Symbols>, Box<Node> ),
	FormalParameters( u32, u32, Box<Symbols>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Option<(Box<Symbols>, Box<Node>, Box<Node>)> ),
	ParameterDeclaration( u32, u32, Option<Box<Symbols>>, Box<Vec<Box<Node>>>, Box<Vec<Box<Symbols>>>, Box<Symbols>, Box<Node> ),
	Parameter( u32, u32, Box<Node>, Box<Node>, Option<(Box<Symbols>, Box<Node>)> ),
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
	Port( u32, u32, Box<Node>, Option<Box<Node>> ),
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
					Symbols::End( _ , _ ) => (),
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
					_ => return Err(Box::new(format!("Expecting '|' in case statement at position: '{}'", start_pos)))
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
						let symbol18 = self.symbol.clone()?;
						self.advance();
						Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new(symbol18)) )
					},
					_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'MODULE' at position: '{}'", start_pos)))
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
							_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'IN' at position: '{}'", start_pos)))
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
					_ => return Err(Box::new(format!("Expecting ';' of module after 'MODULE' ident at position: '{}'", start_pos)))
				};

				let imp = match self.symbol.clone()? {
					Symbols::Import( _ , _ ) => Some( self.parse_import_list()? ),
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
					_ => return Err(Box::new(format!("Expecting 'END' in module at position: '{}'", start_pos)))
				};

				let symbol12 = match self.symbol.clone()? {
					Symbols::Ident( s , _ , t ) => {
						module_name_end = *t;
						let symbol21 = self.symbol.clone()?;
						self.advance();
						Box::new( Node::Ident(s, self.lexer.get_start_position(), Box::new( symbol21 )) )
 					},
					_ => return Err(Box::new(format!("Expecting 'Ident' of module after 'END' at position: '{}'", start_pos)))
				};

				let period = match self.symbol.clone()? {
					Symbols::Period( _ , _ ) => {
						let symbol20 = self.symbol.clone()?;
						self.advance();
						Box::new( symbol20 )
					},
					_ => return Err(Box::new(format!("Expecting '.' at end of module at position: '{}'", start_pos)))
				};

				if module_name_start != module_name_end {
					return Err(Box::new(format!("Expecting 'MODULE' name '{}' to be equal to 'END' name '{}' in module declaration at position: '{}'", module_name_start, module_name_end, start_pos)))
				}

				Ok( Box::new( Node::Module(start_pos, self.lexer.get_start_position(), Box::new(symbol1), template, symbol2, in_part, symbol3, imp, decl, body, symbol11, symbol12, period) ) )
			},
			_ => Err(Box::new(format!("Expecting 'MODULE' in module declaration at position: '{}'", start_pos)))
		}
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
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut const_declaration_local = Box::new(Vec::<Box<Node>>::new());
					match self.symbol.clone()? {
						Symbols::Begin( _ , _ ) |
						Symbols::End( _ , _ ) |
						Symbols::Const( _ , _ ) |
						Symbols::Var( _ , _ ) |
						Symbols::Type( _ , _ ) |
						Symbols::Procedure( _ , _ ) |
						Symbols::Operator( _ , _ ) |
						Symbols::SemiColon( _ , _ ) => (),
						_ => {
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
						}
					}
					const_declarations.push( Box::new(Node::ConstDeclaration(start_pos, self.lexer.get_start_position(), Box::new(symbol), const_declaration_local) ) )
				},
				Symbols::Type(_, _) => {
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut type_declaration_local = Box::new(Vec::<Box<Node>>::new());
					match self.symbol.clone()? {
						Symbols::Begin( _ , _ ) |
						Symbols::End( _ , _ ) |
						Symbols::Const( _ , _ ) |
						Symbols::Var( _ , _ ) |
						Symbols::Type( _ , _ ) |
						Symbols::Procedure( _ , _ ) |
						Symbols::Operator( _ , _ ) |
						Symbols::SemiColon( _ , _ ) => (),
						_ => {
							type_declaration_local.push( self.parse_type_declaration()? );
							loop {
								match self.symbol.clone()? {
									Symbols::SemiColon( _ , _ ) => {
										separators.push( Box::new(self.symbol.clone()?) );
										self.advance();
										type_declaration_local.push( self.parse_type_declaration()? )
									},
									_ => break
								}
							}
						}
					}
					type_declarations.push( Box::new(Node::TypeDeclaration(start_pos, self.lexer.get_start_position(), Box::new(symbol), type_declaration_local) ) )
				},
				Symbols::Var(_, _) => {
					let symbol = self.symbol.clone()?;
					self.advance();
					let mut var_declaration_local = Box::new(Vec::<Box<Node>>::new());
					match self.symbol.clone()? {
						Symbols::Begin( _ , _ ) |
						Symbols::End( _ , _ ) |
						Symbols::Const( _ , _ ) |
						Symbols::Var( _ , _ ) |
						Symbols::Type( _ , _ ) |
						Symbols::Procedure( _ , _ ) |
						Symbols::Operator( _ , _ ) |
						Symbols::SemiColon( _ , _ ) => (),
						_ => {
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
						}
					}
					var_declarations.push( Box::new(Node::VarDeclaration(start_pos, self.lexer.get_start_position(), Box::new(symbol), var_declaration_local) ) )
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
			_ => Err(Box::new(format!("Expecting ';' in type declaration at position: '{}'", start_pos)))
		}
	}

	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		let left = self.parse_identifier_definition()?;

		match self.symbol.clone()? {
			Symbols::Equal( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting '=' in type declaration at position: '{}'", start_pos)))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let right = self.parse_type()?;

		match self.symbol.clone()? {
			Symbols::SemiColon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ';' in type declaration at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'ARRAY' in array type at position: '{}'", start_pos)))
		}
		let symbol1 = self.symbol.clone()?;
		self.advance();

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());
		let mut is_math = false;

		match self.symbol.clone()? {
			Symbols::Of( _ , _ ) => (),
			Symbols::Times( _ , _ ) |
			Symbols::QuestionMark( _ , _ ) => {
				is_math = true;
				nodes.push( Box::new(Node::MathArraySize(start_pos, self.lexer.get_start_position(), None, Some(Box::new(self.symbol.clone()?)))) );
				self.advance()
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
			_ => return Err(Box::new(format!("Expecting 'OF' in array type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'RECORD' in record type at position: '{}'", start_pos)))
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
					_ => return Err(Box::new(format!("Expecting ')' in record type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'END' in record type at position: '{}'", start_pos)))
		}
		let symbol2= self.symbol.clone()?;
		self.advance();

		Ok( Box::new(Node::RecordType(start_pos, self.lexer.get_start_position(), Box::new(symbol1), base, el_var, el_op, Box::new(symbol2))) )
	}

	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<String>> {
		let start_pos = self.lexer.get_start_position();

		match self.symbol.clone()? {
			Symbols::Pointer( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'POINTER' in pointer type at position: '{}'", start_pos)))
		}
		let symbol1= self.symbol.clone()?;
		self.advance();

		let flags = match self.symbol.clone()? {
			Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
			_ => None
		};

		match self.symbol.clone()? {
			Symbols::To( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting 'TO' in pointer type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'PROCEDURE' in procedure type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'OBJECT' in object type at position: '{}'", start_pos)))
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
							_ => return Err(Box::new(format!("Expecting ')' in object type at position: '{}'", start_pos)))
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
					_ => return Err(Box::new(format!("Expecting 'END' in object type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'ENUM' in enum type at position: '{}'", start_pos)))
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
					_ => return Err(Box::new(format!("Expecting ')' in enum type at position: '{}'", start_pos)))
				}
				let symbol12= self.symbol.clone()?;
				self.advance();

				Some( (Box::new(symbol11), right, Box::new(symbol12)) )
			},
			_ => None
		};

		let mut nodes = Box::new(Vec::<Box<Node>>::new());
		let mut separators = Box::new(Vec::<Box<Symbols>>::new());
		let mut is_first = true;

		loop {
			let start_pos2 = self.lexer.get_start_position();

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
			_ => return Err(Box::new(format!("Expecting 'END' in enum type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'CELL' or 'CELLNET' in cell or cellnet type at position: '{}'", start_pos)))
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
					_ => return Err(Box::new(format!("Expecting ')' in cell / cellnet type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'END' in cell / cellnet type at position: '{}'", start_pos)))
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
				_ => return Err(Box::new(format!("Expecting 'Ident' literal in port declaration at position: '{}'", start_pos)))
			};

			let flags = match self.symbol.clone()? {
				Symbols::LeftBrace( _ , _ ) => Some( self.parse_flags()? ),
				_ => None
			};

			nodes.push(Box::new( ( ident, flags ) ) )
		}

		match self.symbol.clone()? {
			Symbols::Colon( _ , _ ) => (),
			_ => return Err(Box::new(format!("Expecting ':' in port declaration at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'PORT' in port type at position: '{}'", start_pos)))
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
			_ => return Err(Box::new(format!("Expecting 'IN' or 'OUT' in port type at position: '{}'", start_pos)))
		};

		let first = match self.symbol.clone()? {
			Symbols::LeftParen( _ , _ ) => {
				let symbol11= self.symbol.clone()?;
				self.advance();

				let right = self.parse_expression()?;

				match self.symbol.clone()? {
					Symbols::RightParen( _ , _ ) => (),
					_ => return Err(Box::new(format!("Expecting ')' in port type at position: '{}'", start_pos)))
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
													 Box::new(Vec::<Box<Node>>::new()),
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

}