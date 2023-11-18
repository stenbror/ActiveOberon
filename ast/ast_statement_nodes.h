#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class IfStatementNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_left;
            Token m_symbol2;
            std::shared_ptr<Node> m_right;

        public:
            IfStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> left, Token symbol2, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_left = left; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
    };

    class ElifStatementNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            Token m_symbol1;
            std::shared_ptr<Node> m_right;
            Token m_symbol2;
            std::shared_ptr<Node> m_next;

        public:
            ElifStatementNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, Token symbol1, std::shared_ptr<Node> right, Token symbol2, std::shared_ptr<Node> next) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_left = left; m_right = right; m_next = next;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            std::shared_ptr<Node> get_next_node() { return m_right; }
    };

    class ElseStatementNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            Token m_symbol1;
            std::shared_ptr<Node> m_right;

        public:
            ElseStatementNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, Token symbol1, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_left = left; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
    };


}