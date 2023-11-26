use std::io::IsTerminal;

#[derive()]
enum Symbols
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
	Ident(u32, u32, Box<str>),
	Integer(u32, u32, Box<str>),
	Real(u32, u32, Box<str>),
	String(u32, u32, Box<str>),
	Character(u32, u32, char)
}

pub trait ScannerMethods
{
	fn new() -> Self;
	fn get_symbol(&self) -> Symbols;
	fn is_reserved_keyword(&self, start : u32, end: u32, keyword: &str) -> Option<Symbols>;
}

pub struct Scanner
{

}

impl ScannerMethods for Scanner
{
	fn new() -> Scanner {
		Scanner{

		}
	}

	fn get_symbol(&self) -> Symbols {
		todo!()
	}

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
			_ => None
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}