
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class StatementSequenceNode : public Node
    {
        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_separators;
            
        public:
            StatementSequenceNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> separators) : Node(start, end, NodeFamily::Statement)
            {
                m_nodes = nodes; m_separators = separators;
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_statements() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_separators() { return m_separators; }
    };
}