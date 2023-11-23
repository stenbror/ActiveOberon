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

Active Oberon is the latest member of the Algol family of languages. Derived directly from the language Oberon (-2) and languages such as Modula, Pascal, etc.

I will first write a complete compiler/linker/build system with an inline assembler for ARM v8 and X86-64 instruction sets. With it a minimum of support
libraries as source code that will be compiled and optimized each time you create your own programs. The design goal is to remove waste and bloated executables
that most languages and systems are used to now.

Code will be native and optimized with only the library functions you need to be packed into a single executable file in ELF format.

If we get enough interest for the project, I am thinking about writing the compiler/linker / build tool in Active Oberon later, and using the C++ version as
a bootstrap for the real compiler and system. Regardless will the C++ version be as complete as the final product? It will be up to you which you will use and
all libraries will be available without changes for both.

Active Oberon is a highly typed and garbage-collected language for system programming.

## Bootstrapping the system in the future

* You will first compile the C++ version of the Active Oberon compiler/linker / build tool.
* Then you will build the Active Oberon-based compiler with the help of the C++ bootstrap compiler.
* Finally you will use the finished Active Oberon-based compiler to build itself and you have the final product.

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

<BR /> 

<tABLE>
  <TR> <TD> ARRAY </TD> <TD> OBJECT </TD> <TD> POINTER </TD> <TD> RECORD </TD> <TD> ADDRESS </TD> <TD> SIZE </TD> <TD> ALIAS </TD> </TR>
</tABLE>

All reserved keywords must be in upper case. Code editors should support you in writing lowercase keywords and then make them uppercase automatically in your code, 
or else you will need to be friends with the caps lock on your keyboard.

### Operators and delimiters

<TABLE>
   <TR>  <TD> ( </TD> <TD> ) </TD> <TD> [ </TD> <TD> ] </TD> <TD> { </TD> <TD> } </TD> <TD> | </TD> <TD> " </TD> <TD> ' </TD> <TD> , </TD> <TD> ` </TD> </TR>
   <TR>  <TD> . </TD> <TD> .. </TD> <TD> : </TD> <TD> ; </TD> <TD> & </TD> <TD> ~ </TD> <TD> ^ </TD> <TD> ? </TD> <TD> # </TD> <TD> .#= </TD> <TD> .=& </TD> </TR>
   <TR>  <TD> &lt;  </TD> <TD> .&lt; </TD> <TD> &lt;= </TD> <TD> .&lt;= </TD> <TD> &gt; </TD> <TD> .&gt; </TD> <TD> &gt;= </TD> <TD> .&gt;= </TD> <TD> + </TD> <TD> +* </TD> <TD> - </TD> </TR>
   <TR>  <TD> * </TD> <TD> -* </TD> <TD> ** </TD> <TD> / </TD> <TD> ./ </TD> <TD> \ </TD> </TR>
</TABLE>

### Additional reserved keywords for built-in procedures and types

These can not be used for variable names etc. Just for the intended use of types and procedures provided by the system.

<TABLE>
  <TR> <TD> ABS </TD> <TD> ADDRESS </TD> <TD> ADDRESSOF </TD> <TD> ALL </TD> <TD> ANY </TD> <TD> ASH </TD> </TR>
  <TR> <TD> ASSERT </TD> <TD> BOOLEAN </TD> <TD> CAP </TD> <TD> CAS </TD> <TD> CHAR </TD> <TD> CHR </TD> </TR>
  <TR> <TD> COMPLEX </TD> <TD> COMPLEX32 </TD> <TD> COMPLEX64 </TD> <TD> COPY </TD> <TD> DEC </TD> <TD> DECMUL </TD> </TR>
  <TR> <TD> DIM </TD> <TD> ENTER </TD> <TD> ENTIERH </TD> <TD> EXCL </TD> <TD> FIRST </TD> <TD> FLOAT32 </TD> </TR>
  <TR> <TD> FLOAT64 </TD> <TD> FLOOR </TD> <TD> HALT </TD> <TD> IM </TD> <TD> INC </TD> <TD> INCL </TD> </TR>
  <TR> <TD> INCMUL </TD> <TD> INCR </TD> <TD> INTEGER </TD> <TD> INTEGERSET </TD> <TD> LAST </TD> <TD> LEN </TD> </TR>
  <TR>  <TD> LONG </TD> <TD> LONGINTEGER </TD> <TD> LSH </TD> <TD> MAX </TD> <TD> MIN </TD> <TD> OBJECT </TD> </TR>
  <TR>  <TD> ODD </TD> <TD> RANGE </TD> <TD> RE </TD> <TD> REAL </TD> <TD> RESHAPE </TD> <TD> ROL </TD> </TR>
  <TR>  <TD> ROR </TD> <TD> ROT </TD> <TD> SET </TD> <TD> SET8 </TD> <TD> SET16 </TD> <TD> SET32 </TD>  </TR>
  <TR>  <TD> SET64 </TD> <TD> SHL </TD> <TD> SHORT </TD> <TD> SHR </TD> <TD> SIGNED8 </TD> <TD> SIGNED16 </TD>  </TR>
  <TR>  <TD> SIGNED32 </TD> <TD> SIGNED64 </TD> <TD> SIZE </TD> <TD> SIZEOF </TD> <TD> STEP </TD> <TD> SUM </TD>  </TR>
  <TR>  <TD> UNSIGNED8 </TD> <TD> UNSIGNED16 </TD> <TD> UNSIGNED32 </TD> <TD> UNSIGNED64 </TD>  </TR>
</TABLE>

### The system module is built in and contains the following procedures

<TABLE>
  <TR>  <TD> SYSTEM.BYTE </TD> <TD> SYSTEM.GET </TD> <TD> SYSTEM.PUT </TD> </TR>
  <TR>  <TD> SYSTEM.PUT8 </TD> <TD> SYSTEM.PUT16 </TD> <TD> SYSTEM.PUT32 </TD> </TR>
  <TR>  <TD> SYSTEM.PUT64 </TD> <TD> SYSTEM.SET </TD> <TD> SYSTEM.GET8 </TD> </TR>
  <TR>  <TD> SYSTEM.GET16 </TD> <TD> SYSTEM.GET32 </TD> <TD> SYSTEM.GET64 </TD> </TR>
  <TR>  <TD> SYSTEM.VAL </TD> <TD> SYSTEM.MOVE </TD> <TD> SYSTEM.REF </TD> </TR>
  <TR>  <TD> SYSTEM.NEW </TD> <TD> SYSTEM.TYPECODE </TD> <TD> SYSTEM.HALT </TD> </TR>
  <TR>  <TD> SYSTEM.SIZE </TD> <TD> SYSTEM.ADR </TD> <TD> SYSTEM.MSK </TD> </TR>
  <TR>  <TD> SYSTEM.BIT </TD> <TD> SYSTEM.Time </TD> <TD> SYSTEM.Date </TD> </TR>
  <TR>  <TD> SYSTEM.GetStackPointer </TD> <TD> SYSTEM.SetStackPointer </TD> <TD> SYSTEM.GetFRameBuffer </TD> </TR>
  <TR>  <TD> SYSTEM.SetFramePointer </TD> <TD> SYSTEM.GetActivity </TD> <TD> SYSTEM.SetActivity </TD> </TR>
  <TR>  <TD> </TD> <TD> </TD> <TD> </TD> </TR>
</TABLE>


### Fundamental types

Below we describe the fundamental types available to the programmer that are directly mapped to the hardware it runs on.

<TABLE>
  <TR> <TH> Typename </TH> <TH> Size </TH> <TH> Valid values </TH> </TR>
  <TR> <TD> BOOLEAN </TD> <TD> 1 byte </TD> <TD> TRUE, FALSE </TD> </TR>
  <TR> <TD> CHAR </TD> <TD> 4 bytes </TD> <TD> UNICODE UTF32 </TD> </TR>
  <TR> <TD> SIGNED8 </TD> <TD> 1 byte </TD> <TD> -2^7 .. 2^7 - 1 </TD> </TR>
  <TR> <TD> SIGNED16 </TD> <TD> 2 bytes </TD> <TD> -2^15 .. 2^15 - 1 </TD> </TR>
  <TR> <TD> SIGNED32 </TD> <TD> 4 bytes </TD> <TD> -2^31 .. 2^31 - 1 </TD> </TR>
  <TR> <TD> SIGNED32 </TD> <TD> 8 bytes </TD> <TD> -2^63 .. 2^63 - 1 </TD> </TR>
  <TR> <TD> UNSIGNED8 </TD> <TD> 1 byte </TD> <TD> 0 .. 2^8 -1 </TD> </TR>
  <TR> <TD> UNSIGNED16 </TD> <TD> 2 bytes </TD> <TD> 0 .. 2^16 -1 </TD> </TR>
  <TR> <TD> UNSIGNED32 </TD> <TD> 4 bytes </TD> <TD> 0 .. 2^32 -1 </TD> </TR>
  <TR> <TD> UNSIGNED64 </TD> <TD> 8 bytes </TD> <TD> 0 .. 2^64 -1 </TD> </TR>
  <TR> <TD> FLOAT32 </TD> <TD> 4 bytes </TD> <TD> -3.4028^38 .. +3.4028^38 </TD> </TR>
  <TR> <TD> FLOAT64 </TD> <TD> 8 bytes </TD> <TD> -1.7976^308 .. +1.7976^308 </TD> </TR>
  <TR> <TD> SET8 </TD> <TD> 1 byte </TD> <TD> Flags between 0 and 7 </TD> </TR>
  <TR> <TD> SET16 </TD> <TD> 2 bytes </TD> <TD> Flags between 0 and 15 </TD> </TR>
  <TR> <TD> SET32 </TD> <TD> 4 bytes </TD> <TD> Flags between 0 and 31 </TD> </TR>
  <TR> <TD> SET64 </TD> <TD> 8 bytes </TD> <TD> Flags between 0 and 63 </TD> </TR>
  <TR> <TD> REAL </TD> <TD> </TD> <TD> Default floating point type. double in C </TD> </TR>
  <TR> <TD> INTEGER </TD> <TD> Machine word </TD> <TD> signed integer in machine word size </TD> </TR>
  <TR> <TD> ADDRESS </TD> <TD> Address width </TD> <TD> unsigned integer in address range </TD> </TR>
  <TR> <TD> SIZE </TD> <TD> Address width </TD> <TD> signed inteher in address range </TD> </TR>
  <TR> <TD> SET </TD> <TD> Address width </TD> <TD> Set with address width </TD> </TR>
</TABLE>

#### Literal name

All literal names must be in the form:  <I> Letter { Letter | Digit |  '_' } </I> Example: 'Build_Compiler12'

#### Numbers

Numbers are either integers or real numbers. Numbers can be decimal or hexadecimal. <BR />

*  7FF0 is a 16-bit number ( Like 0x7ff0 in C++ )
*   Real numbers in the format like <I> 0.45D-11 </I> ( FLOAT32 )
*   Real numbers in the format like <I> 7.11E+8 </I> ( FLOAT64 )

#### Charater

All characters are internally handled as UTF-32 characters, so a char is between 0 and FFFFFFFF , but only valid character codes in Unicode. Also in the format of 'x' etc.

#### Strings

All strings are arrays of UTF-32 characters. Defined as " Text goes here! "

<B> More to come later! </B>
