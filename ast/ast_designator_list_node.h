
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class DesignatorListNode : public Node
    {
        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            
        public:
            DesignatorListNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes) 
                : Node(start, end, NodeFamily::Expression)
            {
                m_nodes = nodes;
                
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
    };
}