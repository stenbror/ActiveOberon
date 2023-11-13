
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class UnaryOpNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_right;

        public:
            UnaryOpNode(unsigned int start, unsigned int end, Token symbol, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol; m_right = right;
            }
            Token get_operator() { return m_symbol1; }
            std::shared_ptr<Node> get_right() { return m_right; }
    };
}