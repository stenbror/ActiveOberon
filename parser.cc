
#include <parser.h>
#include <ast/ast_relation_node.h>
#include <ast/ast_range_node.h>
#include <ast/ast_add_op_node.h>
#include <ast/ast_mul_op_node.h>
#include <ast/ast_unary_op_node.h>
#include <ast/ast_unary_expression_node.h>
#include <ast/ast_literal_node.h>
#include <ast/ast_atom_literal_node.h>
#include <ast/ast_atom_of_literal_node.h>
#include <ast/ast_parenthesis_expression_node.h>
#include <ast/ast_memory_new_node.h>
#include <ast/ast_set_node.h>
#include <ast/ast_array_node.h>
#include <ast/ast_expression_list.h>
#include <ast/ast_index_list_node.h>
#include <ast/ast_designator_list_node.h>
#include <ast/ast_designator_types_node.h>
#include <ast/ast_statement_sequence_node.h>

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

std::shared_ptr<Node> ActiveOberonParser::parse_qualified_identifier()
{
    return nullptr;
}

// Statement rules ////////////////////////////////////////////////////////////////////////////////////////////////////

std::shared_ptr<Node> ActiveOberonParser::parse_statement()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_case()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_statement_block()
{
    return nullptr;
}

std::shared_ptr<Node> ActiveOberonParser::parse_statement_sequence()
{
    auto start_pos = m_curSymbol.start_pos;
    auto nodes = std::make_shared<std::vector<std::shared_ptr<Node>>>();
    auto separators = std::make_shared<std::vector<Token>>();

    nodes->push_back(parse_statement());

    while (m_curSymbol.symbol == Symbols::SemiColon)
    {
        separators->push_back(m_curSymbol);
        m_curSymbol = m_lexer->get_symbol();

        nodes->push_back(parse_statement());
    }

    return std::make_shared<StatementSequenceNode>(start_pos, m_curSymbol.start_pos, nodes, separators);
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
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();
                if (m_curSymbol.symbol == Symbols::Of)
                {
                    auto symbol2 = m_curSymbol;
                    m_curSymbol = m_lexer->get_symbol();
                    auto right = parse_factor();

                    return std::make_shared<AtomOfLiteralNode>(start_pos, m_curSymbol.start_pos, symbol1, symbol2, right);
                }

                return std::make_shared<AtomLiteralNode>(start_pos, m_curSymbol.start_pos, symbol1);
            }
        case Symbols::Alias:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                if (m_curSymbol.symbol != Symbols::Of) throw ;

                auto symbol2 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();
                auto right = parse_factor();

                return std::make_shared<AtomOfLiteralNode>(start_pos, m_curSymbol.start_pos, symbol1, symbol2, right);
            }
        case Symbols::New:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                auto left = parse_qualified_identifier();
                if (m_curSymbol.symbol != Symbols::LeftParen) throw ;

                auto symbol2 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                auto right = parse_expression_list();
                if (m_curSymbol.symbol != Symbols::RightParen) throw ;

                auto symbol3 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                return std::make_shared<MemoryNewExpressionNode>(start_pos, m_curSymbol.start_pos, symbol1, left, symbol2, right, symbol3);
            }
        case Symbols::LeftParen:
            {
                auto symbol1 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                auto right = parse_expression();
                if (m_curSymbol.symbol != Symbols::RightParen) throw ;

                auto symbol2 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                return std::make_shared<ParenthesisExpressionNode>(start_pos, m_curSymbol.start_pos, symbol1, right, symbol2);
            }

        default:    throw ;
    }

}

std::shared_ptr<Node> ActiveOberonParser::parse_designator_operations()
{
    auto start_pos = m_curSymbol.start_pos;
    auto nodes = std::make_shared<std::vector<std::shared_ptr<Node>>>();

    while ( m_curSymbol.symbol == Symbols::LeftParen ||
            m_curSymbol.symbol == Symbols::Period ||
            m_curSymbol.symbol == Symbols::LeftBracket ||
            m_curSymbol.symbol == Symbols::Arrow ||
            m_curSymbol.symbol == Symbols::Transpose)
            {
                auto start_pos_local = m_curSymbol.start_pos;

                switch (m_curSymbol.symbol)
                {
                    case Symbols::LeftParen:
                        {
                            auto symbol1 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();
                            std::shared_ptr<Node> node = nullptr;

                            if (m_curSymbol.symbol != Symbols::RightParen)
                            {
                                node = parse_expression_list();
                            }

                            if (m_curSymbol.symbol != Symbols::RightParen) throw ;

                            auto symbol2 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();
                        
                            nodes->push_back(std::make_shared<DesignatorCallNode>(start_pos_local, m_curSymbol.start_pos, symbol1, node, symbol2));
                        }
                        break;
                    case Symbols::LeftBracket:
                        {
                            auto symbol1 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();
                            std::shared_ptr<Node> node = nullptr;

                            if (m_curSymbol.symbol != Symbols::RightBracket)
                            {
                                node = parse_index_list();
                            }

                            if (m_curSymbol.symbol != Symbols::RightBracket) throw ;

                            auto symbol2 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();
                        
                            nodes->push_back(std::make_shared<DesignatorIndexNode>(start_pos_local, m_curSymbol.start_pos, symbol1, node, symbol2));
                        }
                        break;
                    case Symbols::Period:
                        {
                            auto symbol1 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();

                            if (m_curSymbol.symbol != Symbols::Ident) throw ;

                            auto symbol2 = m_curSymbol;
                            auto content = m_lexer->get_content_collected();
                            m_curSymbol = m_lexer->get_symbol();

                            nodes->push_back(std::make_shared<DesignatorDotNameNode>(start_pos_local, m_curSymbol.start_pos, symbol1, std::make_shared<LiteralNode>(start_pos_local, m_curSymbol.start_pos, symbol2, content)));
                        }
                        break;
                    case Symbols::Arrow:
                    case Symbols::Transpose:
                        {
                            auto symbol1 = m_curSymbol;
                            m_curSymbol = m_lexer->get_symbol();

                            nodes->push_back(std::make_shared<DesignatorNode>(start_pos_local, m_curSymbol.start_pos, symbol1));
                        }
                        break;
                    default:    break;
                }
            }

    return std::make_shared<DesignatorListNode>(start_pos, m_curSymbol.symbol, nodes);
}

std::shared_ptr<Node> ActiveOberonParser::parse_expression_list()
{
    auto start_pos = m_curSymbol.start_pos;
    auto nodes = std::make_shared<std::vector<std::shared_ptr<Node>>>();
    auto commas = std::make_shared<std::vector<Token>>();

    auto node = parse_expression();

    if (m_curSymbol.symbol == Symbols::Comma)
    {
        nodes->push_back(node);

        while (m_curSymbol.symbol == Symbols::Comma)
        {
            commas->push_back(m_curSymbol);
            m_curSymbol = m_lexer->get_symbol();

            nodes->push_back(parse_expression());
        }

        return std::make_shared<ExpressionListNode>(start_pos, m_curSymbol.start_pos, nodes, commas);
    }

    return node;
}

std::shared_ptr<Node> ActiveOberonParser::parse_index_list()
{
    auto start_pos = m_curSymbol.start_pos;
    std::shared_ptr<Node> nodes_left = nullptr;
    std::shared_ptr<Node> nodes_right = nullptr;
    auto symbol1 = Token { Symbols::Empty, 0, 0 };
    auto symbol2 = Token { Symbols::Empty, 0, 0 };
    auto symbol3 = Token { Symbols::Empty, 0, 0 };

    if (m_curSymbol.symbol == Symbols::QuestionMark)
    {
        symbol2 = m_curSymbol;
        m_curSymbol = m_lexer->get_symbol();

        if (m_curSymbol.symbol == Symbols::Comma)
        {
            symbol3 = m_curSymbol;
            m_curSymbol = m_lexer->get_symbol();

            nodes_right = parse_expression_list();
        }
    }
    else
    {
        nodes_left = parse_expression_list();

        if (m_curSymbol.symbol == Symbols::Comma)
        {
            symbol1 = m_curSymbol;
            m_curSymbol = m_lexer->get_symbol();

            if (m_curSymbol.symbol == Symbols::QuestionMark)
            {
                symbol2 = m_curSymbol;
                m_curSymbol = m_lexer->get_symbol();

                if (m_curSymbol.symbol == Symbols::Comma)
                {
                    symbol1 = m_curSymbol;
                    m_curSymbol = m_lexer->get_symbol();

                    nodes_right = parse_expression_list();
                }
            }
            else throw ;
        }
    }

    return  std::make_shared<IndexListNode>(start_pos, m_curSymbol.start_pos, nodes_left, symbol1, symbol2, symbol3, nodes_right);
}

std::shared_ptr<Node> ActiveOberonParser::parse_array()
{
    auto start_pos = m_curSymbol.start_pos;
    auto nodes = std::make_shared<std::vector<std::shared_ptr<Node>>>();
    auto commas = std::make_shared<std::vector<Token>>();

    auto symbol1 = m_curSymbol;
    m_curSymbol = m_lexer->get_symbol();

    nodes->push_back(parse_expression());

    while (m_curSymbol.symbol == Symbols::Comma)
    {
        commas->push_back(m_curSymbol);
        m_curSymbol = m_lexer->get_symbol();

        nodes->push_back(parse_expression());
    }

    if (m_curSymbol.symbol != Symbols::RightBracket) throw ;
    auto symbol2 = m_curSymbol;
    m_curSymbol = m_lexer->get_symbol();

    return std::make_shared<ArrayNode>(start_pos, m_curSymbol.start_pos, symbol1, nodes, commas, symbol2);
}

std::shared_ptr<Node> ActiveOberonParser::parse_set()
{
    auto start_pos = m_curSymbol.start_pos;
    auto nodes = std::make_shared<std::vector<std::shared_ptr<Node>>>();
    auto commas = std::make_shared<std::vector<Token>>();

    auto symbol1 = m_curSymbol;
    m_curSymbol = m_lexer->get_symbol();

    nodes->push_back(parse_range_expression());

    while (m_curSymbol.symbol == Symbols::Comma)
    {
        commas->push_back(m_curSymbol);
        m_curSymbol = m_lexer->get_symbol();

        nodes->push_back(parse_range_expression());
    }

    if (m_curSymbol.symbol != Symbols::RightBrace) throw ;
    auto symbol2 = m_curSymbol;
    m_curSymbol = m_lexer->get_symbol();

    return std::make_shared<SetNode>(start_pos, m_curSymbol.start_pos, symbol1, nodes, commas, symbol2);
}