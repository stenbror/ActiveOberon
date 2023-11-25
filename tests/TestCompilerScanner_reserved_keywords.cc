
#include <memory>

#include "assertions_macros.h"
#include "../scanner.h"

using namespace ActiveOberon::Compiler;

void reserved_keyword_AWAIT()
{
    auto text = std::u32string(U"AWAIT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Await);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_BEGIN()
{
    auto text = std::u32string(U"BEGIN");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Begin);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_BY()
{
    auto text = std::u32string(U"BY");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::By);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

int main() {
    reserved_keyword_AWAIT();
    reserved_keyword_BEGIN();
    reserved_keyword_BY();
    return 0;
}
