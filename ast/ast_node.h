
#pragma once

namespace ActiveOberon::Compiler
{
    struct Location
    {
        unsigned int start_pos;
        unsigned int end_pos;
    };

    enum NodeFamily
    {
        Expression,
        Statement,
        Assembler,
        Block
    };

    class Node
    {
        private:
            unsigned int start_pos;
            unsigned int end_pos;
            NodeFamily m_family;

        public:
            Node(unsigned int start, unsigned int end, NodeFamily family)
            {
                start_pos = start;
                end_pos = end;
                this->m_family = family;
            }

            Location get_location()
            {
                return Location
                {
                    start_pos,
                    end_pos
                };
            }

            bool is_statement() { return m_family == NodeFamily::Statement; }

            bool is_expression() { return m_family == NodeFamily::Expression; }

            bool is_block() { return m_family == NodeFamily::Block; }

            bool is_assembler() { return m_family == NodeFamily::Assembler; }
    };
}