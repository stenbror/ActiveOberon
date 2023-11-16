
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{

    class StatementBlockNode : public Node
    {
        private:
            Token m_symbol1;
            std::shared_ptr<Node> m_flags;
            std::shared_ptr<Node> m_statements;
            Token m_symbol2;
            
        public:
            StatementBlockNode(unsigned int start, unsigned int end, Token symbol1, std::shared_ptr<Node> flags, std::shared_ptr<Node> statements, Token symbol2) : Node(start, end, NodeFamily::Statement)
            {
                m_symbol1 = symbol1; m_symbol2 = symbol2; m_flags = flags; m_statements = statements;
            }
            Token get_begin() { return m_symbol1; }
            std::shared_ptr<Node> get_flags() { return m_flags; }
            std::shared_ptr<Node> get_statements() { return m_statements; }
            Token get_end() { return m_symbol2; }
    };
}