
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Scanner module for lexical analyzing of source files

use std::io::IsTerminal;
use std::string;
use crate::scanner::Symbols::String;

#[derive()]
pub enum Symbols
{
	Empty,
	EnfOfFile(u32),

	Await(u32, u32),
	Begin(u32, u32),
	By(u32, u32),
	Const(u32, u32),
	Case(u32, u32),
	Cell(u32, u32),
	Cellnet(u32, u32),
	Code(u32, u32),
	Definition(u32, u32),
	Do(u32, u32),
	Div(u32, u32),
	End(u32, u32),
	Enum(u32, u32),
	Else(u32, u32),
	Elsif(u32, u32),
	Exit(u32, u32),
	Extern(u32, u32),
	False(u32, u32),
	For(u32, u32),
	Finally(u32, u32),
	If(u32, u32),
	Ignore(u32, u32),
	Imag(u32, u32),
	In(u32, u32),
	Is(u32, u32),
	Import(u32, u32),
	Loop(u32, u32),
	Module(u32, u32),
	Mod(u32, u32),
	Nil(u32, u32),
	Of(u32, u32),
	Or(u32, u32),
	Out(u32, u32),
	Operator(u32, u32),
	Procedure(u32, u32),
	Port(u32, u32),
	Repeat(u32, u32),
	Return(u32, u32),
	Self_(u32, u32),
	New(u32, u32),
	Result(u32, u32),
	Then(u32, u32),
	True(u32, u32),
	To(u32, u32),
	Type(u32, u32),
	Until(u32, u32),
	Var(u32, u32),
	While(u32, u32),
	With(u32, u32),

	/* Types */
	Any(u32, u32),
	Array(u32, u32),
	Object(u32, u32),
	Pointer(u32, u32),
	Record(u32, u32),
	Address(u32, u32),
	Size(u32, u32),
	Alias(u32, u32),

	/* Operators */
	NotEqual(u32, u32), /* '#' */
	And(u32, u32), /* '&' */
	LeftParen(u32, u32), /* '(' */
	RightParen(u32, u32), /* ')' */
	Times(u32, u32), /* '*' */
	TimesTimes(u32, u32), /* '**' */
	Plus(u32, u32), /* '+' */
	PlusTimes(u32, u32), /* '+*' */
	Comma(u32, u32), /* ',' */
	Minus(u32, u32), /* '-' */
	Period(u32, u32), /* '.' */
	Upto(u32, u32), /* ''.. */
	DotTimes(u32, u32), /* '.*' */
	DotSlash(u32, u32), /* './' */
	DotEqual(u32, u32), /* ''.= */
	DotUnEqual(u32, u32), /* '.#' */
	DotGreater(u32, u32), /* '.>' */
	DotGreaterEqual(u32, u32), /* '.>=' */
	DotLess(u32, u32), /* '.<' */
	DotLessEqual(u32, u32), /* '.<=' */
	Slash(u32, u32), /* '/' */
	Colon(u32, u32), /* ':' */
	Becomes(u32, u32), /* ':=' */
	SemiColon(u32, u32), /* ';' */
	Less(u32, u32), /* '<' */
	LessEqual(u32, u32), /* '<=' */
	Equal(u32, u32), /* '=' */
	Greater(u32, u32), /* '>' */
	GreaterEqual(u32, u32), /* '>=' */
	LeftBracket(u32, u32), /* '[' */
	RightBracket(u32, u32), /* ']' */
	Arrow(u32, u32), /* '^' */
	LeftBrace(u32, u32), /* '{' */
	Bar(u32, u32), /* '|' */
	RightBrace(u32, u32), /* '}' */
	Not(u32, u32), /* '~' */
	BackSlash(u32, u32), /* '\' */
	Transpose(u32, u32), /* '`' */
	QuestionMark(u32, u32), /* '?' */
	QuestionMarks(u32, u32), /* '??' */
	ExclaimMark(u32, u32), /* '!' */
	ExclaimMarks(u32, u32), /* '!!' */
	LessLess(u32, u32), /* '<<' */
	LessLessQ(u32, u32), /* '<<?' */
	GreaterGreater(u32, u32), /* '>>' */
	GreaterGreaterQ(u32, u32), /* '>>?' */

	/* Literals */
	Ident(u32, u32, Box<std::string::String>),
	Integer(u32, u32, Box<std::string::String>),
	Real(u32, u32, Box<std::string::String>),
	String(u32, u32, Box<std::string::String>),
	Character(u32, u32, char)
}

pub trait ScannerMethods
{
	fn new() -> Self;
	fn get_char(&self) -> char;
	fn peek_char(&self) -> char;
	fn get_start_position(&self) -> u32;
	fn get_symbol(&mut self) -> Result<Symbols, Box<std::string::String>>;
	fn is_reserved_keyword(&self, start : u32, end: u32, keyword: &str) -> Option<Symbols>;
}

pub struct Scanner
{
	start_pos: u32,
	symbol: Symbols,
	index: u32
}

impl ScannerMethods for Scanner
{
	fn new() -> Scanner {
		Scanner{
			start_pos: 0,
			symbol: Symbols::Empty,
			index: 0
		}
	}

	fn get_char(&self) -> char {
		todo!()
	}

	fn peek_char(&self) -> char {
		todo!()
	}

	fn get_start_position(&self) -> u32 {
		self.start_pos
	}

	/// Get the next valid symbol in source file and return it to the parser
	fn get_symbol(&mut self) -> Result<Symbols, Box<std::string::String>> {

		/* Remove whitespace */
		loop {
			let ch = self.peek_char();
			match ch {
				' '  | '\t' => {
					let _ = self.get_char();
					continue
				},
				_ => {
					break
				}
			}
		}

		self.start_pos = self.index; /* Save start position of current symbol */

		/* Operators or delimiters */
		match self.peek_char() {
			'(' => {
				let _ = self.get_char();
				return Ok(Symbols::LeftParen(self.start_pos, self.index))
			},
			')' => {
				let _ = self.get_char();
				return Ok(Symbols::RightParen(self.start_pos, self.index))
			},
			'[' => {
				let _ = self.get_char();
				return Ok(Symbols::LeftBracket(self.start_pos, self.index))
			},
			']' => {
				let _ = self.get_char();
				return Ok(Symbols::RightBracket(self.start_pos, self.index))
			},
			'{' => {
				let _ = self.get_char();
				return Ok(Symbols::LeftBrace(self.start_pos, self.index))
			},
			'}' => {
				let _ = self.get_char();
				return Ok(Symbols::RightBrace(self.start_pos, self.index))
			},
			'|' => {
				let _ = self.get_char();
				return Ok(Symbols::Bar(self.start_pos, self.index))
			},
			'#' => {
				let _ = self.get_char();
				return Ok(Symbols::NotEqual(self.start_pos, self.index))
			},
			'&' => {
				let _ = self.get_char();
				return Ok(Symbols::And(self.start_pos, self.index))
			},
			',' => {
				let _ = self.get_char();
				return Ok(Symbols::Comma(self.start_pos, self.index))
			},
			'-' => {
				let _ = self.get_char();
				return Ok(Symbols::Minus(self.start_pos, self.index))
			},
			'/' => {
				let _ = self.get_char();
				return Ok(Symbols::Slash(self.start_pos, self.index))
			},
			';' => {
				let _ = self.get_char();
				return Ok(Symbols::SemiColon(self.start_pos, self.index))
			},
			'=' => {
				let _ = self.get_char();
				return Ok(Symbols::Equal(self.start_pos, self.index))
			},
			'^' => {
				let _ = self.get_char();
				return Ok(Symbols::Arrow(self.start_pos, self.index))
			},
			'~' => {
				let _ = self.get_char();
				return Ok(Symbols::Not(self.start_pos, self.index))
			},
			'\\' => {
				let _ = self.get_char();
				return Ok(Symbols::BackSlash(self.start_pos, self.index))
			},
			'`' => {
				let _ = self.get_char();
				return Ok(Symbols::Transpose(self.start_pos, self.index))
			},
			'*' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'*' => {
						_ = self.get_char();
						Ok(Symbols::TimesTimes(self.start_pos, self.index))
					},
					_ => {
						Ok(Symbols::Times(self.start_pos, self.index))
					}
				}
			},
			'+' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'*' => {
						_ = self.get_char();
						Ok(Symbols::PlusTimes(self.start_pos, self.index))
					},
					_ => {
						Ok(Symbols::Plus(self.start_pos, self.index))
					}
				}
			},
			':' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'=' => {
						_ = self.get_char();
						Ok(Symbols::Becomes(self.start_pos, self.index))
					},
					_ => {
						Ok(Symbols::Colon(self.start_pos, self.index))
					}
				}
			},
			'<' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'=' => {
						_ = self.get_char();
						Ok(Symbols::LessEqual(self.start_pos, self.index))
					},
					'<' => {
						_ = self.get_char();
						match self.peek_char() {
							'?' => {
								_ = self.get_char();
								Ok(Symbols::LessLessQ(self.start_pos, self.index))
							},
							_ => {
								Ok(Symbols::LessLess(self.start_pos, self.index))
							}
						}
					},
					_ => {
						Ok(Symbols::Less(self.start_pos, self.index))
					}
				}
			},
			'>' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'=' => {
						_ = self.get_char();
						Ok(Symbols::GreaterEqual(self.start_pos, self.index))
					},
					'>' => {
						_ = self.get_char();
						match self.peek_char() {
							'?' => {
								_ = self.get_char();
								Ok(Symbols::GreaterGreaterQ(self.start_pos, self.index))
							},
							_ => {
								Ok(Symbols::GreaterGreater(self.start_pos, self.index))
							}
						}
					},
					_ => {
						Ok(Symbols::Greater(self.start_pos, self.index))
					}
				}
			},
			'?' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'?' => {
						_ = self.get_char();
						Ok(Symbols::QuestionMarks(self.start_pos, self.index))
					},
					_ => {
						Ok(Symbols::QuestionMark(self.start_pos, self.index))
					}
				}
			},
			'!' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'!' => {
						_ = self.get_char();
						Ok(Symbols::ExclaimMarks(self.start_pos, self.index))
					},
					_ => {
						Ok(Symbols::ExclaimMark(self.start_pos, self.index))
					}
				}
			},
			'.' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'*' => {
						_ = self.get_char();
						Ok(Symbols::DotTimes(self.start_pos, self.index))
					},
					'/' => {
						_ = self.get_char();
						Ok(Symbols::DotSlash(self.start_pos, self.index))
					},
					'=' => {
						_ = self.get_char();
						Ok(Symbols::DotEqual(self.start_pos, self.index))
					},
					'#' => {
						_ = self.get_char();
						Ok(Symbols::DotUnEqual(self.start_pos, self.index))
					},
					'.' => {
						_ = self.get_char();
						Ok(Symbols::Upto(self.start_pos, self.index))
					},
					'<' => {
						_ = self.get_char();
						match self.peek_char() {
							'=' => {
								_ = self.get_char();
								Ok(Symbols::DotLessEqual(self.start_pos, self.index))
							},
							_ => {
								Ok(Symbols::DotLess(self.start_pos, self.index))
							}
						}
					},
					'>' => {
						_ = self.get_char();
						match self.peek_char() {
							'=' => {
								_ = self.get_char();
								Ok(Symbols::DotGreaterEqual(self.start_pos, self.index))
							},
							_ => {
								Ok(Symbols::DotGreater(self.start_pos, self.index))
							}
						}
					},
					_ => {
						Ok(Symbols::Period(self.start_pos, self.index))
					}
				}
			},
			'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'I' | 'L' | 'M' | 'N' | 'O' | 'P' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' => {
				/* Potentially a reserved keyword */
				let mut buffer = std::string::String::new();
				loop {
					let _cur = self.peek_char();
					if _cur.is_alphanumeric() || _cur == '_' {
						buffer.push(self.get_char());
						continue
					}
					break
				}
				let res = self.is_reserved_keyword(self.start_pos, self.index, buffer.as_str());
				return match res {
					Some(x) => {
						Ok(x) /* We found reserved keyword and returns symbol */
					},
					_ => {
						/* We found an indent */
						Ok(Symbols::Ident(self.start_pos, self.index, Box::new(buffer)))
					}
				}
			}
			_ => {
				/* Handling identifiers */
				if self.peek_char().is_alphabetic() {
					let mut buffer = std::string::String::new();
					loop {
						let _cur = self.peek_char();
						if _cur.is_alphanumeric() || _cur == '_' {
							buffer.push(self.get_char());
							continue
						}
						break
					}
					return Ok(Symbols::Ident(self.start_pos, self.index, Box::new(buffer)))
				}
			}
		}

		Err(Box::new(format!("Invalid symbol in source file at position: '{}'", self.index)))
	}

	/// Lookup valid reserved keywords and get symbol if found.
	fn is_reserved_keyword(&self, start: u32, end: u32, keyword: &str) -> Option<Symbols> {
		match keyword {
			"AWAIT" 		=> Some(Symbols::Await(start, end)),
			"BEGIN" 		=> Some(Symbols::Begin(start, end)),
			"BY" 			=> Some(Symbols::By(start, end)),
			"CONST" 		=> Some(Symbols::Const(start, end)),
			"CASE" 			=> Some(Symbols::Case(start, end)),
			"CELL" 			=> Some(Symbols::Cell(start, end)),
			"CELLNET" 		=> Some(Symbols::Cellnet(start, end)),
			"CODE" 			=> Some(Symbols::Code(start, end)),
			"DEFINITION" 	=> Some(Symbols::Definition(start, end)),
			"DO" 			=> Some(Symbols::Do(start, end)),
			"DIV" 			=> Some(Symbols::Div(start, end)),
			"END" 			=> Some(Symbols::End(start, end)),
			"ENUM" 			=> Some(Symbols::Enum(start, end)),
			"ELSE" 			=> Some(Symbols::Else(start, end)),
			"ELSIF" 		=> Some(Symbols::Elsif(start, end)),
			"EXIT" 			=> Some(Symbols::Exit(start, end)),
			"EXTERN" 		=> Some(Symbols::Extern(start, end)),
			"FALSE" 		=> Some(Symbols::False(start, end)),
			"FOR" 			=> Some(Symbols::For(start, end)),
			"FINALLY" 		=> Some(Symbols::Finally(start, end)),
			"IF" 			=> Some(Symbols::If(start, end)),
			"IGNORE" 		=> Some(Symbols::Ignore(start, end)),
			"IMAG" 			=> Some(Symbols::Imag(start, end)),
			"IN" 			=> Some(Symbols::In(start, end)),
			"IS" 			=> Some(Symbols::Is(start, end)),
			"IMPORT"		=> Some(Symbols::Import(start, end)),
			"LOOP"			=> Some(Symbols::Loop(start, end)),
			"MODULE"		=> Some(Symbols::Module(start, end)),
			"MOD"			=> Some(Symbols::Mod(start, end)),
			"NIL"			=> Some(Symbols::Nil(start, end)),
			"OF"			=> Some(Symbols::Of(start, end)),
			"OR"			=> Some(Symbols::Or(start, end)),
			"OUT"			=> Some(Symbols::Out(start, end)),
			"OPERATOR"		=> Some(Symbols::Operator(start, end)),
			"PROCEDURE"		=> Some(Symbols::Procedure(start, end)),
			"PORT"			=> Some(Symbols::Port(start, end)),
			"REPEAT"		=> Some(Symbols::Repeat(start, end)),
			"RETURN"		=> Some(Symbols::Return(start, end)),
			"SELF"			=> Some(Symbols::Self_(start, end)),
			"NEW"			=> Some(Symbols::New(start, end)),
			"RESULT"		=> Some(Symbols::Result(start, end)),
			"THEN"			=> Some(Symbols::Then(start, end)),
			"TRUE"			=> Some(Symbols::True(start, end)),
			"TO"			=> Some(Symbols::To(start, end)),
			"TYPE"			=> Some(Symbols::Type(start, end)),
			"UNTIL"			=> Some(Symbols::Until(start, end)),
			"VAR"			=> Some(Symbols::Var(start, end)),
			"WHILE"			=> Some(Symbols::While(start, end)),
			"WITH"			=> Some(Symbols::With(start, end)),
			"ANY"			=> Some(Symbols::And(start, end)),
			"ARRAY"			=> Some(Symbols::Array(start, end)),
			"OBJECT"		=> Some(Symbols::Object(start, end)),
			"POINTER"		=> Some(Symbols::Pointer(start, end)),
			"RECORD"		=> Some(Symbols::Record(start, end)),
			"ADDRESS"		=> Some(Symbols::Address(start, end)),
			"SIZE"			=> Some(Symbols::Size(start, end)),
			"ALIAS"			=> Some(Symbols::Alias(start, end)),
			_ => None
		}
	}
}

// Unittests for scanner module

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}