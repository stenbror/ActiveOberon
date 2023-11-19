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
            Token m_symbol1;    /* 'IF' */
            std::shared_ptr<Node> m_left;
            Token m_symbol2;    /* 'THEN' */
            std::shared_ptr<Node> m_right;
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<Node> m_else;
            Token m_symbol3; /* 'END' */

        public:
            IfStatementNode(
                        unsigned int start, 
                        unsigned int end, 
                        Token symbol1, 
                        std::shared_ptr<Node> left, 
                        Token symbol2, 
                        std::shared_ptr<Node> right,
                        std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes,
                        std::shared_ptr<Node> node,
                        Token symbol3
                    ) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_left = left; m_right = right; m_nodes = nodes; m_else = node;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<Node> get_next_node() { return m_else; }
            Token get_symbol3() { return m_symbol3; }
    };

    class ElsifStatementNode : public Node
    {
        private:
            Token m_symbol1;    /* 'ELSIF' */
            std::shared_ptr<Node> m_right;
            Token m_symbol2;    /* 'THEN' */
            std::shared_ptr<Node> m_next;

        public:
            ElsifStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> right, Token symbol2, std::shared_ptr<Node> next) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_right = right; m_next = next;
            }
            Token get_symbol1() { return m_symbol1; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            std::shared_ptr<Node> get_next_node() { return m_right; }
    };

    class ElseStatementNode : public Node
    {
        private:
            Token m_symbol1;    /* 'ELSE' */
            std::shared_ptr<Node> m_right;

        public:
            ElseStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Statement) 
            {
                m_symbol1 = symbol1; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
    };



    class WhileStatementNode : public Node
    {
        private:
            Token m_symbol1;    /* 'WHILE' */
            std::shared_ptr<Node> m_left;
            Token m_symbol2;    /* 'DO' */
            std::shared_ptr<Node> m_right;
            Token m_symbol3;    /* 'END' */

        public:
            WhileStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> left, Token symbol2, std::shared_ptr<Node> right, Token symbol3) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_left = left; m_right = right; 
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            Token get_symbol3() { return m_symbol3; }
    };

    class RepeatStatementNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_left;   /* Statement sequence */
            std::shared_ptr<Node> m_right;  /* Expression */
            Token m_symbol2;

        public:
            RepeatStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> left, Token symbol2, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_left = left; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            Token get_symbol2() { return m_symbol2; }
    };

}