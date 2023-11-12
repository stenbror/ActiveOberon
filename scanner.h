
#pragma once

namespace ActiveOberon::Compiler
{

    enum Symbols 
    {
        EOF,

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

        public:
            ActiveOberonScanner();
            ~ActiveOberonScanner();
            Token get_symbol();

    };

}