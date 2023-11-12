
#include <parser.h>
#include <ast/ast_relation_node.h>

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
    return nullptr;
}