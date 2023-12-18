
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Instruction set for X86-64 module for compiling and linking of projects written in ActiveOberon language


const CPU_8086 : u32 = 0;
const CPU_186 : u32 = 1;
const CPU_286 : u32 = 2;
const CPU_386 : u32 = 4;
const CPU_486 : u32 = 8;
const CPU_PENTIUM : u32 = 16;
const CPU_P6 : u32 = 32;
const CPU_KATMAI : u32 = 64;
const CPU_WILLAMETTE : u32 = 128;
const CPU_PRESCOTT : u32 = 256;
const CPU_AMD64 : u32 = 512;
const CPU_PROTECTED : u32 = 1024;
const CPU_PRIVILEGED : u32 = 2048;
const CPU_SSE : u32 = 4096;
const CPU_SSE2 : u32 = 8192;
const CPU_SSE3 : u32 = 16384;
const CPU_3DNOW : u32 = 32768;
const CPU_MMX : u32 = 65536;
const CPU_FPU : u32 = 131072;

type CpuFlags = u32;


/// Encode a single assembler instructions with operands
fn encode_instruction_amd64(instruction: Box<String>, operands: Box<Vec<Box<String>>>, flags: CpuFlags) -> Result<Box<Vec<u8>>, Box<String>> {

    Ok(Box::new(Vec::<u8>::new()))
}

/// Decode a single array of bytes to assembler instruction with operands
fn decode_instruction_amd64(code: Box<Vec<u8>>, flags: CpuFlags) -> Result<Box<String>, Box<String>>{
    Ok(Box::new(String::new()))
}