
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class IndexListNode : public Node
    {
        private:
            std::shared_ptr<Node> m_nodes_left;
            Token m_symbol1;
            std::shared_ptr<Node> m_nodes_right;
            Token m_symbol2;
            Token m_symbol3;
            
        public:
            IndexListNode(unsigned int start, unsigned int end, std::shared_ptr<Node> nodes_left, Token symbol1, Token symbol2, Token symbol3, std::shared_ptr<Node> nodes_right) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_nodes_left = nodes_left;
                m_symbol1 = symbol1;
                m_symbol2 = symbol2;
                m_symbol3 = symbol3;
                m_nodes_right = nodes_right;
                
            }
            std::shared_ptr<Node> get_left_node_list() { return m_nodes_left; }
            Token get_comma_one() { return m_symbol1; }
            Token get_question_mark() { return m_symbol2; }
            Token get_comma_two() { return m_symbol2; }
            std::shared_ptr<Node> get_right_node_list() { return m_nodes_right; }
    };
}