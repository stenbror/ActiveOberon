
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Inline assembler for X86-64 module for compiling and linking of projects written in ActiveOberon language


enum AMD64Symbols {
    None,
    Ident(u32, u32, Box<String>),
    Label(u32, u32, Box<String>),
    Number(u32, u32, i64),
    String_(u32, u32, Box<String>),
    Period(u32, u32),
    SemiColon(u32, u32),
    Colon(u32, u32),
    Comma(u32, u32),
    Plus(u32, u32),
    Minus(u32, u32),
    Times(u32, u32),
    Div(u32, u32),
    Modulo(u32, u32),
    Negate(u32, u32),
    LeftParen(u32, u32),
    RightParen(u32, u32),
    LeftBracket(u32, u32),
    RightBracket(u32, u32),
    LeftCurly(u32, u32),
    RightCurly(u32, u32),
    At(u32, u32),
    Dollar(u32, u32),
    EndOfFile(u32)
}

enum AMD64Node {
    None
}



pub trait AssemblerAMD64Methods {
    fn new(text: Vec<char>, offset_position: u32) -> Self;
    fn get_position(&self) -> u32;
    fn get_char(&mut self) -> char;
    fn next_char(&mut self) -> ();
    fn skip_whitespace(&mut self) -> ();
    fn get_ident(&mut self) -> Box<String>;
    fn get_number(&mut self) -> Result<i64, Box<String>>;
    fn get_string(&mut self) -> Box<String>;
    fn get_symbol(&mut self) -> Result<Box<AMD64Symbols>, Box<String>>;
    fn factor(&mut self) -> Result<Box<AMD64Node>, Box<String>>;
    fn term(&mut self) -> Result<Box<AMD64Node>, Box<String>>;
    fn expression(&mut self) -> Result<Box<AMD64Node>, Box<String>>;
}

pub struct AssemblerAMD64 {
    buffer: Vec<char>,	/* Sourcecode as a vector of chars */
    start_pos: u32,		/* Start of current analyzed symbol */
    index: u32			/* Position into vector */
}

impl AssemblerAMD64Methods for AssemblerAMD64 {
    fn new(text: Vec<char>, offset_position: u32) -> Self {
        AssemblerAMD64 {
            buffer: text,
            start_pos: offset_position,
            index: 0
        }
    }

    fn get_position(&self) -> u32 {
        self.index
    }

    fn get_char(&mut self) -> char {
        match self.buffer.get(self.index as usize) {
            Some(x) => {
                return x.clone()
            },
            _ => '\0'
        }
    }

    fn next_char(&mut self) -> () {
        if self.index as usize <= self.buffer.len() - 1 {
            self.index += 1;
        }
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            match self.get_char() {
                ' ' | '\t' => {
                    self.next_char();
                    continue
                }, /* Remove whitespace */
                ';' => { /* Remove comments */
                    loop {
                        self.next_char();
                        match self.get_char() {
                            '\r' | '\n' | '\0' => break,
                            _ => {
                                self.next_char()
                            }
                        }
                    }
                    break
                },
                _ => break
            }
        }
    }

    fn get_ident(&mut self) -> Box<String> {
        let mut text = String::new();
        loop {
            match self.get_char() {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                    text.push(self.get_char())
                },
                _ => break
            }
            self.next_char();
        }

        Box::new(text)
    }

    fn get_number(&mut self) -> Result<i64, Box<String>> {
        let mut text = String::new();
        let mut value : i64 = 0;

        loop {
            match self.get_char() {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' ..= 'f' | 'A' ..= 'F' => {
                    text.push(self.get_char());
                    self.next_char();
                },
                _ => break
            }
        }

        match self.get_char() {
            'H' | 'X' => {
                self.next_char();
                for el in text.chars() {
                    match el {
                        '0' => {
                            value = value * 0x10 + 0
                        },
                        '1' => {
                            value = value * 0x10 + 1
                        },
                        '2' => {
                            value = value * 0x10 + 2
                        },
                        '3' => {
                            value = value * 0x10 + 3
                        },
                        '4' => {
                            value = value * 0x10 + 4
                        },
                        '5' => {
                            value = value * 0x10 + 5
                        },
                        '6' => {
                            value = value * 0x10 + 6
                        },
                        '7' => {
                            value = value * 0x10 + 7
                        },
                        '8' => {
                            value = value * 0x10 + 8
                        },
                        '9' => {
                            value = value * 0x10 + 9
                        },
                        'a' | 'A' => {
                            value = value * 0x10 + 10
                        },
                        'b' | 'B' => {
                            value = value * 0x10 + 11
                        },
                        'c' | 'C' => {
                            value = value * 0x10 + 12
                        },
                        'd' | 'D' => {
                            value = value * 0x10 + 13
                        },
                        'e' | 'E' => {
                            value = value * 0x10 + 14
                        },
                        'f' | 'F' => {
                            value = value * 0x10 + 15
                        },
                        _ => ()
                    }
                }
            },
            _ => {

                for el in text.chars() {
                    match el {
                        '0' => {
                            value = value * 10
                        },
                        '1' => {
                            value = value * 10 + 1
                        },
                        '2' => {
                            value = value * 10 + 2
                        },
                        '3' => {
                            value = value * 10 + 3
                        },
                        '4' => {
                            value = value * 10 + 4
                        },
                        '5' => {
                            value = value * 10 + 5
                        },
                        '6' => {
                            value = value * 10 + 6
                        },
                        '7' => {
                            value = value * 10 + 7
                        },
                        '8' => {
                            value = value * 10 + 8
                        },
                        '9' => {
                            value = value * 10 + 9
                        },
                        _ => {
                            return Err(Box::new(format!("Found hex digit in non hex number at position: '{}'", self.get_position())))
                        }
                    }
                }
            }
        }

        Ok(value)
    }

    fn get_string(&mut self) -> Box<String> {
        let mut text = String::new();
        text.push(self.get_char());
        self.next_char();

        loop {
            match self.get_char() {
                '\'' => {
                    text.push(self.get_char());
                    break
                },
                '\0' => break,
                _ => {
                    text.push(self.get_char())
                }
            }
            self.next_char()
        }

        Box::new(text)
    }

    /// Assembler lexer for AMD64 Syntax
    fn get_symbol(&mut self) -> Result<Box<AMD64Symbols>, Box<String>> {
        self.skip_whitespace();
        let mut start_pos = self.get_position();

        return match self.get_char() {
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                let symbol = self.get_ident();
                self.skip_whitespace();
                match self.get_char() {
                    ':' => {
                        self.next_char();
                        Ok(Box::new(AMD64Symbols::Label(start_pos, self.get_position(), symbol)))
                    },
                    _ => {
                        Ok(Box::new(AMD64Symbols::Ident(start_pos, self.get_position(), symbol)))
                    }
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let symbol = self.get_number()?;
                Ok(Box::new(AMD64Symbols::Number(start_pos, self.get_position(), symbol)))
            },
            '\'' => {
                let symbol = self.get_string();
                Ok(Box::new(AMD64Symbols::String_(start_pos, self.get_position(), symbol)))
            },
            '.' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Period(start_pos, self.get_position())))
            },
            ';' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::SemiColon(start_pos, self.get_position())))
            },
            ':' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Colon(start_pos, self.get_position())))
            },
            ',' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Comma(start_pos, self.get_position())))
            },
            '+' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Plus(start_pos, self.get_position())))
            },
            '-' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Minus(start_pos, self.get_position())))
            },
            '*' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Times(start_pos, self.get_position())))
            },
            '/' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Div(start_pos, self.get_position())))
            },
            '%' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Modulo(start_pos, self.get_position())))
            },
            '~' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Negate(start_pos, self.get_position())))
            },
            '(' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::LeftParen(start_pos, self.get_position())))
            },
            ')' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::RightParen(start_pos, self.get_position())))
            },
            '[' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::LeftBracket(start_pos, self.get_position())))
            },
            ']' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::RightBracket(start_pos, self.get_position())))
            },
            '{' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::LeftCurly(start_pos, self.get_position())))
            },
            '}' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::RightCurly(start_pos, self.get_position())))
            },
            '@' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::At(start_pos, self.get_position())))
            },
            '$' => {
                self.next_char();
                Ok(Box::new(AMD64Symbols::Dollar(start_pos, self.get_position())))
            },
            '\0' => {
                Ok(Box::new(AMD64Symbols::EndOfFile(self.get_position())))
            },
            '\r' => {
                self.next_char();
                match self.get_char() {
                    '\n' => {
                        self.next_char()
                    },
                    _ => ()
                }
                self.get_symbol()
            },
            '\n' => {
                self.next_char();
                self.get_symbol()
            },
            _ => Err(Box::new(format!("Invalid symbol in inline assembler at position: '{}'", self.get_position())))
        }
    }

    fn factor(&mut self) -> Result<Box<AMD64Node>, Box<String>> {
        todo!()
    }

    fn term(&mut self) -> Result<Box<AMD64Node>, Box<String>> {
        todo!()
    }

    fn expression(&mut self) -> Result<Box<AMD64Node>, Box<String>> {
        todo!()
    }
}


#[cfg(test)]
mod tests {

    use crate::amd64_assembler::{ AssemblerAMD64, AssemblerAMD64Methods, AMD64Symbols };

    #[test]
    fn test_assembler_amd64_lexer_ident() {
        let source = "test".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Ident( 0, 4, t ) => {
                        assert_eq!(*t, String::from("test"))
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_label() {
        let source = "_test1:".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Label( 0, 7, t ) => {
                        assert_eq!(*t, String::from("_test1"))
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_string() {
        let source = "'Hello, World!'".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::String_( 0, 14, t ) => {
                        assert_eq!(*t, String::from("'Hello, World!'"))
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_number_hex_7fh() {
        let source = "7FH".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Number( 0, 3, t ) => {
                        assert_eq!(t, 0x7f)
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_number_hex_7fabx() {
        let source = "7fabX".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Number( 0, 5, t ) => {
                        assert_eq!(t, 0x7fab)
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_number_1000() {
        let source = "1000".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Number( 0, 4, t ) => {
                        assert_eq!(t, 1000)
                    },
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_plus() {
        let source = "  +  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Plus( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_minus() {
        let source = "  -  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Minus( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_times() {
        let source = "  *  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Times( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_div() {
        let source = "  /  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Div( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_period() {
        let source = "  .  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Period( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_comma() {
        let source = "  ,  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Comma( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_colon() {
        let source = "  :  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::Colon( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_left_paren() {
        let source = "  (  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::LeftParen( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_left_bracket() {
        let source = "  [  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::LeftBracket( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_left_curly() {
        let source = "  {  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::LeftCurly( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_right_paren() {
        let source = "  )  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::RightParen( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_right_bracket() {
        let source = "  ]  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::RightBracket( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_assembler_amd64_lexer_right_curly() {
        let source = "  }  ".chars().collect();
        let mut lexer = AssemblerAMD64::new(source, 0);
        let res = lexer.get_symbol();

        match res {
            Ok( x ) => {
                match *x {
                    AMD64Symbols::RightCurly( 2, 3 ) => assert!(true),
                    _ => { assert!(false) }
                }
            },
            _ => assert!(false)
        }
    }
}