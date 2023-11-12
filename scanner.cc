
#include <scanner.h>

using namespace ActiveOberon::Compiler;



ActiveOberonScanner::ActiveOberonScanner()
{
    

}

ActiveOberonScanner::~ActiveOberonScanner()
{

}

// Get next symbol in source file /////////////////////////////////////////////////////////////////////////////////////
Token ActiveOberonScanner::get_symbol() {

    while (*p == ' ' || *p == '\t' || *p == '\r' || *p == '\n') p++; /* Eat whitespace first */

    unsigned int start_pos = (unsigned int)(p - pBufferStart);

    switch (*p) {
        case '\0':  return Token { Symbols::EndOfFile, start_pos, start_pos };
        case '(':  p++; return Token { Symbols::LeftParen, start_pos, (unsigned int)(p - pBufferStart) };
        case ')':  p++; return Token { Symbols::RightParen, start_pos, (unsigned int)(p - pBufferStart) };
        case '[':  p++; return Token { Symbols::LeftBracket, start_pos, (unsigned int)(p - pBufferStart) };
        case ']':  p++; return Token { Symbols::RightParen, start_pos, (unsigned int)(p - pBufferStart) };
        case '{':  p++; return Token { Symbols::LeftBrace, start_pos, (unsigned int)(p - pBufferStart) };
        case '}':  p++; return Token { Symbols::RightBrace, start_pos, (unsigned int)(p - pBufferStart) };
        case '|':  p++; return Token { Symbols::Bar, start_pos, (unsigned int)(p - pBufferStart) };
        case '#':  p++; return Token { Symbols::NotEqual, start_pos, (unsigned int)(p - pBufferStart) };
        case '&':  p++; return Token { Symbols::And, start_pos, (unsigned int)(p - pBufferStart) };
        case ',':  p++; return Token { Symbols::Comma, start_pos, (unsigned int)(p - pBufferStart) };
        case '-':  p++; return Token { Symbols::Minus, start_pos, (unsigned int)(p - pBufferStart) };
        case '/':  p++; return Token { Symbols::Slash, start_pos, (unsigned int)(p - pBufferStart) };
        case ';':  p++; return Token { Symbols::SemiColon, start_pos, (unsigned int)(p - pBufferStart) };
        case '=':  p++; return Token { Symbols::Equal, start_pos, (unsigned int)(p - pBufferStart) };
        case '^':  p++; return Token { Symbols::Arrow, start_pos, (unsigned int)(p - pBufferStart) };
        case '~':  p++; return Token { Symbols::Not, start_pos, (unsigned int)(p - pBufferStart) };
        case '\\':  p++; return Token { Symbols::BackSlash, start_pos, (unsigned int)(p - pBufferStart) };
        case '`':  p++; return Token { Symbols::Transpose, start_pos, (unsigned int)(p - pBufferStart) };

        case '*':
            p++;
            if (*p == '*') {
                        p++; return Token { Symbols::TimesTimes, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::Times, start_pos, (unsigned int)(p - pBufferStart) };
         case '+':
            p++;
            if (*p == '*') {
                        p++; return Token { Symbols::PlusTimes, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::Plus, start_pos, (unsigned int)(p - pBufferStart) };
        case ':':
            p++;
            if (*p == '=') {
                        p++; return Token { Symbols::Becomes, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::Colon, start_pos, (unsigned int)(p - pBufferStart) };
        case '<':
            p++;
            switch (*p)
            {
                case '=':   p++; return Token { Symbols::LessEqual, start_pos, (unsigned int)(p - pBufferStart) };
                case '<':
                    p++;
                    if (*p == '?') 
                    {
                        p++;
                        return Token { Symbols::LessLessQ, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::LessLess, start_pos, (unsigned int)(p - pBufferStart) };

                default:    return Token { Symbols::Less, start_pos, (unsigned int)(p - pBufferStart) };
            }
        case '>':
            p++;
            switch (*p)
            {
                case '=':   p++; return Token { Symbols::GreaterEqual, start_pos, (unsigned int)(p - pBufferStart) };
                case '>':
                    p++;
                    if (*p == '?') 
                    {
                        p++;
                        return Token { Symbols::GreaterGreaterQ, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::GreaterGreater, start_pos, (unsigned int)(p - pBufferStart) };

                default:    return Token { Symbols::Greater, start_pos, (unsigned int)(p - pBufferStart) };
            }
        case '?':
            p++;
            if (*p == '?') {
                        p++; return Token { Symbols::QuestionMarks, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::QuestionMark, start_pos, (unsigned int)(p - pBufferStart) };
        case '!':
            p++;
            if (*p == '!') {
                        p++; return Token { Symbols::ExclaimMarks, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::ExclaimMark, start_pos, (unsigned int)(p - pBufferStart) };

        case '.':
            p++;
            switch (*p) {
                case '*':   p++; return Token { Symbols::DotTimes, start_pos, (unsigned int)(p - pBufferStart) };
                case '/':   p++; return Token { Symbols::DotSlash, start_pos, (unsigned int)(p - pBufferStart) };
                case '=':   p++; return Token { Symbols::DotEqual, start_pos, (unsigned int)(p - pBufferStart) };
                case '#':   p++; return Token { Symbols::DotUnEqual, start_pos, (unsigned int)(p - pBufferStart) };
                case '<':   
                    p++;
                    if (*p == '=') {
                        p++; return Token { Symbols::DotGreaterEqual, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::DotGreater, start_pos, (unsigned int)(p - pBufferStart) };
                case '>':
                    p++;
                    if (*p == '=') {
                        p++; return Token { Symbols::DotLessEqual, start_pos, (unsigned int)(p - pBufferStart) };
                    }
                    return Token { Symbols::DotLess, start_pos, (unsigned int)(p - pBufferStart) };
                case '.':   p++; return Token { Symbols::Upto, start_pos, (unsigned int)(p - pBufferStart) };
                default:    return Token { Symbols::Period, start_pos, (unsigned int)(p - pBufferStart) };
            }


        default:    
        
            if (is_ident_start_character(*p)) 
            {
                m_buffer.clear();
                while (is_ident_character(*p)) m_buffer << *p++;

                auto it = m_keywords.find( m_buffer.str() );

                if (it != m_keywords.end())
                {
                    return Token { it->second, start_pos, (unsigned int)(p - pBufferStart) }; /* Reserved keyword found */
                }

                return Token { Symbols::Ident, start_pos, (unsigned int)(p - pBufferStart) };
            }
        
        
            return Token { Symbols::EndOfFile, start_pos, (unsigned int)(p - pBufferStart) };
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