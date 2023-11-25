
#include <memory>

#include "assertions_macros.h"
#include "../scanner.h"

using namespace ActiveOberon::Compiler;

// UnitTests //////////////////////////////////////////////////////////////////

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

void reserved_keyword_CONST()
{
    auto text = std::u32string(U"CONST");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Const);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_CASE()
{
    auto text = std::u32string(U"CASE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Case);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_CELL()
{
    auto text = std::u32string(U"CELL");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Cell);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_CELLNET()
{
    auto text = std::u32string(U"CELLNET");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::CellNet);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 7);
}

void reserved_keyword_CODE()
{
    auto text = std::u32string(U"CODE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Code);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_DO()
{
    auto text = std::u32string(U"DO");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Do);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_DIV()
{
    auto text = std::u32string(U"DIV");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Div);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_END()
{
    auto text = std::u32string(U"END");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::End);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_ENUM()
{
    auto text = std::u32string(U"ENUM");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Enum);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_ELSE()
{
    auto text = std::u32string(U"ELSE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Else);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_ELSIF()
{
    auto text = std::u32string(U"ELSIF");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Elsif);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_EXIT()
{
    auto text = std::u32string(U"EXIT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Exit);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_EXTERN()
{
    auto text = std::u32string(U"EXTERN");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Extern);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_FOR()
{
    auto text = std::u32string(U"FOR");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::For);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_FINALLY()
{
    auto text = std::u32string(U"FINALLY");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Finally);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 7);
}

void reserved_keyword_IF()
{
    auto text = std::u32string(U"IF");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::If);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_IGNORE()
{
    auto text = std::u32string(U"IGNORE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Ignore);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_IMAG()
{
    auto text = std::u32string(U"IMAG");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Imag);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_IS()
{
    auto text = std::u32string(U"IS");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Is);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_IN()
{
    auto text = std::u32string(U"IN");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::In);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_IMPORT()
{
    auto text = std::u32string(U"IMPORT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Import);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_LOOP()
{
    auto text = std::u32string(U"LOOP");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Loop);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_MODULE()
{
    auto text = std::u32string(U"MODULE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Module);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_MOD()
{
    auto text = std::u32string(U"MOD");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Mod);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_NIL()
{
    auto text = std::u32string(U"NIL");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Nil);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_OF()
{
    auto text = std::u32string(U"OF");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Of);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_OR()
{
    auto text = std::u32string(U"OR");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Or);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_OUT()
{
    auto text = std::u32string(U"OUT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Out);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_OPERATOR()
{
    auto text = std::u32string(U"OPERATOR");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Operator);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 8);
}

void reserved_keyword_PROCEDURE()
{
    auto text = std::u32string(U"PROCEDURE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Procedure);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 9);
}

void reserved_keyword_PORT()
{
    auto text = std::u32string(U"PORT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Port);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_REPEAT()
{
    auto text = std::u32string(U"REPEAT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Repeat);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_RETURN()
{
    auto text = std::u32string(U"RETURN");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Return);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_SELF()
{
    auto text = std::u32string(U"SELF");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Self);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_NEW()
{
    auto text = std::u32string(U"NEW");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::New);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_RESULT()
{
    auto text = std::u32string(U"RESULT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Result);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_THEN()
{
    auto text = std::u32string(U"THEN");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Then);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_TRUE()
{
    auto text = std::u32string(U"TRUE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::True);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_TO()
{
    auto text = std::u32string(U"TO");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::To);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void reserved_keyword_TYPE()
{
    auto text = std::u32string(U"TYPE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Type);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_UNTIL()
{
    auto text = std::u32string(U"UNTIL");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Until);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_VAR()
{
    auto text = std::u32string(U"VAR");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Var);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 3);
}

void reserved_keyword_WHILE()
{
    auto text = std::u32string(U"WHILE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::While);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_WITH()
{
    auto text = std::u32string(U"WITH");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::With);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_ARRAY()
{
    auto text = std::u32string(U"ARRAY");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Array);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_OBJECT()
{
    auto text = std::u32string(U"OBJECT");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Object);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_POINTER()
{
    auto text = std::u32string(U"POINTER");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Pointer);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 7);
}

void reserved_keyword_RECORD()
{
    auto text = std::u32string(U"RECORD");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Record);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}

void reserved_keyword_ADDRESS()
{
    auto text = std::u32string(U"ADDRESS");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Address);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 7);
}

void reserved_keyword_SIZE()
{
    auto text = std::u32string(U"SIZE");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Size);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 4);
}

void reserved_keyword_ALIAS()
{
    auto text = std::u32string(U"ALIAS");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Alias);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

// Should fail UnitTests //////////////////////////////////////////////////////

void reserved_keyword_failure_lowercase()
{
    auto text = std::u32string(U"ALiAS");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();
    auto content = lexer->get_content_collected();

    ASSERT_EQUAL(symb.symbol, Symbols::Ident);
    ASSERT_EQUAL(content.compare(std::u32string(U"ALiAS")), 0 );
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 5);
}

void reserved_keyword_failure_ketword_addition()
{
    auto text = std::u32string(U"UNTILX");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();
    auto content = lexer->get_content_collected();
    
    ASSERT_EQUAL(symb.symbol, Symbols::Ident);
    ASSERT_EQUAL(content.compare(std::u32string(U"UNTILX")), 0 );
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 6);
}


// Test harness for reserved keywords /////////////////////////////////////////

int main() {

    reserved_keyword_AWAIT();
    reserved_keyword_BEGIN();
    reserved_keyword_BY();
    reserved_keyword_CONST();
    reserved_keyword_CASE();
    reserved_keyword_CELL();
    reserved_keyword_CELLNET();
    reserved_keyword_CODE();
    reserved_keyword_DO();
    reserved_keyword_DIV();
    reserved_keyword_END();
    reserved_keyword_ENUM();
    reserved_keyword_ELSE();
    reserved_keyword_ELSIF();
    reserved_keyword_EXIT();
    reserved_keyword_EXTERN();
    reserved_keyword_FOR();
    reserved_keyword_FINALLY();
    reserved_keyword_IF();
    reserved_keyword_IGNORE();
    reserved_keyword_IMAG();
    reserved_keyword_IS();
    reserved_keyword_IN();
    reserved_keyword_LOOP();
    reserved_keyword_MODULE();
    reserved_keyword_MOD();
    reserved_keyword_NIL();
    reserved_keyword_OF();
    reserved_keyword_OR();
    reserved_keyword_OUT();
    reserved_keyword_OPERATOR();
    reserved_keyword_PROCEDURE();
    reserved_keyword_PORT();
    reserved_keyword_REPEAT();
    reserved_keyword_RETURN();
    reserved_keyword_SELF();
    reserved_keyword_NEW();
    reserved_keyword_RESULT();
    reserved_keyword_THEN();
    reserved_keyword_TRUE();
    reserved_keyword_TO();
    reserved_keyword_TYPE();
    reserved_keyword_UNTIL();
    reserved_keyword_VAR();
    reserved_keyword_WHILE();
    reserved_keyword_WITH();
    reserved_keyword_ARRAY();
    reserved_keyword_OBJECT();
    reserved_keyword_POINTER();
    reserved_keyword_RECORD();
    reserved_keyword_ADDRESS();
    reserved_keyword_SIZE();
    reserved_keyword_ALIAS();

    reserved_keyword_failure_lowercase();
    reserved_keyword_failure_ketword_addition();

    return 0;
}

// END ////////////////////////////////////////////////////////////////////////