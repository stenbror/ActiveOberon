#pragma once

#include <ast/ast_node.h>
#include <ast/ast_literal_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{
    class IdentifierDefinitionNode : public Node
    {
        private:
            std::shared_ptr<LiteralNode> m_name;
            Token m_symbol;

        public:
            IdentifierDefinitionNode(unsigned int start, unsigned int end, std::shared_ptr<LiteralNode> name, Token symbol) : Node(start, end, NodeFamily::Block)
            {
                m_name = name; m_symbol = symbol;
            }
            std::shared_ptr<Node> get_name_node() { return m_name; }
            Token get_symbol() { return m_symbol; }
            bool is_export_read_only() { return m_symbol.symbol == Symbols::Minus; }
            bool is_export_read_write() { return m_symbol.symbol == Symbols::Times; }
            std::u32string get_name_text() { return m_name->get_content(); }
    };

}