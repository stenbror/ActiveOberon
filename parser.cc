
#include <parser.h>

using namespace ActiveOberon::Compiler;

ActiveOberonParser::ActiveOberonParser(std::shared_ptr<ActiveOberonScanner> scanner)
{
    m_lexer = scanner;
    m_curSymbol = m_lexer->get_symbol();
}

ActiveOberonParser::~ActiveOberonParser() 
{

}