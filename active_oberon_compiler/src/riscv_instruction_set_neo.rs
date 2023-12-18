
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Instruction set for Risc V module for compiling and linking of projects written in ActiveOberon language

type CpuFlags = u32;

fn encode_instruction_risc_v(instruction: Box<String>, operands: Box<Vec<Box<String>>>, flags: CpuFlags) -> Result<Box<Vec<u8>>, Box<String>> {

    Ok(Box::new(Vec::<u8>::new()))
}

fn decode_instruction_risc_v(code: Box<Vec<u8>>, flags: CpuFlags) -> Result<Box<String>, Box<String>>{
    Ok(Box::new(String::new()))
}