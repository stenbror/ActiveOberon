
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Inline assembler for X86-64 module for compiling and linking of projects written in ActiveOberon language


enum AMD64Symbols {
    None,
    Ident(u32, u32, Box<String>),
    Label(u32, u32, Box<String>),
    Number(u32, u32, Box<String>),
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



pub trait AssemblerAMD64Methods {
    fn new() -> Self;
    fn get_position(&self) -> u32;
    fn get_char(&mut self) -> char;
    fn next_char(&mut self) -> ();
    fn skip_whitespace(&mut self) -> ();
    fn get_ident(&mut self) -> Box<String>;
    fn get_number(&mut self) -> Box<String>;
    fn get_string(&mut self) -> Box<String>;
    fn get_symbol(&mut self) -> Result<Box<AMD64Symbols>, Box<String>>;
}

pub struct AssemblerAMD64 {

}

impl AssemblerAMD64Methods for AssemblerAMD64 {
    fn new() -> Self {
        AssemblerAMD64 {

        }
    }

    fn get_position(&self) -> u32 {
        todo!()
    }

    fn get_char(&mut self) -> char {
        todo!()
    }

    fn next_char(&mut self) -> () {
        todo!()
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
        todo!()
    }

    fn get_number(&mut self) -> Box<String> {
        todo!()
    }

    fn get_string(&mut self) -> Box<String> {
        todo!()
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
                        let label = *symbol + ":";
                        Ok(Box::new(AMD64Symbols::Label(start_pos, self.get_position(), Box::new(label))))
                    },
                    _ => {
                        Ok(Box::new(AMD64Symbols::Ident(start_pos, self.get_position(), symbol)))
                    }
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let symbol = self.get_number();
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
            }
            _ => Err(Box::new(format!("Invalid symbol in inline assembler at position: '{}'", self.get_position())))
        }
    }
}


#[cfg(test)]
mod tests {

}