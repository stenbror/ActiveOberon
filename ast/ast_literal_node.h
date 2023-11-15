
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>

namespace ActiveOberon::Compiler
{

    class LiteralNode : public Node
    {
        private:
            Token m_symbol;
            std::u32string m_content;

        public:
            LiteralNode(unsigned int start, unsigned int end, Token symbol, std::u32string content) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol = symbol; m_content = content;
            }
            Token get_literal_type() { return m_symbol; }
            std::u32string get_content() { return m_content; }
    };
}