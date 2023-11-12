#!/bin/bash
clang++ -o ActiveOberon  -I. scanner.cc main.cc  -std=c++20 
strip ActiveOberon