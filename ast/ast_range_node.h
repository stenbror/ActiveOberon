
#pragma once

#include <ast/ast_node.h>
#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class RangeNode : public Node
    {
        private:
            std::shared_ptr<Node> m_left;
            Token m_symbol1;
            std::shared_ptr<Node> m_right;
            Token m_symbol2;
            std::shared_ptr<Node> m_by;
            Token m_times;

        public:
            RangeNode(unsigned int start, unsigned int end, std::shared_ptr<Node> left, Token symbol1, std::shared_ptr<Node> right, Token symbol2, std::shared_ptr<Node> by, Token times) : Node(start, end, NodeFamily::Expression )
            {
                m_left = left; m_right = right; m_by = by; m_symbol1 = symbol1; m_symbol2 = symbol2; m_times = times;
            }
            std::shared_ptr<Node> get_start() { return m_left; }
            Token get_symbol1() { return m_symbol1; }
            std::shared_ptr<Node> get_end() { return m_right; }
            Token get_symbol2() { return m_symbol2; }
            std::shared_ptr<Node> get_by() { return m_by; }
            Token get_star() { return m_times; }

    };
}