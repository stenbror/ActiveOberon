
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>

namespace ActiveOberon::Compiler
{

    class AtomOfLiteralNode : public Node
    {
        private:
            Token m_symbol1;
            Token m_symbol2;
            std::shared_ptr<Node> m_right;
            
        public:
            AtomOfLiteralNode(unsigned int start, unsigned int end, Token symbol1, Token symbol2, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            Token get_symbol2() { return m_symbol2; }
    };
}