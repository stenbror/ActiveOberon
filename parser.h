
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

        public: /* Declaration rules */
            std::shared_ptr<Node> parse_module();


            std::shared_ptr<Node> parse_flags();

            std::shared_ptr<Node> parse_qualified_identifier();

        public: /* Statement rules */
            std::shared_ptr<Node> parse_statement();
            std::shared_ptr<Node> parse_case();
            std::shared_ptr<Node> parse_statement_block();
            std::shared_ptr<Node> parse_statement_sequence();

        public: /* Expression rules */
            std::shared_ptr<Node> parse_expression();
            std::shared_ptr<Node> parse_range_expression();
            std::shared_ptr<Node> parse_SimpleExpression();
            std::shared_ptr<Node> parse_term();
            std::shared_ptr<Node> parse_factor();
            std::shared_ptr<Node> parse_unary_expression();
            std::shared_ptr<Node> parse_primary_expression();
            std::shared_ptr<Node> parse_designator_operations();
            std::shared_ptr<Node> parse_expression_list();
            std::shared_ptr<Node> parse_index_list();
            std::shared_ptr<Node> parse_array();
            std::shared_ptr<Node> parse_set();
    };

}