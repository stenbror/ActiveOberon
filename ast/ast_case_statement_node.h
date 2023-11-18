
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class CaseStatementNode : public Node
    {
        private:
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> m_nodes;
            std::shared_ptr<std::vector<Token>> m_separators;
            Token m_symbol;
            std::shared_ptr<Node> m_statements;
            
        public:
            CaseStatementNode(unsigned int start, unsigned int end, std::shared_ptr<std::vector<std::shared_ptr<Node>>> nodes, std::shared_ptr<std::vector<Token>> separators, Token symbol, std::shared_ptr<Node> statements) : Node(start, end, NodeFamily::Statement)
            {
                m_nodes = nodes; m_separators = separators; m_symbol = symbol; m_statements = statements;
            }
            std::shared_ptr<std::vector<std::shared_ptr<Node>>> get_nodes() { return m_nodes; }
            std::shared_ptr<std::vector<Token>> get_separators() { return m_separators; }
            Token get_symbol() { return m_symbol; }
            std::shared_ptr<Node> get_statements() { return m_statements; }
    };
}