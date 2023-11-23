# ActiveOberon
Native 64-bit ActiveOberon compiler for ARM v8 and x86-64 with inline assembler under Linux for now.

I am starting to write the compiler front end. That is a scanner and parser with a symbol table handler and driver for taking commands from the command line.
In addition, it will be UnitTests for all scanners and parser and symbol tables before I move on to build the backend for ARM and X86-64 and also the inline
assembler will be added later.

I am just starting this project, so you will have very little to do with the compiler for quite some time. The compiler is expected to run only under Linux at this time.

## Building project
mkdir build <br />
cd build <br />
cmake .. <br />
make <br />
ctest <br />
<br />

strip ActiveOberon <br />

## What is Active Oberon

Active Oberon is the latest member of the Algol family of languages. Derived directly from the language Oberon (-2) and languages as Modula, Pascal etc.

I will first write a complete compiler / linker / build system with inline assembler for ARM v8 and X86-64 instruction sets. With it a minimum of support
libraries as source code that will be compiled and optimized each time you create your own programs. Design goals is to remove vaste and bloated executable
that most languges and systems is used to now.

Code will be native and optimized with only the library functions you need packed into a single executable file in ELF format.

If we get enough interrest for the project, i am thinking about writing the compiler / linker / build tool in Active Oberon later, and use the C++ version as
a bootstrap for the real compiler and system. Regardless will the C++ version be as complete as the final product. It will be upto you which you will use and
all libraries will be available without changes for both.

Active Oberon is a highly typed and garbage collected language for system programming.

## Bootstrapping the system in the future

* You will first compile the c++ version of the Active Oberon compiler / linker / build tool.
* Then you will build the Active Oberon based compiler with the help of the C++ bootstrap compiler.
* Finally you will use the finished Active Oberon based compiler to build it self and you have the final product.

## Language overview

This is not a full description of the language, but a good start for understanding what the language <b> Active Oberon </b> is all about.

### Reserved keywords

<TABLE>
  <TR> <TD> ASYNC </TD> <TD> BEGIN </TD> <TD> BY </TD> <TD> CONST </TD> <TD> CASE </TD> </TR>
  <TR> <TD> CELL </TD> <TD> CELLNET </TD> <TD> CODE </TD> <TD> DO </TD> <TD> DIV </TD> </TR>
  <TR> <TD> END </TD> <TD> ENUM </TD> <TD> ELSE </TD> <TD> ELSIF </TD> <TD> EXIT </TD> </TR>
  <TR> <TD> EXTERN </TD> <TD> FALSE </TD> <TD> FOR </TD> <TD> FINALLY </TD> <TD> IF </TD> </TR>
  <TR> <TD> IGNORE </TD> <TD> IMAG </TD> <TD> IN </TD> <TD> IS </TD> <TD> IMPORT </TD> </TR>
  <TR> <TD> LOOP </TD> <TD> MODULE </TD> <TD> MOD </TD> <TD> NIL </TD> <TD> OF </TD> </TR>
  <TR> <TD> OR </TD> <TD> OUT </TD> <TD> OPERATOR </TD> <TD> PROCEDURE </TD> <TD> PORT </TD> </TR>
  <TR> <TD> REPEAT </TD> <TD> RETURN </TD> <TD> SELF </TD> <TD> NEW </TD> <TD> RESULT </TD> </TR>
  <TR> <TD> THEN </TD> <TD> TRUE </TD> <TD> TO </TD> <TD> TYPE </TD> <TD> UNTIL </TD> </TR>
  <TR> <TD> VAR </TD> <TD> WHILE </TD> <TD> WITH </TD> <TD>  </TD> <TD>  </TD> </TR>
</TABLE>

### Operatrors

### data types

