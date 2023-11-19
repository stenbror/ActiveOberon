#pragma once

#include <ast/ast_node.h>
#include <ast/ast_literal_node.h>
#include <scanner.h>
#include <memory>
#include <vector>

namespace ActiveOberon::Compiler
{
    class QualidentIdentifierNode : public Node
    {
        private:
            std::shared_ptr<LiteralNode> m_left;
            Token m_symbol;
            std::shared_ptr<LiteralNode> m_right;

        public:
            QualidentIdentifierNode(unsigned int start, unsigned int end, std::shared_ptr<LiteralNode> left, Token symbol, std::shared_ptr<LiteralNode> right) : Node(start, end, NodeFamily::Block)
            {
                m_left = left; m_symbol = symbol; m_right = right;
            }
            std::shared_ptr<Node> get_left_node() { return m_left; }
            Token get_symbol() { return m_symbol; }
            std::shared_ptr<Node> get_right_node() { return m_right; }
            std::u32string get_content_first() { return m_left->get_content(); }
            std::u32string get_content_second() { return m_right->get_content(); }
    };

}