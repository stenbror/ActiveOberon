
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class AddOpNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            Token m_symbol1;
            std::shared_ptr<Node> m_right;

        public:
            AddOpNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, Token symbol, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Expression)
            {
                m_left = left; m_symbol1 = symbol; m_right = right;
            }
            std::shared_ptr<Node> get_left() { return m_left; }
            Token get_operator() { return m_symbol1; }
            std::shared_ptr<Node> get_right() { return m_right; }
    };
}