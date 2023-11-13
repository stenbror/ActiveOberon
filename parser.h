
#pragma once

#include <scanner.h>
#include <memory>
#include <ast/ast_node.h>

namespace ActiveOberon::Compiler
{

    class ActiveOberonParser
    {
        protected:
            std::shared_ptr<ActiveOberonScanner> m_lexer;
            Token m_curSymbol;

        public:
            ActiveOberonParser(std::shared_ptr<ActiveOberonScanner> scanner);
            ~ActiveOberonParser();

            std::shared_ptr<Node> parse_module();


        public:
            std::shared_ptr<Node> parse_expression();
            std::shared_ptr<Node> parse_range_expression();
            std::shared_ptr<Node> parse_SimpleExpression();
            std::shared_ptr<Node> parse_term();
            std::shared_ptr<Node> parse_factor();
            std::shared_ptr<Node> parse_unary_expression();
    };

}