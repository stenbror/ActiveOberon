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

    class ForStatementNode : public Node
    {
        private:
            Token m_symbol1;    /* 'FOR' */
            std::shared_ptr<Node> m_first;  /*  Identifier */
            Token m_symbol2;    /* ':=' */
            std::shared_ptr<Node> m_second; /*  Expression */
            Token m_symbol3;    /* 'TO' */
            std::shared_ptr<Node> m_third;  /*  'to' Expression */
            Token m_symbol4;    /* 'BY' */
            std::shared_ptr<Node> m_four;   /*  'by' Expression */
            Token m_symbol5;    /* 'DO' */
            std::shared_ptr<Node> m_five;   /* Statement sequence */
            Token m_symbol6;    /* 'END' */
            

        public:
            ForStatementNode(
                    unsigned int start, 
                    unsigned int end,
                    Token symbol1,
                    std::shared_ptr<Node> first,
                    Token symbol2,
                    std::shared_ptr<Node> second,
                    Token symbol3,
                    std::shared_ptr<Node> third,
                    Token symbol4,
                    std::shared_ptr<Node> four,
                    Token symbol5,
                    std::shared_ptr<Node> five,
                    Token symbol6
                ) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_symbol4 = symbol4; m_symbol5 = symbol5; m_symbol6 = symbol6;
                m_first = first; m_second = second; m_third = third; m_four = four; m_five = five;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_first() { return m_first; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_second() { return m_second; }
            Token get_symbol3() { return m_symbol3; }
            std::shared_ptr<Node> get_third() { return m_third; }
            Token get_symbol4() { return m_symbol4; }
            std::shared_ptr<Node> get_fourth() { return m_four; }
            Token get_symbol5() { return m_symbol5; }
            std::shared_ptr<Node> get_five() { return m_five; }
            Token get_symbol6() { return m_symbol6; }
    };

    class LoopStatementNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_right;
            Token m_symbol2;

        public:
            LoopStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> right, Token symbol2) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_right() { return m_right; }
            Token get_symbol2() { return m_symbol2; }
    };

    class ExitStatementNode : public Node{

        private:
            Token m_symbol;

        public:
            ExitStatementNode(unsigned int start, unsigned int end, Token symbol1) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol = symbol1;
            }
            Token get_symbol() { return m_symbol; }
    };

    class SimpleStatementNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_right;

        public:
            SimpleStatementNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> right) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_right = right;
            }
            Token get_symbol() { return m_symbol1; }
            std::shared_ptr<Node> get_node() { return m_right; }
    };

    class InlineAssemblerNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_right;
            Token m_symbol2;

        public:
            InlineAssemblerNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> right, Token symbol2) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_right = right;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_right() { return m_right; }
            Token get_symbol2() { return m_symbol2; }
    };

}