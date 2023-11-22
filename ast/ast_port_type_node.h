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

        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_separators;
            Token m_symbol1;
            std::shared_ptr<Node> m_right;

        public:
            PortDeclarationNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> separators, Token symbol1, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Block)
            {
                m_nodes = nodes; m_separators = separators; m_symbol1 = symbol1; m_right = right;
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_separators() { return m_separators; }
            Token get_symbol() { return m_symbol1; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
    };

    class PortListNode : public Node{

        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_separators;

        public:
            PortListNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> separators) : Node(start, end, NodeFamily::Block)
            {
                m_nodes = nodes; m_separators = separators;
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_separators() { return m_separators; }
    };

}