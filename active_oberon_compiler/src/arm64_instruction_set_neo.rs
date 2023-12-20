
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Instruction set for ARM v8 module for compiling and linking of projects written in ActiveOberon language

type CpuFlags = u32;

fn encode_instruction_arm64(instruction: Box<String>, operands: Box<Vec<Box<String>>>, flags: CpuFlags) -> Result<Box<Vec<u8>>, Box<String>> {

    match &*instruction.as_str() {
        "ADC" => {},
        "ADD" => {},
        "AND" => {},
        "B" => {},
        "BIC" => {},
        "BKPT" => {},
        "BL" => {},
        "BLX" => {},
        "BX" => {},
        "CDP" => {},
        "CLZ" => {},
        "CMN" => {},
        "CMP" => {},
        "EOR" => {},
        "FABSD" => {},
        "FABSS" => {},
        "FADDD" => {},
        "FADDS" => {},
        "FCMPD" => {},
        "FCMPED" => {},
        "FCMPES" => {},
        "FCMPEZD" => {},
        "FCMPEZS" => {},
        "FCMPS" => {},
        "FCMPZD" => {},
        "FCMPZS" => {},
        "FCPYD" => {},
        "FCPYS" => {},
        "FCVTDS" => {},
        "FCVTSD" => {},
        "FDIVD" => {},
        "FDIVS" => {},
        "FLDD" => {},
        "FLDMIAD" => {},
        "FLDMIAS" => {},
        "FLDMIAX" => {},
        "FLDMDBD" => {},
        "FLDMDBS" => {},
        "FLDMDBX" => {},
        "FLDS" => {},
        "FMACD" => {},
        "FMACS" => {},
        "FMDHR" => {},
        "FMDLR" => {},
        "FMRDH" => {},
        "FMRDL" => {},
        "FMRS" => {},
        "FMRX" => {},
        "FMSCD" => {},
        "FMSCS" => {},
        "FMSR" => {},
        "FMSTAT" => {},
        "FMULD" => {},
        "FMULS" => {},
        "FMXR" => {},
        "FNEGD" => {},
        "FNEGS" => {},
        "FNMACD" => {},
        "FNMACS" => {},
        "FNMSCD" => {},
        "FNMSCS" => {},
        "FNMULD" => {},
        "FNMULS" => {},
        "FSITOD" => {},
        "FSITOS" => {},
        "FSQRTD" => {},
        "FSQRTS" => {},
        "FSTD" => {},
        "FSTMIAD" => {},
        "FSTMIAS" => {},
        "FSTMIAX" => {},
        "FSTMDBD" => {},
        "FSTMDBS" => {},
        "FSTMDBX" => {},
        "FSTS" => {},
        "FSUBD" => {},
        "FSUBS" => {},
        "FTOSID" => {},
        "FTOSIZD" => {},
        "FTOSIS" => {},
        "FTOSIZS" => {},
        "FTOUID" => {},
        "FTOUIZD" => {},
        "FTOUIS" => {},
        "FTOUIZS" => {},
        "FUITOD" => {},
        "FUITOS" => {},
        "LDC" => {},
        "LDM" => {},
        "LDR" => {},
        "MCR" => {},
        "MCR" => {},
        "MCRR" => {},
        "MLA" => {},
        "MOV" => {},
        "MRC" => {},
        "MRC" => {},
        "MRRC" => {},
        "MRS" => {},
        "MSR" => {},
        "MUL" => {},
        "MVN" => {},
        "ORR" => {},
        "PLD" => {},
        "QADD" => {},
        "QDADD" => {},
        "QDSUB" => {},
        "QSUB" => {},
        "RSB" => {},
        "RSC" => {},
        "SBC" => {},
        "SMLABB" => {},
        "SMLABT" => {},
        "SMLAL" => {},
        "SMLATB" => {},
        "SMLATT" => {},
        "SMLALBB" => {},
        "SMLALBT" => {},
        "SMLALTB" => {},
        "SMLALTT" => {},
        "SMLAWB" => {},
        "SMLAWT" => {},
        "SMULBB" => {},
        "SMULBT" => {},
        "SMULTB" => {},
        "SMULTT" => {},
        "SMULWB" => {},
        "SMULWT" => {},
        "SMULL" => {},
        "STC" => {},
        "STM" => {},
        "STR" => {},
        "SUB" => {},
        "SWI" => {},
        "SWP" => {},
        "TEQ" => {},
        "TST" => {},
        "UMLAL" => {},
        "UMULL" => {},
        "ISB" => {},
        "VADD" => {},
        "VADDL" => {},
        "VADDW" => {},
        "VMUL" => {},
        "VMULL" => {},
        "VMSR" => {},
        "VMRS" => {},
        "VLDR" => {},
        "VSTR" => {},
        "VDIV" => {},
        "VMLA" => {},
        "VMLS" => {},
        "VMIN" => {},
        "VMAX" => {},
        "VSUB" => {},
        "VABS" => {},
        "VABD" => {},
        "LSL" => {},
        "LSR" => {},
        "VLD" => {},
        "VST" => {},
        "VPADD" => {},
        "VMOV" => {},
        "SEV" => {},
        "DSB" => {},
        "LDREX" => {},
        "STREX" => {},
        "ADR" => {},
        "LDREXB" => {},
        "STREXB" => {},
        "DMB" => {},
        "CLREX" => {},
        "REV" => {},
        "UXTH" => {},
        "WFE" => {},
        "WFI" => {},
        "MOVW" => {},
        "UDF" => {},
        _ => return Err(Box::new(String::from("Illegal instruction!")))
    }

    Ok(Box::new(Vec::<u8>::new()))
}

fn decode_instruction_arm64(code: Box<Vec<u8>>, flags: CpuFlags) -> Result<Box<String>, Box<String>>{
    Ok(Box::new(String::new()))
}