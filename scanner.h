
#pragma once

#include <map>
#include <string>
#include <sstream>

namespace ActiveOberon::Compiler
{
    typedef std::basic_stringstream<char32_t> u32sstream;

    enum Symbols 
    {
        EndOfFile,

        /* Reserved keywords */
        Await,
        Begin,
        By,
        Const,
        Case,
        Cell,
        CellNet,
        Code,
        Definition,
        Do,
        Div,
        End,
        Enum,
        Else,
        Elsif,
        Exit,
        Extern,
        False,
        For,
        Finally,
        If,
        Ignore,
        Imag,
        In,
        Is,
        Import,
        Loop,
        Module,
        Mod,
        Nil,
        Of,
        Or,
        Out,
        Operator,
        Procedure,
        Port,
        Repeat,
        Return,
        Self,
        New,
        Result,
        Then,
        True,
        To,
        Type,
        Until,
        Var,
        While,
        With,

        /* Types */
        Any,
        Array,
        Object,
        Pointer,
        Record,
        Address,
        Size,
        Alias,

        /* Operators */
        NotEqual, /* '#' */
        And, /* '&' */
        LeftParen, /* '(' */
        RightParen, /* ')' */
        Times, /* '*' */
        TimesTimes, /* '**' */
        Plus, /* '+' */
        PlusTimes, /* '+*' */
        Comma, /* ',' */
        Minus, /* '-' */
        Period, /* '.' */
        Upto, /* ''.. */
        DotTimes, /* '.*' */
        DotSlash, /* './' */
        DotEqual, /* ''.= */
        DotUnEqual, /* '.#' */
        DotGreater, /* '.>' */
        DotGreaterEqual, /* '.>=' */
        DotLess, /* '.<' */
        DotLessEqual, /* '.<=' */
        Slash, /* '/' */
        Colon, /* ':' */
        Becomes, /* ':=' */
        SemiColon, /* ';' */
        Less, /* '<' */
        LessEqual, /* '<=' */
        Equal, /* '=' */
        Greater, /* '>' */
        GreaterEqual, /* '>=' */
        LeftBracket, /* '[' */
        RightBracket, /* ']' */
        Arrow, /* '^' */
        LeftBrace, /* '{' */
        Bar, /* '|' */
        RightBrace, /* '}' */
        Not, /* '~' */
        BackSlash, /* '\' */
        Transpose, /* '`' */
        QuestionMark, /* '?' */
        QuestionMarks, /* '??' */
        ExclaimMark, /* '!' */
        ExclaimMarks, /* '!!' */
        LessLess, /* '<<' */
        LessLessQ, /* '<<?' */
        GreaterGreater, /* '>>' */
        GreaterGreaterQ, /* '>>?' */

        /* Literals */
        Ident,
        Integer,
        Real,
        String,
        Character
    };

    struct Token
    {
        Symbols symbol;
        unsigned int start_pos;
        unsigned int end_pos;
    };


    class ActiveOberonScanner
    {
        private:
            char32_t *pBufferStart, *p;
            u32sstream m_buffer;

            const std::map<std::basic_string<char32_t>, Symbols> m_keywords = {
                { U"AWAIT",     Symbols::Await },
                { U"BEGIN",     Symbols::Begin },
                { U"BY",        Symbols::By },
                { U"CONST",     Symbols::Const },
                { U"CASE",      Symbols::Case },
                { U"CELL",      Symbols::Cell },
                { U"CELLNET",   Symbols::CellNet },
                { U"CODE",      Symbols::Code },
                { U"DEFINITION",Symbols::Definition },
                { U"DO",        Symbols::Do },
                { U"DIV",       Symbols::Div },
                { U"END",       Symbols::End },
                { U"ENUM",      Symbols::Enum },
                { U"ELSE",      Symbols::Else },
                { U"ELSIF",     Symbols::Elsif },
                { U"EXIT",      Symbols::Exit },
                { U"EXTERN",    Symbols::Extern },
                { U"FALSE",     Symbols::False },
                { U"FOR",       Symbols::For },
                { U"FINALLY",   Symbols::Finally },
                { U"IF",        Symbols::If },
                { U"IGNORE",    Symbols::Ignore },
                { U"IMAG",      Symbols::Imag },
                { U"IN",        Symbols::In },
                { U"IS",        Symbols::Is },
                { U"IMPORT",    Symbols::Import },
                { U"LOOP",      Symbols::Loop },
                { U"MODULE",    Symbols::Module },
                { U"MOD",       Symbols::Mod },
                { U"NIL",       Symbols::Nil },
                { U"OF",        Symbols::Of },
                { U"OR",        Symbols::Or },
                { U"OUT",       Symbols::Out },
                { U"OPERATOR",  Symbols::Operator },
                { U"PROCEDURE", Symbols::Procedure },
                { U"PORT",      Symbols::Port },
                { U"REPEAT",    Symbols::Repeat },
                { U"RETURN",    Symbols::Return },
                { U"SELF",      Symbols::Self },
                { U"NEW",       Symbols::New },
                { U"RESULT",    Symbols::Result },
                { U"THEN",      Symbols::Then },
                { U"TRUE",      Symbols::True },
                { U"TO",        Symbols::To },
                { U"TYPE",      Symbols::Type },
                { U"UNTIL",     Symbols::Until },
                { U"VAR",       Symbols::Var },
                { U"WHILE",     Symbols::While },
                { U"WITH",      Symbols::With }
            };

        public:
            ActiveOberonScanner();
            ~ActiveOberonScanner();
            Token get_symbol();
            bool is_ident_start_character(char32_t ch);
            bool is_ident_character(char32_t ch);

    };

}