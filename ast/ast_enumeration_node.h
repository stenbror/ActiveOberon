#pragma once

#include <ast/ast_node.h>
#include <ast/ast_literal_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class EnumerationNode : public Node
    {
        private:
            Token m_symbol1;                                                    /* 'ENUM' */
            Token m_symbol2;                                                    /* '(' */
            std::shared_ptr<Node> m_left;                                       /* qualident identifier */
            Token m_symbol3;                                                    /* '')'*/
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_elements;     /* elements */
            std::shared_ptr<std::vector<Token>> m_separators;                   /* ',' */
            Token m_symbol4;                                                    /* 'END' */

        public:
            EnumerationNode(
                    unsigned int start, 
                    unsigned int end, 
                    Token symbol1, 
                    Token symbol2, 
                    std::shared_ptr<Node> left, 
                    Token symbol3,
                    std::shared_ptr<std::vector<std::shared_ptr<Node>>> elements, 
                    std::shared_ptr<std::vector<Token>> separators,
                    Token symbol4
                ) : Node(start, end, NodeFamily::Block)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_symbol3 = symbol3; m_symbol4 = symbol4; m_left = left; m_elements = elements; m_separators = separators;
            }
            Token get_symbol1() { return m_symbol1; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_left() { return m_left; }
            Token get_symbol3() { return m_symbol3; }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_elements() { return m_elements; }
            std::shared_ptr<std::vector<Token>> get_separators() { return m_separators; }
            Token get_symbol4() { return m_symbol4; }
    };

    class EnumerationValueNode : public Node
    {
        private:
            std::shared_ptr<Node> m_key;
            Token m_symbol;
            std::shared_ptr<Node> m_value;

        public:
            EnumerationValueNode(unsigned int start, unsigned int end, std::shared_ptr<Node> key, Token symbol, std::shared_ptr<Node> value) : Node(start, end, NodeFamily::Block)
            {
                m_key = key; m_symbol = symbol; m_value = value; 
            }
            std::shared_ptr<Node> get_key() { return m_key; }
            Token get_symbol() { return m_symbol; }
            std::shared_ptr<Node> get_value() { return m_value; }
    };
}