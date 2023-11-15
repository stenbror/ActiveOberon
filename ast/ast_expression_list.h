
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <string>
#include <vector>

namespace ActiveOberon::Compiler
{

    class ExpressionListNode : public Node
    {
        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_commas;
            
        public:
            ExpressionListNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> commas) : Node(start, end, NodeFamily::Expression)
            {
                m_nodes = nodes; m_commas = commas;
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_commas() { return m_commas; }
    };
}