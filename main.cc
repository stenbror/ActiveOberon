
#include <scanner.h>
#include <memory>
#include <iostream>

using namespace ActiveOberon::Compiler;

int main()
{
    std::cout << std::endl << "\033[92mActive Oberon Compiler\033[0m, version 0.0.1 [Build: 2023-11-16] - Written by Richard Magnor Stenbro." << std::endl << std::endl;
    

    auto text = std::u32string(U"  MODULE  ");
    
    auto lexer = std::make_shared<ActiveOberonScanner>(text);

    auto symb = lexer->get_symbol();

    if (symb.symbol == Symbols::Module) std::cout << " Found:: MODULE " <<std::endl;
    if (symb.start_pos == 2 && symb.end_pos == 8) std::cout << "[2::8]" << std::endl;
    
}