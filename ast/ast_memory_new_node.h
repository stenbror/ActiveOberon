
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>

namespace ActiveOberon::Compiler
{

    class MemoryNewExpressionNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_left;
            Token m_symbol2;
            std::shared_ptr<Node> m_right;
            Token m_symbol3;
            
        public:
            MemoryNewExpressionNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> left, Token symbol2, std::shared_ptr<Node> right, Token symbol3) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_left = left; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            Token get_symbol3() { return m_symbol3; }
    };
}