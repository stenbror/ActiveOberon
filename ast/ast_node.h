
#pragma once

namespace ActiveOberon::Compiler
{
    struct Location
    {
        unsigned int start_pos;
        unsigned int end_pos;
    };

    class Node
    {
        private:
            unsigned int start_pos;
            unsigned int end_pos;
            bool m_statement;

        public:
            Node(unsigned int start, unsigned int end, bool is_statement)
            {
                start_pos = start;
                end_pos = end;
                this->m_statement = is_statement;
            }

            Location get_location()
            {
                return Location
                {
                    start_pos,
                    end_pos
                };
            }

            bool is_statement() { return m_statement; }

            bool is_expression() { return !m_statement; }
    };
}