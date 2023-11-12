
#include <scanner.h>
#include <memory>
#include <iostream>

using namespace ActiveOberon::Compiler;

int main()
{
    std::cout << std::endl << "\033[92mActive Oberon Compiler\033[0m, version 0.0.1 [Build: 2023-11-12] - Written by Richard Magnor Stenbro." << std::endl << std::endl;
    

    auto lexer = std::make_shared<ActiveOberonScanner>();
}