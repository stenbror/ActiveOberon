# ActiveOberon
Native 64-bit ActiveOberon compiler for ARM v8 and x86-64 with inline assembler under Linux for now.

I am starting to write the compiler front end. That is a scanner and parser with a symbol table handler and driver for taking commands from the command line.
In addition, it will be UnitTests for all scanners and parser and symbol tables before I move on to build the backend for ARM and X86-64 and also the inline
assembler will be added later.

I am just starting this project, so it will have very little to do with the compiler for quite some time. The compiler is expected to run only under Linux at this time.

## Building project
mkdir build <br />
cd build <br />
cmake .. <br />
make <br />
ctest <br />
