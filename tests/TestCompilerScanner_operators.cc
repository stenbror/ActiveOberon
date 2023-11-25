
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

void operators_comma()
{
  auto text = std::u32string(U",");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Comma);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_transpose()
{
  auto text = std::u32string(U"`");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Transpose);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_period()
{
  auto text = std::u32string(U".");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Period);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_upto()
{
  auto text = std::u32string(U"..");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Upto);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
}

void operators_colon()
{
  auto text = std::u32string(U":");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Colon);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_semicolon()
{
  auto text = std::u32string(U";");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::SemiColon);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_and()
{
  auto text = std::u32string(U"&");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::And);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_not()
{
  auto text = std::u32string(U"~");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Not);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_arrow()
{
  auto text = std::u32string(U"^");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::Arrow);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_questionmark()
{
  auto text = std::u32string(U"?");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::QuestionMark);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_not_equal()
{
  auto text = std::u32string(U"#");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::NotEqual);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 1);
}

void operators_dot_un_equal()
{
  auto text = std::u32string(U".#");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);
    auto symb = lexer->get_symbol();    

    ASSERT_EQUAL(symb.symbol, Symbols::DotUnEqual);
    ASSERT_EQUAL(symb.start_pos, 0);
    ASSERT_EQUAL(symb.end_pos, 2);
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
  operators_comma();
  operators_transpose();
  operators_period();
  operators_upto();
  operators_colon();
  operators_semicolon();
  operators_and();
  operators_not();
  operators_arrow();
  operators_questionmark();
  operators_not_equal();
  operators_dot_un_equal();


  return 0;
}

// END ////////////////////////////////////////////////////////////////////////