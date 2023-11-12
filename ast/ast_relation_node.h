
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class RelationNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            Token m_operator;
            std::shared_ptr<Node> m_right;

        public:
            RelationNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, Token symbol, std::shared_ptr<Node> right) 
                : Node(start, end, NodeFamily::Expression)
                {
                    m_left = left; m_operator = symbol; m_right = right;
                }
            std::shared_ptr<Node> get_left() { return m_left; }
            Token get_operator() { return m_operator; }
            std::shared_ptr<Node> get_right() { return m_right; }
    };

}