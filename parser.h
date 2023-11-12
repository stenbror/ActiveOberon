
#pragma once

#include <scanner.h>
#include <memory>

namespace ActiveOberon::Compiler
{

    class ActiveOberonParser
    {
        protected:
            std::shared_ptr<ActiveOberonScanner> m_lexer;
            Token m_curSymbol;

        public:
            ActiveOberonParser(std::shared_ptr<ActiveOberonScanner> scanner);
            ~ActiveOberonParser();
    };

}