#pragma once

#include <ast/ast_node.h>
#include <ast/ast_literal_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class PortTypeNode : public Node
    {
        private:
            Token m_symbol1;
            Token m_symbol2;
            Token m_symbol3;
            std::shared_ptr<Node> m_right;
            Token m_symbol4;

        public:
            PortTypeNode(unsigned int start, unsigned int end, Token symbol1, Token symbol2, Token symbol3, std::shared_ptr<Node> right, Token symbol4) : Node(start, end, NodeFamily::Block)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_symbol4 = symbol4; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            Token get_symbol2() { return m_symbol2; }
            Token get_symbol3() { return m_symbol3; }
            std::shared_ptr<Node> get_right() { return m_right; }
            Token get_symbol4() { return m_symbol4; }
    };

    class PortDeclarationNode : public Node{

    };

    class PortListNode : public Node{

    };

}