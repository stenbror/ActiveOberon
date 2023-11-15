
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>
#include <vector>

namespace ActiveOberon::Compiler
{

    class ArrayNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_commas;
            Token m_symbol2;
            
            
        public:
            ArrayNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> commas, Token symbol2) : Node(start, end, NodeFamily::Expression)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_nodes = nodes; m_commas = commas;
            }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_commas() { return m_commas; }
            Token get_symbol2() { return m_symbol2; }
    };
}