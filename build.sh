#!/bin/bash
clang++ -o ActiveOberon  -I. parser.cc scanner.cc main.cc  -std=c++20 
strip ActiveOberon