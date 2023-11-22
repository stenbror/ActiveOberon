# ActiveOberon
Native 64bits ActiveOberon compiler for ARM v8 and x86-64 with inline assembler.

I am starting on writing the compiler front end. That is scanner and parser with symbol table handler and driver for taking commands from command line.
In addition, it will be UnitTests for all scanner and parser and symbol table before i move on to build the backend for ARM and X86-64 and also the inline
assembler will be added later.

I am just starting this prosject, so it will be very litle to do with the compiler for quite some time. Compiler is expected to run only under Linux at this time.

## Building project
make -C build <br />
cd build <br />
cmake .. <br />
make <br />
ctest <br />
