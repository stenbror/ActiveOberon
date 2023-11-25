cmake_minimum_required(VERSION 3.14...3.27)

set(CMAKE_CXX_COMPILER "/usr/bin/clang++")
set(CMAKE_CXX_STANDARD_REQUIRED "c++-20")


target_include_directories(ActiveOberon PUBLIC ../)
set(CMAKE_CXX_FLAGS "-I.. -std=c++20")

set(HEADER_FILES ../*.h)

add_executable(TestCompilerScanner_Operators TestCompilerScanner_operators.cc ../scanner.cc)
add_executable(TestCompilerScanner_ReservedKeywords TestCompilerScanner_reserved_keywords.cc ../scanner.cc) 