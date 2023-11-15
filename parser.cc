
#include <parser.h>
#include <ast/ast_relation_node.h>
#include <ast/ast_range_node.h>
#include <ast/ast_add_op_node.h>
#include <ast/ast_mul_op_node.h>
#include <ast/ast_unary_op_node.h>
#include <ast/ast_unary_expression_node.h>
#include <ast/ast_literal_node.h>
#include <ast/ast_atom_literal_node.h>

using namespace ActiveOberon::Compiler;

ActiveOberonParser::ActiveOberonParser(std::shared_ptr<ActiveOberonScanner> scanner)
{
    m_lexer = scanner;
    m_curSymbol = m_lexer->get_symbol();
}

ActiveOberonParser::~ActiveOberonParser() 
{

}

std::shared_ptr<Node> ActiveOberonParser::parse_module()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_flags()
{
    return nullptr;
}




// Expression rules ///////////////////////////////////////////////////////////////////////////////////////////////////

std::shared_ptr<Node> ActiveOberonParser::parse_expression()
{
    auto start_pos = m_curSymbol.start_pos;
    auto left = parse_range_expression();

    switch (m_curSymbol.symbol)
    {
        case Symbols::Equal:
        case Symbols::NotEqual:
        case Symbols::Less:
        case Symbols::LessEqual:
        case Symbols::Greater:
        case Symbols::GreaterEqual:
        case Symbols::In:
        case Symbols::Is:
        case Symbols::DotEqual:
        case Symbols::DotUnEqual:
        case Symbols::DotLess:
        case Symbols::DotLessEqual:
        case Symbols::DotGreater:
        case Symbols::DotGreaterEqual:
        case Symbols::QuestionMarks:
        case Symbols::ExclaimMarks:
        case Symbols::LessLessQ:
        case Symbols::GreaterGreaterQ:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();
                auto right = parse_range_expression();
                return std::make_shared<RelationNode>(start_pos, m_curSymbol.start_pos, left, symbol1, right);
            }
        default:    return left;
    }
}

std::shared_ptr<Node> ActiveOberonParser::parse_range_expression()
{
    auto start_pos = m_curSymbol.start_pos;
    
    switch (m_curSymbol.symbol) 
    {
        case Symbols::Times:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();
                return std::make_shared<RangeNode>(start_pos, m_curSymbol.start_pos, nullptr, Token { Symbols::Empty, 0, 0 }, nullptr, Token { Symbols::Empty, 0, 0 }, nullptr, symbol1);
            }
        default:
            {
                auto left = m_curSymbol.symbol != Symbols::Upto ? parse_SimpleExpression() : nullptr;
                if (m_curSymbol.symbol == Symbols::Upto)
                {
                    auto symbol1 = m_curSymbol;
                    m_lexer->get_symbol();
                    std::shared_ptr<Node> right;

                    switch (m_curSymbol.symbol)
                    {
                        case Symbols::Equal:
                        case Symbols::NotEqual:
                        case Symbols::Less:
                        case Symbols::LessEqual:
                        case Symbols::Greater:
                        case Symbols::GreaterEqual:
                        case Symbols::In:
                        case Symbols::Is:
                        case Symbols::DotEqual:
                        case Symbols::DotUnEqual:
                        case Symbols::DotLess:
                        case Symbols::DotLessEqual:
                        case Symbols::DotGreater:
                        case Symbols::DotGreaterEqual:
                        case Symbols::QuestionMarks:
                        case Symbols::ExclaimMarks:
                        case Symbols::LessLessQ:
                        case Symbols::GreaterGreaterQ:
                        case Symbols::SemiColon:
                        case Symbols::Begin:
                        case Symbols::Do:
                            right = nullptr;
                            break;
                        default:
                            right = parse_SimpleExpression();
                    }

                    if (m_curSymbol.symbol == Symbols::By)
                    {
                        auto symbol2 = m_curSymbol;
                        m_curSymbol = m_lexer->get_symbol();
                        auto by = parse_SimpleExpression();

                        return std::make_shared<RangeNode>(start_pos, m_curSymbol.start_pos, left, symbol1, right, symbol2, by, Token { Symbols::Empty, 0, 0 });
                    }

                    return std::make_shared<RangeNode>(start_pos, m_curSymbol.start_pos, left, symbol1, right, Token { Symbols::Empty, 0, 0 }, nullptr, Token { Symbols::Empty, 0, 0 });
                }

                return left;
            }
    }
}

std::shared_ptr<Node> ActiveOberonParser::parse_SimpleExpression()
{
    auto start_pos = m_curSymbol.start_pos;
    auto left = parse_term();

    switch (m_curSymbol.symbol)
    {
        case Symbols::Plus:
        case Symbols::Minus:
        case Symbols::Or:
            {
                while (m_curSymbol.symbol == Symbols::Plus || m_curSymbol.symbol == Symbols::Minus || m_curSymbol.symbol == Symbols::Or)
                {
                    auto symbol1 = m_curSymbol;
                    m_curSymbol = m_lexer->get_symbol();
                    auto right = parse_term();

                    left = std::make_shared<AddOpNode>(start_pos, m_curSymbol.start_pos, left, symbol1, right);
                }

                return left;
            }
        default:    return left;
    }
}

std::shared_ptr<Node> ActiveOberonParser::parse_term()
{
    auto start_pos = m_curSymbol.start_pos;
    auto left = parse_factor();

    switch (m_curSymbol.symbol)
    {
        case Symbols::Times:
        case Symbols::Slash:
        case Symbols::Div:
        case Symbols::Mod:
        case Symbols::And:
        case Symbols::DotTimes:
        case Symbols::DotSlash:
        case Symbols::BackSlash:
        case Symbols::TimesTimes:
        case Symbols::PlusTimes:
            {
                while (m_curSymbol.symbol == Symbols::Times | m_curSymbol.symbol == Symbols::Slash | m_curSymbol.symbol == Symbols::Div | m_curSymbol.symbol == Symbols::Mod | m_curSymbol.symbol == Symbols::And | m_curSymbol.symbol == Symbols::DotTimes
                    | m_curSymbol.symbol == Symbols::Slash | m_curSymbol.symbol == Symbols::BackSlash | m_curSymbol.symbol == Symbols::TimesTimes | m_curSymbol.symbol == Symbols::PlusTimes)
                {
                    auto symbol1 = m_curSymbol;
                    m_curSymbol = m_lexer->get_symbol();
                    auto right = parse_factor();

                    left = std::make_shared<MulOpNode>(start_pos, m_curSymbol.start_pos, left, symbol1, right);
                }

                return left;

            }
        default:    return left;
    }
}

std::shared_ptr<Node> ActiveOberonParser::parse_factor()
{
    auto start_pos = m_curSymbol.start_pos;

    switch (m_curSymbol.symbol)
    {
        case Symbols::Not:
        case Symbols::Plus:
        case Symbols::Minus:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();
                auto right = parse_factor();

                return std::make_shared<UnaryOpNode>(start_pos, m_curSymbol.start_pos, symbol1, right);
            }
        default:
            return parse_unary_expression();
    }
}

std::shared_ptr<Node> ActiveOberonParser::parse_unary_expression()
{
    auto start_pos = m_curSymbol.start_pos;
    auto left = parse_primary_expression();

    std::shared_ptr<Node> right = nullptr;
    std::shared_ptr<Node> flags = nullptr;

    switch (m_curSymbol.symbol)
    {
        case Symbols::LeftParen:
        case Symbols::Period:
        case Symbols::LeftBracket:
        case Symbols::Arrow:
        case Symbols::Transpose:
            right = parse_designator_operations();
            break;
        default:    break;
    }

    if (m_curSymbol.symbol == Symbols::LeftBrace) flags = parse_flags();

    if (right == nullptr && flags == nullptr) return left;

    return std::make_shared<UnaryExpressionNode>(start_pos, m_curSymbol.start_pos, left, right, flags);
}

std::shared_ptr<Node> ActiveOberonParser::parse_primary_expression()
{
    auto start_pos = m_curSymbol.start_pos;
    
    switch (m_curSymbol.symbol)
    {
        case Symbols::Ident:
        case Symbols::Integer:
        case Symbols::Real:
        case Symbols::Character:
        case Symbols::String:
            {
                auto symbol1 = m_curSymbol;
                auto content = m_lexer->get_content_collected();
                m_curSymbol = m_lexer->get_symbol();

                return std::make_shared<LiteralNode>(start_pos, m_curSymbol.start_pos, symbol1, content);
            }
        case Symbols::LeftBrace:    
            return parse_set();

        case Symbols::LeftBracket:  
            return parse_array();

        case Symbols::Nil:
        case Symbols::Imag:
        case Symbols::True:
        case Symbols::False:
        case Symbols::Self:
        case Symbols::Result:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                return std::make_shared<AtomLiteralNode>(start_pos, m_curSymbol.start_pos, symbol1);
            }
        case Symbols::Address:
        case Symbols::Size:
        case Symbols::Alias:
        case Symbols::New:
        case Symbols::LeftParen:
            return nullptr;
        default:    throw ;
    }

}

std::shared_ptr<Node> ActiveOberonParser::parse_designator_operations()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_expression_list()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_index_list()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_array()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_set()
{
    return nullptr;
}