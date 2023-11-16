
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class DesignatorCallNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_node;
            Token m_symbol2;
            
        public:
            DesignatorCallNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> node, Token symbol2) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_node = node;
                
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_node() { return m_node; }
            Token get_symbol2() { return m_symbol2; }
    };

    class DesignatorIndexNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_node;
            Token m_symbol2;
            
        public:
            DesignatorIndexNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> node, Token symbol2) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_node = node;
                
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_node() { return m_node; }
            Token get_symbol2() { return m_symbol2; }
    };

    class DesignatorDotNameNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_node;
            
        public:
            DesignatorDotNameNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> node) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_node = node;
                
            }
            Token get_dot() { return m_symbol1; }
            std::shared_ptr<Node> get_identifier() { return m_node; }
    };

    class DesignatorNode : public Node
    {
        private:
            Token m_symbol1;
            
        public:
            DesignatorNode(unsigned int start, unsigned int end, Token symbol1) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1;
                
            }
            Token get_symbol() { return m_symbol1; }
    };
}