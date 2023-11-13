
#include <parser.h>
#include <ast/ast_relation_node.h>
#include <ast/ast_range_node.h>
#include <ast/ast_add_op_node.h>

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
    return nullptr;
}