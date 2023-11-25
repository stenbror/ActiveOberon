
#include "scanner.h"
#include <iostream>

using namespace ActiveOberon::Compiler;



ActiveOberonScanner::ActiveOberonScanner(std::u32string source)
{
    m_source = source;
    m_index = 0;

}

ActiveOberonScanner::~ActiveOberonScanner()
{

}

// Get next symbol in source file /////////////////////////////////////////////////////////////////////////////////////
Token ActiveOberonScanner::get_symbol() {

    while (get_char() == ' ' || get_char() == '\t' || get_char() == '\r' || get_char() == '\n') m_index++;  /* Eat whitespace first */
    
    unsigned int start_pos = m_index;

    switch (get_char()) {
        case '\0':  return Token { Symbols::EndOfFile, start_pos, start_pos };
        case '(':  m_index++; return Token { Symbols::LeftParen, start_pos, m_index };
        case ')':  m_index++; return Token { Symbols::RightParen, start_pos, m_index };
        case '[':  m_index++; return Token { Symbols::LeftBracket, start_pos, m_index };
        case ']':  m_index++; return Token { Symbols::RightBracket, start_pos, m_index };
        case '{':  m_index++; return Token { Symbols::LeftBrace, start_pos, m_index };
        case '}':  m_index++; return Token { Symbols::RightBrace, start_pos, m_index };
        case '|':  m_index++; return Token { Symbols::Bar, start_pos, m_index };
        case '#':  m_index++; return Token { Symbols::NotEqual, start_pos, m_index };
        case '&':  m_index++; return Token { Symbols::And, start_pos, m_index };
        case ',':  m_index++; return Token { Symbols::Comma, start_pos, m_index };
        case '-':  m_index++; return Token { Symbols::Minus, start_pos, m_index };
        case '/':  m_index++; return Token { Symbols::Slash, start_pos, m_index };
        case ';':  m_index++; return Token { Symbols::SemiColon, start_pos, m_index };
        case '=':  m_index++; return Token { Symbols::Equal, start_pos, m_index };
        case '^':  m_index++; return Token { Symbols::Arrow, start_pos, m_index };
        case '~':  m_index++; return Token { Symbols::Not, start_pos, m_index };
        case '\\':  m_index++; return Token { Symbols::BackSlash, start_pos, m_index };
        case '`':  m_index++; return Token { Symbols::Transpose, start_pos, m_index };

        case '*':
            m_index++;
            if (get_char() == '*') {
                        m_index++; return Token { Symbols::TimesTimes, start_pos, m_index };
                    }
                    return Token { Symbols::Times, start_pos, m_index };
         case '+':
            m_index++;
            if (get_char() == '*') {
                        m_index++; return Token { Symbols::PlusTimes, start_pos, m_index };
                    }
                    return Token { Symbols::Plus, start_pos, m_index };
        case ':':
            m_index++;
            if (get_char() == '=') {
                        m_index++; return Token { Symbols::Becomes, start_pos, m_index };
                    }
                    return Token { Symbols::Colon, start_pos, m_index };
        case '<':
            m_index++;
            switch (get_char())
            {
                case '=':   m_index++; return Token { Symbols::LessEqual, start_pos, m_index };
                case '<':
                    m_index++;
                    if (get_char() == '?') 
                    {
                        m_index++;
                        return Token { Symbols::LessLessQ, start_pos, m_index };
                    }
                    return Token { Symbols::LessLess, start_pos, m_index };

                default:    return Token { Symbols::Less, start_pos, m_index };
            }
        case '>':
            m_index++;
            switch (get_char())
            {
                case '=':   m_index++; return Token { Symbols::GreaterEqual, start_pos, m_index };
                case '>':
                    m_index++;
                    if (get_char() == '?') 
                    {
                        m_index++;
                        return Token { Symbols::GreaterGreaterQ, start_pos, m_index };
                    }
                    return Token { Symbols::GreaterGreater, start_pos, m_index };

                default:    return Token { Symbols::Greater, start_pos, m_index };
            }
        case '?':
            m_index++;
            if (get_char() == '?') {
                        m_index++; return Token { Symbols::QuestionMarks, start_pos, m_index };
                    }
                    return Token { Symbols::QuestionMark, start_pos, m_index };
        case '!':
            m_index++;
            if (get_char() == '!') {
                        m_index++; return Token { Symbols::ExclaimMarks, start_pos, m_index };
                    }
                    return Token { Symbols::ExclaimMark, start_pos, m_index };

        case '.':
            m_index++;
            switch (get_char()) {
                case '*':   m_index++; return Token { Symbols::DotTimes, start_pos, m_index };
                case '/':   m_index++; return Token { Symbols::DotSlash, start_pos, m_index };
                case '=':   m_index++; return Token { Symbols::DotEqual, start_pos, m_index };
                case '#':   m_index++; return Token { Symbols::DotUnEqual, start_pos, m_index };
                case '<':   
                    m_index++;
                    if (get_char() == '=') {
                        m_index++; return Token { Symbols::DotLessEqual, start_pos, m_index };
                    }
                    return Token { Symbols::DotLess, start_pos, m_index };
                case '>':
                    m_index++;
                    if (get_char() == '=') {
                        m_index++; return Token { Symbols::DotGreaterEqual, start_pos, m_index };
                    }
                    return Token { Symbols::DotGreater, start_pos, m_index };
                case '.':   m_index++; return Token { Symbols::Upto, start_pos, m_index };
                default:    return Token { Symbols::Period, start_pos, m_index };
            }


        default:    
        
            if (is_ident_start_character(get_char())) 
            {
                m_buffer.clear();
                while (is_ident_character(get_char())) { m_buffer << get_char(); m_index++; }

                auto it = m_keywords.find( m_buffer.str() );

                if (it != m_keywords.end())
                {
                    return Token { it->second, start_pos, m_index }; /* Reserved keyword found */
                }

                return Token { Symbols::Ident, start_pos, m_index };
            }
        
        
            return Token { Symbols::EndOfFile, start_pos, m_index };
    }    
}

bool ActiveOberonScanner::is_ident_start_character(char32_t ch) 
{
    return ch == '_' || (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z'); 
}

bool ActiveOberonScanner::is_ident_character(char32_t ch)
{
    return is_ident_start_character(ch) || (ch >= '0' && ch <= '9');
}

char32_t ActiveOberonScanner::get_char()
{
    return m_index < m_source.length() ? m_source.at(m_index) : U'\0';
}