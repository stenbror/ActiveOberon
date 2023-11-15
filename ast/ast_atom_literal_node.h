
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>

namespace ActiveOberon::Compiler
{

    class AtomLiteralNode : public Node
    {
        private:
            Token m_symbol;
            
        public:
            AtomLiteralNode(unsigned int start, unsigned int end, Token symbol) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol = symbol;
            }
            Token get_literal_type() { return m_symbol; }
    };
}