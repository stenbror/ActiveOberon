
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Scanner module for lexical analyzing of source files

use std::string;

#[derive(Clone, PartialEq, Debug)]
pub enum Symbols
{
	Empty,
	EndOfFile(u32),

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
	Character(u32, u32, Box<std::string::String>)
}

pub trait ScannerMethods
{
	fn new(text: &'static str) -> Self;
	fn length(&self) -> u32;
	fn get_char(&mut self) -> char;
	fn peek_char(&self) -> char;
	fn get_start_position(&self) -> u32;
	fn get_symbol(&mut self) -> Result<Symbols, Box<std::string::String>>;
	fn peek_symbol(&mut self) -> Result<Symbols, Box<std::string::String>>;
	fn is_reserved_keyword(&self, start : u32, end: u32, keyword: &str) -> Option<Symbols>;
	fn is_string_or_character(&mut self) -> Result<Symbols, Box<std::string::String>>;
	fn slice_assembler_code(&mut self) -> Box<Vec<char>>;
}

pub struct Scanner
{
	buffer: Vec<char>,	/* Sourcecode as a vector of chars */
	start_pos: u32,		/* Start of current analyzed symbol */
	index: u32			/* Position into vector */
}

impl ScannerMethods for Scanner
{
	fn new(text: &'static str) -> Scanner {
		Scanner{
			buffer: text.chars().collect(),
			start_pos: 0,
			index: 0
		}
	}

	fn length(&self) -> u32 {
		self.buffer.len() as u32
	}

	fn get_char(&mut self) -> char {
		match self.buffer.get(self.index as usize) {
			Some(x) => {
				if self.index <= (self.length() - 1) {
					self.index = self.index + 1;
				}
				return x.clone()
			},
			_ => '\0'
		}
	}

	fn peek_char(&self) -> char {
		match self.buffer.get(self.index as usize) {
			Some(x) => {
				return x.clone()
			},
			_ => '\0'
		}
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
			'\0' => {
				return Ok(Symbols::EndOfFile(self.index))
			}
			'\r' => { /* Line shift with either single or two characters */
				let _ = self.get_char();
				return match self.peek_char() {
					'\n' => {
						_ = self.get_char();
						self.get_symbol()
					},
					_ => {
						self.get_symbol()
					}
				}
			},
			'\n' => {	/* Line shift with single character */
				let _ = self.get_char();
				return self.get_symbol();
			},
			'(' => {
				let _ = self.get_char();
				return match self.peek_char() {
					'*' => {	/* We have comments in source code, we also need to handled nested comments */
						_ = self.get_char();
						let mut level = 1;
						while level > 0 && self.peek_char() != '\0' {
							match self.peek_char() {
								'*' => {
									_ = self.get_char();
									if self.peek_char() == ')' {
										level = level - 1;
										_ = self.get_char();
									}
								},
								'(' => {
									_ = self.get_char();
									if self.peek_char() == '*' {
										level = level + 1;
										_ = self.get_char();
									}
								},
								_ => {
									_ = self.get_char();
								}
							}
						}
						if level != 0 {
							return Err(Box::new(format!("Unterminated comment at position: '{}'", self.index)))
						}
						self.get_symbol()
					},
					_ => {
						Ok(Symbols::LeftParen(self.start_pos, self.index))
					}
				}
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
				if self.index < (self.length() - 1) && self.buffer[(self.index + 1) as usize] == '"' {
					let res = self.is_string_or_character();
					return match res {
						Ok(x) => {
							Ok(x)
						},
						Err(e) => {
							Err(e)
						}
					}
				}
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
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
				let mut buffer = std::string::String::new();
				if self.peek_char() == '0' {
					buffer.push(self.get_char());
					match self.peek_char() {
						'x' => {
							buffer.push(self.get_char());
							match self.peek_char() {
								'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
								'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
								'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
									buffer.push(self.get_char());
								},
								_ => return Err(Box::new(format!("Need hex digit in hex integer at position: '{}'", self.index)))
							}
							loop {
								match self.peek_char() {
									'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
									'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
									'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
										buffer.push(self.get_char());
									},
									'`' => {
										buffer.push(self.get_char());
										match self.peek_char() {
											'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
											'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
											'A' | 'B' | 'C' | 'D' | 'E' | 'F' => continue,
											_ =>  return Err(Box::new(format!("Need hex digit in hex integer at position: '{}'", self.index)))
										}
									},
									_ => break
								}
							}
							return Ok(Symbols::Integer(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
						},
						'b' => {
							buffer.push(self.get_char());
							if self.peek_char() != '0' && self.peek_char() != '1' && self.peek_char() != '`' {
								return Err(Box::new(format!("Need '0' or '1' in binary integer at position: '{}'", self.index)))
							}
							loop {
								match self.peek_char() {
									'1' | '0' => {
										buffer.push(self.get_char());
										continue
									},
									'`' => {
										buffer.push(self.get_char());
										if self.peek_char() != '0' && self.peek_char() != '1' {
											return Err(Box::new(format!("Need '0' or '1' in binary integer at position: '{}'", self.index)))
										}
										continue
									},
									_ => break
								}
							}
							if self.peek_char().is_ascii_digit() {
								return Err(Box::new(format!("Found digits not in binary integer at position: '{}'", self.index)))
							}
							return Ok(Symbols::Integer(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
						},
						_ => ()
					}
				}
				let mut found_hex = false;
				if buffer.len() == 0 {
					buffer.push(self.get_char())
				}

				loop {
					match self.peek_char() {
						'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
							buffer.push(self.get_char());
							continue
						},
						'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
						'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
							found_hex = true;
							buffer.push(self.get_char());
							continue
						},
						'`' => {
							buffer.push(self.get_char());
							match self.peek_char() {
								'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
								'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
								'A' | 'B' | 'C' | 'D' | 'E' | 'F' => continue,
								_ =>  return Err(Box::new(format!("Need digit in integer at position: '{}'", self.index)))
							}
						},
						_ => break
					}
				}
				return match self.peek_char() {
					'H' => {
						buffer.push(self.get_char());
						Ok(Symbols::Integer(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
					},
					'X' => {
						buffer.push(self.get_char());
						Ok(Symbols::Character(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
					},
					'.' => {
						buffer.push(self.get_char());
						if !self.peek_char().is_ascii_digit() {
							return Err(Box::new(format!("Need digit(s) after '.' in real number at position: '{}'", self.index)))
						}
						loop {
							match self.peek_char() {
								'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
									buffer.push(self.get_char());
									continue
								},
								_ => break
							}
						}
						match self.peek_char() {
							'D' | 'E' => {
								buffer.push(self.get_char());
								if self.peek_char() == '+' || self.peek_char() == '-' {
									buffer.push(self.get_char());
								}
								if !self.peek_char().is_ascii_digit() {
									return Err(Box::new(format!("Need digit(s) after scale factor in real number at position: '{}'", self.index)))
								}
								loop {
									match self.peek_char() {
										'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
											buffer.push(self.get_char());
											continue
										},
										_ => break
									}
								}
							},
							_ => ()
						}
						Ok(Symbols::Real(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
					},
					_ => {
						if found_hex {
							return Err(Box::new(format!("Need 'H' at end of hex integer at position: '{}'", self.index)))
						}
						Ok(Symbols::Integer(self.start_pos, self.index, Box::new(std::string::String::from(buffer.as_str()))))
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
			},
			'\'' | '"' => {
				let res = self.is_string_or_character();
				return match res {
					Ok(x) => {
						Ok(x)
					},
					Err(e) => {
						Err(e)
					}
				}
			},
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

	/// Check next symbol in source code without advancing position in source code.
	fn peek_symbol(&mut self) -> Result<Symbols, Box<string::String>> {
		let save_start_pos = self.index;
		let symbol = self.get_symbol();
		self.index = save_start_pos;
		symbol
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
			"DEFINITION"	=> Some(Symbols::Definition(start, end)),
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
			"ANY"			=> Some(Symbols::Any(start, end)),
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

	/// Handling string or character
	fn is_string_or_character(&mut self) -> Result<Symbols, Box<std::string::String>> {
		let mut buffer = std::string::String::new();
		let mut single_quote = true;

		match self.peek_char() {
			'\\' => {
				single_quote = true;
				buffer.push(self.get_char()); /* '\' */
				buffer.push(self.get_char()); /* '"' */
				loop {
					match self.peek_char() {
						'\\' => {
							let s1 = self.get_char();
							match self.peek_char() {
								'"' => {
									buffer.push(s1);
									buffer.push(self.get_char());
									break
								},
								'\0' => break,
								_ => {
									buffer.push('\\');
									buffer.push(self.get_char() )
								}
							}
						},
						'\0' => {
							return Err(Box::new(format!("Unterminated string of type '\\\"' at position: '{}'", self.index)))
						}
						_ => {
							buffer.push(self.get_char())
						}
					}
				}
			},
			'"' => {
				single_quote = false;
				buffer.push(self.get_char());
				loop {
					match self.peek_char() {
						'\0' => {
							return Err(Box::new(format!("Unterminated string of type '\"' at position: '{}'", self.index)))
						},
						'"' => {
							buffer.push(self.get_char() );
							break
						},
						_ => {
							buffer.push(self.get_char() )
						}
					}
				}
			},
			_ =>  { /* We have ' */
				buffer.push(self.get_char());
				loop {
					match self.peek_char() {
						'\0' => {
							return Err(Box::new(format!("Unterminated string of type '\'' at position: '{}'", self.index)))
						},
						'\'' => {
							buffer.push(self.get_char() );
							break
						},
						_ => {
							buffer.push(self.get_char() )
						}
					}
				}
			}
		}

		return match buffer.len() {
			3 => {
				match single_quote {
					true => Ok(Symbols::Character(self.start_pos, self.index, Box::new(buffer))),
					false => Ok(Symbols::String(self.start_pos, self.index, Box::new(buffer)))
				}
			},
			_ => {
				Ok(Symbols::String(self.start_pos, self.index, Box::new(buffer)))
			}
		}
	}

	fn slice_assembler_code(&mut self) -> Box<Vec<char>> {
		let start: usize = self.index as usize;
		let max = self.length() as usize;
		let mut cur : usize = start;
		let mut end = start;

		loop {
			if max - 3 > cur {
				match ( self.buffer[cur as usize], self.buffer[(cur + 1) as usize], self.buffer[(cur + 2) as usize] ) {
					( 'E' , 'N', 'D' ) => {
						end = cur - 1;
						break;
					},
					_ => {
						cur += 1;
					}
				}
			}
			else {
				end = self.length() as usize - 1;
				break;
			}
		}

		self.index = cur as u32;

		Box::new(self.buffer[ start ..= end].to_vec())
	}
}

// Unittests for scanner module

#[cfg(test)]
mod tests {
	use crate::scanner::{Scanner, ScannerMethods, Symbols};

	#[test]
	fn reserved_keyword_await() {
		let mut scan = Box::new(Scanner::new("AWAIT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Await(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_begin() {
		let mut scan = Box::new(Scanner::new("BEGIN"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Begin(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_by() {
		let mut scan = Box::new(Scanner::new("BY"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::By(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_const() {
		let mut scan = Box::new(Scanner::new("CONST"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Const(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_case() {
		let mut scan = Box::new(Scanner::new("CASE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Case(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_cell() {
		let mut scan = Box::new(Scanner::new("CELL"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Cell(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_cellnet() {
		let mut scan = Box::new(Scanner::new("CELLNET"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Cellnet(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_code() {
		let mut scan = Box::new(Scanner::new("CODE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Code(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_definition() {
		let mut scan = Box::new(Scanner::new("DEFINITION"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Definition(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 10);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_do() {
		let mut scan = Box::new(Scanner::new("DO"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Do(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_div() {
		let mut scan = Box::new(Scanner::new("DIV"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Div(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_end() {
		let mut scan = Box::new(Scanner::new("END"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::End(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_enum() {
		let mut scan = Box::new(Scanner::new("ENUM"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Enum(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_else() {
		let mut scan = Box::new(Scanner::new("ELSE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Else(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_elsif() {
		let mut scan = Box::new(Scanner::new("ELSIF"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Elsif(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_exit() {
		let mut scan = Box::new(Scanner::new("EXIT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Exit(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_extern() {
		let mut scan = Box::new(Scanner::new("EXTERN"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Extern(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_false() {
		let mut scan = Box::new(Scanner::new("FALSE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::False(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_for() {
		let mut scan = Box::new(Scanner::new("FOR"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::For(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_finally() {
		let mut scan = Box::new(Scanner::new("FINALLY"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Finally(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_if() {
		let mut scan = Box::new(Scanner::new("IF"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::If(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_ignore() {
		let mut scan = Box::new(Scanner::new("IGNORE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Ignore(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_imag() {
		let mut scan = Box::new(Scanner::new("IMAG"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Imag(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_in() {
		let mut scan = Box::new(Scanner::new("IN"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::In(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_is() {
		let mut scan = Box::new(Scanner::new("IS"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Is(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_import() {
		let mut scan = Box::new(Scanner::new("IMPORT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Import(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_loop() {
		let mut scan = Box::new(Scanner::new("LOOP"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Loop(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_module() {
		let mut scan = Box::new(Scanner::new("MODULE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Module(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_mod() {
		let mut scan = Box::new(Scanner::new("MOD"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Mod(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_nil() {
		let mut scan = Box::new(Scanner::new("NIL"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Nil(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_of() {
		let mut scan = Box::new(Scanner::new("OF"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Of(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_or() {
		let mut scan = Box::new(Scanner::new("OR"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Or(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_out() {
		let mut scan = Box::new(Scanner::new("OUT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Out(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_operator() {
		let mut scan = Box::new(Scanner::new("OPERATOR"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Operator(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 8);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_procedure() {
		let mut scan = Box::new(Scanner::new("PROCEDURE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Procedure(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_port() {
		let mut scan = Box::new(Scanner::new("PORT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Port(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_repeat() {
		let mut scan = Box::new(Scanner::new("REPEAT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Repeat(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_return() {
		let mut scan = Box::new(Scanner::new("RETURN"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Return(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_self() {
		let mut scan = Box::new(Scanner::new("SELF"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Self_(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_new() {
		let mut scan = Box::new(Scanner::new("NEW"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::New(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_result() {
		let mut scan = Box::new(Scanner::new("RESULT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Result(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_then() {
		let mut scan = Box::new(Scanner::new("THEN"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Then(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_type() {
		let mut scan = Box::new(Scanner::new("TYPE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Type(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_true() {
		let mut scan = Box::new(Scanner::new("TRUE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::True(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_to() {
		let mut scan = Box::new(Scanner::new("TO"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::To(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_until() {
		let mut scan = Box::new(Scanner::new("UNTIL"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Until(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_var() {
		let mut scan = Box::new(Scanner::new("VAR"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Var(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_while() {
		let mut scan = Box::new(Scanner::new("WHILE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::While(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_with() {
		let mut scan = Box::new(Scanner::new("WITH"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::With(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_any() {
		let mut scan = Box::new(Scanner::new("ANY"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Any(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_array() {
		let mut scan = Box::new(Scanner::new("ARRAY"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Array(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_object() {
		let mut scan = Box::new(Scanner::new("OBJECT"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Object(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_pointer() {
		let mut scan = Box::new(Scanner::new("POINTER"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Pointer(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_address() {
		let mut scan = Box::new(Scanner::new("ADDRESS"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Address(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_size() {
		let mut scan = Box::new(Scanner::new("SIZE"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Size(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_alias() {
		let mut scan = Box::new(Scanner::new("ALIAS"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Alias(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn reserved_keyword_alias_with_lowercase() {
		let mut scan = Box::new(Scanner::new("ALiAS1"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Ident(s, e, text) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*text, std::string::String::from("ALiAS1"));
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn literal_name() {
		let mut scan = Box::new(Scanner::new("zer_o_123"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Ident(s, e, text) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*text, std::string::String::from("zer_o_123"));
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_left_paren() {
		let mut scan = Box::new(Scanner::new("("));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LeftParen(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_left_bracket() {
		let mut scan = Box::new(Scanner::new("["));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LeftBracket(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_left_brace() {
		let mut scan = Box::new(Scanner::new("{"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LeftBrace(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_right_paren() {
		let mut scan = Box::new(Scanner::new(")"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::RightParen(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_right_bracket() {
		let mut scan = Box::new(Scanner::new("]"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::RightBracket(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			}, _ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_right_brace() {
		let mut scan = Box::new(Scanner::new("}"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::RightBrace(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_bar() {
		let mut scan = Box::new(Scanner::new("|"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Bar(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_not_equal() {
		let mut scan = Box::new(Scanner::new("#"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::NotEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_and() {
		let mut scan = Box::new(Scanner::new("&"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::And(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_comma() {
		let mut scan = Box::new(Scanner::new(","));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Comma(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_minus() {
		let mut scan = Box::new(Scanner::new("-"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Minus(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_slash() {
		let mut scan = Box::new(Scanner::new("/"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Slash(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_semicolon() {
		let mut scan = Box::new(Scanner::new(";"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::SemiColon(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_equal() {
		let mut scan = Box::new(Scanner::new("="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Equal(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_arrow() {
		let mut scan = Box::new(Scanner::new("^"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Arrow(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_not() {
		let mut scan = Box::new(Scanner::new("~"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Not(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_back_slash() {
		let mut scan = Box::new(Scanner::new("\\"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::BackSlash(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_transpose() {
		let mut scan = Box::new(Scanner::new("`"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Transpose(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_times() {
		let mut scan = Box::new(Scanner::new("*"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Times(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_times_times() {
		let mut scan = Box::new(Scanner::new("**"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::TimesTimes(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_plus() {
		let mut scan = Box::new(Scanner::new("+"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Plus(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_plus_times() {
		let mut scan = Box::new(Scanner::new("+*"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::PlusTimes(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_colon() {
		let mut scan = Box::new(Scanner::new(":"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Colon(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_becomes() {
		let mut scan = Box::new(Scanner::new(":="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Becomes(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_question_mark() {
		let mut scan = Box::new(Scanner::new("?"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::QuestionMark(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_question_marks() {
		let mut scan = Box::new(Scanner::new("??"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::QuestionMarks(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_explain_mark() {
		let mut scan = Box::new(Scanner::new("!"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::ExclaimMark(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_exclaim_marks() {
		let mut scan = Box::new(Scanner::new("!!"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::ExclaimMarks(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_less() {
		let mut scan = Box::new(Scanner::new("<"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Less(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_less_equal() {
		let mut scan = Box::new(Scanner::new("<="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LessEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_less_less() {
		let mut scan = Box::new(Scanner::new("<<"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LessLess(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_less_less_q() {
		let mut scan = Box::new(Scanner::new("<<?"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::LessLessQ(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_greater() {
		let mut scan = Box::new(Scanner::new(">"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Greater(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_greater_equal() {
		let mut scan = Box::new(Scanner::new(">="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::GreaterEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_greater_greater() {
		let mut scan = Box::new(Scanner::new(">>"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::GreaterGreater(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_greater_greater_q() {
		let mut scan = Box::new(Scanner::new(">>?"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::GreaterGreaterQ(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_period() {
		let mut scan = Box::new(Scanner::new("."));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Period(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_upto() {
		let mut scan = Box::new(Scanner::new(".."));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Upto(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_times() {
		let mut scan = Box::new(Scanner::new(".*"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotTimes(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_slash() {
		let mut scan = Box::new(Scanner::new("./"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotSlash(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_equal() {
		let mut scan = Box::new(Scanner::new(".="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_unequal() {
		let mut scan = Box::new(Scanner::new(".#"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotUnEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_less() {
		let mut scan = Box::new(Scanner::new(".<"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotLess(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_less_equal() {
		let mut scan = Box::new(Scanner::new(".<="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotLessEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_greater() {
		let mut scan = Box::new(Scanner::new(".>"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreater(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn operator_delimiters_dot_greater_equal() {
		let mut scan = Box::new(Scanner::new(".>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn single_comment() {
		let mut scan = Box::new(Scanner::new("(* This is a complete single comment *).>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 39);
						assert_eq!(e, 42);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn multiple_comment() {
		let mut scan = Box::new(Scanner::new("(* This (* is *) a complete single comment *).>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 45);
						assert_eq!(e, 48);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn end_of_file() {
		let mut scan = Box::new(Scanner::new(""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::EndOfFile(s) => {
						assert_eq!(s, 0);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn newline_windows() {
		let mut scan = Box::new(Scanner::new("\r\n.>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 2);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn newline_linux() {
		let mut scan = Box::new(Scanner::new("\n.>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 1);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn newline_mac() {
		let mut scan = Box::new(Scanner::new("\r.>="));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::DotGreaterEqual(s, e) => {
						assert_eq!(s, 1);
						assert_eq!(e, 4);
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_zero() {
		let mut scan = Box::new(Scanner::new("0"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 1);
						assert_eq!(*v, std::string::String::from("0"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn real_zero_dot_zero() {
		let mut scan = Box::new(Scanner::new("0.0"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*v, std::string::String::from("0.0"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_hex_zero() {
		let mut scan = Box::new(Scanner::new("0x0"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*v, std::string::String::from("0x0"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_binary_zero() {
		let mut scan = Box::new(Scanner::new("0b0"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*v, std::string::String::from("0b0"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_without_marks() {
		let mut scan = Box::new(Scanner::new("458765"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("458765"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_with_marks() {
		let mut scan = Box::new(Scanner::new("45`8`76`5"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*v, std::string::String::from("45`8`76`5"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_traditional_hex() {
		let mut scan = Box::new(Scanner::new("0FFH"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*v, std::string::String::from("0FFH"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_traditional_hex_with_marks() {
		let mut scan = Box::new(Scanner::new("0`F`FH"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("0`F`FH"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_traditional_hex_2() {
		let mut scan = Box::new(Scanner::new("8FFH"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*v, std::string::String::from("8FFH"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_binary_with_mark() {
		let mut scan = Box::new(Scanner::new("0b1`01"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("0b1`01"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_hex_with_mark() {
		let mut scan = Box::new(Scanner::new("0x7`Fa"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("0x7`Fa"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn integer_hex_without_mark() {
		let mut scan = Box::new(Scanner::new("0x7AFa"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Integer(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("0x7AFa"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_large() {
		let mut scan = Box::new(Scanner::new("9.03458"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 7);
						assert_eq!(*v, std::string::String::from("9.03458"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_large_with_mark() {
		let mut scan = Box::new(Scanner::new("9`100.010"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*v, std::string::String::from("9`100.010"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_exponent_d() {
		let mut scan = Box::new(Scanner::new("9.03458D4"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*v, std::string::String::from("9.03458D4"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_exponent_e() {
		let mut scan = Box::new(Scanner::new("9.03458E4"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 9);
						assert_eq!(*v, std::string::String::from("9.03458E4"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_exponent_e_minus() {
		let mut scan = Box::new(Scanner::new("9.03458E-4"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 10);
						assert_eq!(*v, std::string::String::from("9.03458E-4"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn float_exponent_e_plus() {
		let mut scan = Box::new(Scanner::new("9.03458E+4"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Real(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 10);
						assert_eq!(*v, std::string::String::from("9.03458E+4"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn character_hex_based() {
		let mut scan = Box::new(Scanner::new("0FF7AX"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Character(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 6);
						assert_eq!(*v, std::string::String::from("0FF7AX"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn character_single_quote() {
		let mut scan = Box::new(Scanner::new("'_'"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::Character(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*v, std::string::String::from("'_'"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn character_double_quote_illegal_it_is_string() {
		let mut scan = Box::new(Scanner::new("\"_\""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 3);
						assert_eq!(*v, std::string::String::from("\"_\""))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_empty_single_quote() {
		let mut scan = Box::new(Scanner::new("''"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
						assert_eq!(*v, std::string::String::from("''"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_empty_double_quote() {
		let mut scan = Box::new(Scanner::new("\"\""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
						assert_eq!(*v, std::string::String::from("\"\""))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_empty_raw_double_quote() {
		let mut scan = Box::new(Scanner::new("\\\"\\\""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 4);
						assert_eq!(*v, std::string::String::from("\\\"\\\""))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_raw_double_quote() {
		let mut scan = Box::new(Scanner::new("\\\"This is \"a 'fake'\" \\r string!\\\""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 33);
						assert_eq!(*v, std::string::String::from("\\\"This is \"a 'fake'\" \\r string!\\\""))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_single_quote() {
		let mut scan = Box::new(Scanner::new("'This is a string!'"));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 19);
						assert_eq!(*v, std::string::String::from("'This is a string!'"))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

	#[test]
	fn string_double_quote() {
		let mut scan = Box::new(Scanner::new("\"This is a string!\""));
		let symbol = scan.get_symbol();
		match symbol {
			Ok(x) => {
				match x {
					Symbols::String(s, e, v) => {
						assert_eq!(s, 0);
						assert_eq!(e, 19);
						assert_eq!(*v, std::string::String::from("\"This is a string!\""))
					},
					_ => assert!(false)
				}
			},
			_ => assert!(false)
		}
	}

}