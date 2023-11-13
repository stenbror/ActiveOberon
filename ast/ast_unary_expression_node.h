
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class UnaryExpressionNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            std::shared_ptr<Node> m_right;
            std::shared_ptr<Node> m_flags;

        public:
            UnaryExpressionNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, std::shared_ptr<Node> right, std::shared_ptr<Node> flags) : Node(start, end, NodeFamily::Expression)
            {
                m_left = left; m_right = right; m_flags = flags;
            }
            std::shared_ptr<Node> get_left() { return m_left; }
            std::shared_ptr<Node> get_right() { return m_right; }
            std::shared_ptr<Node> get_flags() { return m_flags; }
    };
}