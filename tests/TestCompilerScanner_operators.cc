
#include <memory>

#include "assertions_macros.h"

#include "../scanner.h"

using namespace ActiveOberon::Compiler;

// UnitTests //////////////////////////////////////////////////////////////////

void operators_left_parenthesis()
{
  auto text = std::u32string(U"(");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::LeftParen);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_right_parenthesis()
{
  auto text = std::u32string(U")");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::RightParen);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_left_bracket()
{
  auto text = std::u32string(U"[");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::LeftBracket);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_right_bracket()
{
  auto text = std::u32string(U"]");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::RightBracket);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_left_brace()
{
  auto text = std::u32string(U"{");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::LeftBrace);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_right_brace()
{
  auto text = std::u32string(U"}");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::RightBrace);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_bar()
{
  auto text = std::u32string(U"|");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Bar);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

// Test harness for reserved keywords /////////////////////////////////////////

int main() {

  operators_left_parenthesis();
  operators_right_parenthesis();
  operators_left_bracket();
  operators_right_bracket();
  operators_left_brace();
  operators_right_brace();
  operators_bar();
  
  return 0;
}

// END ////////////////////////////////////////////////////////////////////////