
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler & Risc V / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Instruction set for X86-64 module for compiling and linking of projects written in ActiveOberon language


pub const CPU_8086 : u32 = 0;
pub const CPU_186 : u32 = 1;
pub const CPU_286 : u32 = 2;
pub const CPU_386 : u32 = 4;
pub const CPU_486 : u32 = 8;
pub const CPU_PENTIUM : u32 = 16;
pub const CPU_P6 : u32 = 32;
pub const CPU_KATMAI : u32 = 64;
pub const CPU_WILLAMETTE : u32 = 128;
pub const CPU_PRESCOTT : u32 = 256;
pub const CPU_AMD64 : u32 = 512;
pub const CPU_PROTECTED : u32 = 1024;
pub const CPU_PRIVILEGED : u32 = 2048;
pub const CPU_SSE : u32 = 4096;
pub const CPU_SSE2 : u32 = 8192;
pub const CPU_SSE3 : u32 = 16384;
pub const CPU_3DNOW : u32 = 32768;
pub const CPU_MMX : u32 = 65536;
pub const CPU_FPU : u32 = 131072;

pub type CpuFlags = u32;


/// Encode a single assembler instructions with operands
fn encode_instruction_amd64(instruction: Box<String>, operands: Box<Vec<Box<String>>>, flags: CpuFlags) -> Result<Box<Vec<u8>>, Box<String>> {

    match &*instruction.as_str() {
        "AAA" => {},
        "AAD" => {},
        "AAM" => {},
        "AAS" => {},
        "ADC" => {},
        "ADD" => {},
        "ADDPD" => {},
        "ADDPS" => {},
        "ADDSD" => {},
        "ADDSS" => {},
        "ADDSUBPD" => {},
        "ADDSUBPS" => {},
        "AND" => {},
        "ANDNPD" => {},
        "ANDNPS" => {},
        "ANDPD" => {},
        "ANDPS" => {},
        "ARPL" => {},
        "BOUND" => {},
        "BSF" => {},
        "BSR" => {},
        "BSWAP" => {},
        "BT" => {},
        "BTC" => {},
        "BTR" => {},
        "BTS" => {},
        "CALL" => {},
        "CALLFAR" => {},
        "CBW" => {},
        "CDQ" => {},
        "CDQE" => {},
        "CLC" => {},
        "CLD" => {},
        "CLFLUSH" => {},
        "CLGI" => {},
        "CLI" => {},
        "CLTS" => {},
        "CMC" => {},
        "CMOVA" => {},
        "CMOVAE" => {},
        "CMOVB" => {},
        "CMOVBE" => {},
        "CMOVC" => {},
        "CMOVE" => {},
        "CMOVG" => {},
        "CMOVGE" => {},
        "CMOVL" => {},
        "CMOVLE" => {},
        "CMOVNA" => {},
        "CMOVNAE" => {},
        "CMOVNB" => {},
        "CMOVNBE" => {},
        "CMOVNC" => {},
        "CMOVNE" => {},
        "CMOVNG" => {},
        "CMOVNGE" => {},
        "CMOVNL" => {},
        "CMOVNLE" => {},
        "CMOVNO" => {},
        "CMOVNP" => {},
        "CMOVNS" => {},
        "CMOVNZ" => {},
        "CMOVO" => {},
        "CMOVP" => {},
        "CMOVPE" => {},
        "CMOVPO" => {},
        "CMOVS" => {},
        "CMOVZ" => {},
        "CMP" => {},
        "CMPPD" => {},
        "CMPPS" => {},
        "CMPS" => {},
        "CMPSB" => {},
        "CMPSD" => {},
        "CMPSQ" => {},
        "CMPSS" => {},
        "CMPSW" => {},
        "CMPXCHG" => {},
        "CMPXCHG16B" => {},
        "CMPXCHG8B" => {},
        "COMISD" => {},
        "COMISS" => {},
        "CPUID" => {},
        "CQO" => {},
        "CVTDQ2PD" => {},
        "CVTDQ2PS" => {},
        "CVTPD2DQ" => {},
        "CVTPD2PI" => {},
        "CVTPD2PS" => {},
        "CVTPI2PD" => {},
        "CVTPI2PS" => {},
        "CVTPS2DQ" => {},
        "CVTPS2PD" => {},
        "CVTPS2PI" => {},
        "CVTSD2SI" => {},
        "CVTSD2SS" => {},
        "CVTSI2SD" => {},
        "CVTSI2SS" => {},
        "CVTSS2SD" => {},
        "CVTSS2SI" => {},
        "CVTTPD2DQ" => {},
        "CVTTPD2PI" => {},
        "CVTTPS2DQ" => {},
        "CVTTPS2PI" => {},
        "CVTTSD2SI" => {},
        "CVTTSS2SI" => {},
        "CWD" => {},
        "CWDE" => {},
        "DAA" => {},
        "DAS" => {},
        "DEC" => {},
        "DIV" => {},
        "DIVPD" => {},
        "DIVPS" => {},
        "DIVSD" => {},
        "DIVSS" => {},
        "EMMS" => {},
        "ENTER" => {},
        "F2XM1" => {},
        "FABS" => {},
        "FADD" => {},
        "FADDP" => {},
        "FBLD" => {},
        "FBSTP" => {},
        "FCHS" => {},
        "FCLEX" => {},
        "FCMOVB" => {},
        "FCMOVBE" => {},
        "FCMOVE" => {},
        "FCMOVNB" => {},
        "FCMOVNBE" => {},
        "FCMOVNE" => {},
        "FCMOVNU" => {},
        "FCMOVU" => {},
        "FCOM" => {},
        "FCOMI" => {},
        "FCOMIP" => {},
        "FCOMP" => {},
        "FCOMPP" => {},
        "FCOS" => {},
        "FDECSTP" => {},
        "FDIV" => {},
        "FDIVP" => {},
        "FDIVR" => {},
        "FDIVRP" => {},
        "FEMMS" => {},
        "FFREE" => {},
        "FIADD" => {},
        "FICOM" => {},
        "FICOMP" => {},
        "FIDIV" => {},
        "FIDIVR" => {},
        "FILD" => {},
        "FIMUL" => {},
        "FINCSTP" => {},
        "FINIT" => {},
        "FIST" => {},
        "FISTP" => {},
        "FISTTP" => {},
        "FISUB" => {},
        "FISUBR" => {},
        "FLD" => {},
        "FLD1" => {},
        "FLDCW" => {},
        "FLDENV" => {},
        "FLDL2E" => {},
        "FLDL2T" => {},
        "FLDLG2" => {},
        "FLDLN2" => {},
        "FLDPI" => {},
        "FLDZ" => {},
        "FMUL" => {},
        "FMULP" => {},
        "FNCLEX" => {},
        "FNINIT" => {},
        "FN" => {},
        "FNSAVE" => {},
        "FNSTCW" => {},
        "FNSTENV" => {},
        "FNSTSW" => {},
        "FPATAN" => {},
        "FPREM" => {},
        "FPREM1" => {},
        "FPTAN" => {},
        "FRNDINT" => {},
        "FRSTOR" => {},
        "FSAVE" => {},
        "FSCALE" => {},
        "FSIN" => {},
        "FSINCOS" => {},
        "FSQRT" => {},
        "FST" => {},
        "FSTCW" => {},
        "FSTENV" => {},
        "FSTP" => {},
        "FSTSW" => {},
        "FSUB" => {},
        "FSUBP" => {},
        "FSUBR" => {},
        "FSUBRP" => {},
        "FTST" => {},
        "FUCOM" => {},
        "FUCOMI" => {},
        "FUCOMIP" => {},
        "FUCOMP" => {},
        "FUCOMPP" => {},
        "FWAIT" => {},
        "FXAM" => {},
        "FXCH" => {},
        "FXRSTOR" => {},
        "FXSAVE" => {},
        "FXTRACT" => {},
        "FYL2X" => {},
        "FYL2XP1" => {},
        "HADDPD" => {},
        "HADDPS" => {},
        "HLT" => {},
        "HSUBPD" => {},
        "HSUBPS" => {},
        "IDIV" => {},
        "IMUL" => {},
        "IN" => {},
        "INC" => {},
        "INS" => {},
        "INSB" => {},
        "INSD" => {},
        "INSW" => {},
        "INT" => {},
        "INT3" => {},
        "INTO" => {},
        "INVD" => {},
        "INVLPG" => {},
        "INVLPGA" => {},
        "IRET" => {},
        "IRETD" => {},
        "IRETQ" => {},
        "JA" => {},
        "JAE" => {},
        "JB" => {},
        "JBE" => {},
        "JC" => {},
        "JCXZ" => {},
        "JE" => {},
        "JECXZ" => {},
        "JG" => {},
        "JGE" => {},
        "JL" => {},
        "JLE" => {},
        "JMP" => {},
        "JMPFAR" => {},
        "JNA" => {},
        "JNAE" => {},
        "JNB" => {},
        "JNBE" => {},
        "JNC" => {},
        "JNE" => {},
        "JNG" => {},
        "JNGE" => {},
        "JNL" => {},
        "JNLE" => {},
        "JNO" => {},
        "JNP" => {},
        "JNS" => {},
        "JNZ" => {},
        "JO" => {},
        "JP" => {},
        "JPE" => {},
        "JPO" => {},
        "JRCXZ" => {},
        "JS" => {},
        "JZ" => {},
        "LAHF" => {},
        "LAR" => {},
        "LDDQU" => {},
        "LDMXCSR" => {},
        "LDS" => {},
        "LEA" => {},
        "LEAVE" => {},
        "LES" => {},
        "LFENCE" => {},
        "LFS" => {},
        "LGDT" => {},
        "LGS" => {},
        "LIDT" => {},
        "LLDT" => {},
        "LMSW" => {},
        "LODS" => {},
        "LODSB" => {},
        "LODSD" => {},
        "LODSQ" => {},
        "LODSW" => {},
        "LO" => {},
        "LOE" => {},
        "LONE" => {},
        "LONZ" => {},
        "LOZ" => {},
        "LSL" => {},
        "LSS" => {},
        "LTR" => {},
        "MASKMOVDQU" => {},
        "MASKMOVQ" => {},
        "MAXPD" => {},
        "MAXPS" => {},
        "MAXSD" => {},
        "MAXSS" => {},
        "MFENCE" => {},
        "MINPD" => {},
        "MINPS" => {},
        "MINSD" => {},
        "MINSS" => {},
        "MOV" => {},
        "MOVAPD" => {},
        "MOVAPS" => {},
        "MOVD" => {},
        "MOVDDUP" => {},
        "MOVDQ2Q" => {},
        "MOVDQA" => {},
        "MOVDQU" => {},
        "MOVHLPS" => {},
        "MOVHPD" => {},
        "MOVHPS" => {},
        "MOVLHPS" => {},
        "MOVLPD" => {},
        "MOVLPS" => {},
        "MOVMSKPD" => {},
        "MOVMSKPS" => {},
        "MOVNTDQ" => {},
        "MOVNTI" => {},
        "MOVNTPD" => {},
        "MOVNTPS" => {},
        "MOVNTQ" => {},
        "MOVQ" => {},
        "MOVQ2DQ" => {},
        "MOVS" => {},
        "MOVSB" => {},
        "MOVSD" => {},
        "MOVSHDUP" => {},
        "MOVSLDUP" => {},
        "MOVSQ" => {},
        "MOVSS" => {},
        "MOVSW" => {},
        "MOVSX" => {},
        "MOVSXD" => {},
        "MOVUPD" => {},
        "MOVUPS" => {},
        "MOVZX" => {},
        "MUL" => {},
        "MULPD" => {},
        "MULPS" => {},
        "MULSD" => {},
        "MULSS" => {},
        "NEG" => {},
        "N" => {},
        "NOT" => {},
        "OR" => {},
        "ORPD" => {},
        "ORPS" => {},
        "OUT" => {},
        "OUTS" => {},
        "OUTSB" => {},
        "OUTSD" => {},
        "OUTSW" => {},
        "PACKSSDW" => {},
        "PACKSSWB" => {},
        "PACKUSWB" => {},
        "PADDB" => {},
        "PADDD" => {},
        "PADDQ" => {},
        "PADDSB" => {},
        "PADDSW" => {},
        "PADDUSB" => {},
        "PADDUSW" => {},
        "PADDW" => {},
        "PAND" => {},
        "PANDN" => {},
        "PAUSE" => {},
        "PAVGB" => {},
        "PAVGUSB" => {},
        "PAVGW" => {},
        "PCMPEQB" => {},
        "PCMPEQD" => {},
        "PCMPEQW" => {},
        "PCMPGTB" => {},
        "PCMPGTD" => {},
        "PCMPGTW" => {},
        "PEXTRW" => {},
        "PF2ID" => {},
        "PF2IW" => {},
        "PFACC" => {},
        "PFADD" => {},
        "PFCMPEQ" => {},
        "PFCMPGE" => {},
        "PFCMPGT" => {},
        "PFMAX" => {},
        "PFMIN" => {},
        "PFMUL" => {},
        "PFNACC" => {},
        "PFPNACC" => {},
        "PFRCP" => {},
        "PFRCPIT1" => {},
        "PFRCPIT2" => {},
        "PFRSQIT1" => {},
        "PFRSQRT" => {},
        "PFSUB" => {},
        "PFSUBR" => {},
        "PI2FD" => {},
        "PI2FW" => {},
        "PINSRW" => {},
        "PMADDWD" => {},
        "PMAXSW" => {},
        "PMAXUB" => {},
        "PMINSW" => {},
        "PMINUB" => {},
        "PMOVMSKB" => {},
        "PMULHRW" => {},
        "PMULHUW" => {},
        "PMULHW" => {},
        "PMULLW" => {},
        "PMULUDQ" => {},
        "P" => {},
        "PA" => {},
        "PAD" => {},
        "PAW" => {},
        "PF" => {},
        "PFD" => {},
        "PFQ" => {},
        "POR" => {},
        "PREFETCH" => {},
        "PREFETCHNTA" => {},
        "PREFETCHT0" => {},
        "PREFETCHT1" => {},
        "PREFETCHT2" => {},
        "PREFETCHW" => {},
        "PSADBW" => {},
        "PSHUFD" => {},
        "PSHUFHW" => {},
        "PSHUFLW" => {},
        "PSHUFW" => {},
        "PSLLD" => {},
        "PSLLDQ" => {},
        "PSLLQ" => {},
        "PSLLW" => {},
        "PSRAD" => {},
        "PSRAW" => {},
        "PSRLD" => {},
        "PSRLDQ" => {},
        "PSRLQ" => {},
        "PSRLW" => {},
        "PSUBB" => {},
        "PSUBD" => {},
        "PSUBQ" => {},
        "PSUBSB" => {},
        "PSUBSW" => {},
        "PSUBUSB" => {},
        "PSUBUSW" => {},
        "PSUBW" => {},
        "PSWAPD" => {},
        "PUNPCKHBW" => {},
        "PUNPCKHDQ" => {},
        "PUNPCKHQDQ" => {},
        "PUNPCKHWD" => {},
        "PUNPCKLBW" => {},
        "PUNPCKLDQ" => {},
        "PUNPCKLQDQ" => {},
        "PUNPCKLWD" => {},
        "PUSH" => {},
        "PUSHA" => {},
        "PUSHAD" => {},
        "PUSHF" => {},
        "PUSHFD" => {},
        "PUSHFQ" => {},
        "PXOR" => {},
        "RCL" => {},
        "RCPPS" => {},
        "RCPSS" => {},
        "RCR" => {},
        "RDMSR" => {},
        "RDPMC" => {},
        "RDTSC" => {},
        "RDTSCP" => {},
        "RET" => {},
        "RETF" => {},
        "ROL" => {},
        "ROR" => {},
        "RSM" => {},
        "RSQRTPS" => {},
        "RSQRTSS" => {},
        "SAHF" => {},
        "SAL" => {},
        "SAR" => {},
        "SBB" => {},
        "SCAS" => {},
        "SCASB" => {},
        "SCASD" => {},
        "SCASQ" => {},
        "SCASW" => {},
        "SETA" => {},
        "SETAE" => {},
        "SETB" => {},
        "SETBE" => {},
        "SETC" => {},
        "SETE" => {},
        "SETG" => {},
        "SETGE" => {},
        "SETL" => {},
        "SETLE" => {},
        "SETNA" => {},
        "SETNAE" => {},
        "SETNB" => {},
        "SETNBE" => {},
        "SETNC" => {},
        "SETNE" => {},
        "SETNG" => {},
        "SETNGE" => {},
        "SETNL" => {},
        "SETNLE" => {},
        "SETNO" => {},
        "SETNP" => {},
        "SETNS" => {},
        "SETNZ" => {},
        "SETO" => {},
        "SETP" => {},
        "SETPE" => {},
        "SETPO" => {},
        "SETS" => {},
        "SETZ" => {},
        "SFENCE" => {},
        "SGDT" => {},
        "SHL" => {},
        "SHLD" => {},
        "SHR" => {},
        "SHRD" => {},
        "SHUFPD" => {},
        "SHUFPS" => {},
        "SIDT" => {},
        "SKINIT" => {},
        "SLDT" => {},
        "SMSW" => {},
        "SQRTPD" => {},
        "SQRTPS" => {},
        "SQRTSD" => {},
        "SQRTSS" => {},
        "STC" => {},
        "STD" => {},
        "STGI" => {},
        "STI" => {},
        "STMXCSR" => {},
        "STOS" => {},
        "STOSB" => {},
        "STOSD" => {},
        "STOSQ" => {},
        "STOSW" => {},
        "STR" => {},
        "SUB" => {},
        "SUBPD" => {},
        "SUBPS" => {},
        "SUBSD" => {},
        "SUBSS" => {},
        "SWAPGS" => {},
        "SYSCALL" => {},
        "SYSENTER" => {},
        "SYSEXIT" => {},
        "SYSRET" => {},
        "TEST" => {},
        "UCOMISD" => {},
        "UCOMISS" => {},
        "UD2" => {},
        "UNPCKHPD" => {},
        "UNPCKHPS" => {},
        "UNPCKLPD" => {},
        "UNPCKLPS" => {},
        "VERR" => {},
        "VERW" => {},
        "VMLOAD" => {},
        "VMMCALL" => {},
        "VMRUN" => {},
        "VMSAVE" => {},
        "WBINVD" => {},
        "WRMSR" => {},
        "XADD" => {},
        "XCHG" => {},
        "XLAT" => {},
        "XLATB" => {},
        "XOR" => {},
        "XORPD" => {},
        "XORPS" => {},
        "VAESKEYGENASSIST" => {},
        "VPMADCSWD" => {},
        "VADDPD" => {},
        "VADDPS" => {},
        "VADDSD" => {},
        "VADDSS" => {},
        "VADDSUBPD" => {},
        "VADDSUBPS" => {},
        "VAESDEC" => {},
        "VAESDECLAST" => {},
        "VAESENC" => {},
        "VAESENCLAST" => {},
        "VAESIMC" => {},
        "VANDNPD" => {},
        "VANDNPS" => {},
        "VANDPD" => {},
        "VANDPS" => {},
        "VBLENDPD" => {},
        "VBLENDPS" => {},
        "VBLENDVPD" => {},
        "VBLENDVPS" => {},
        "VBROADCASTF128" => {},
        "VBROADCASTI128" => {},
        "VBROADCASTSD" => {},
        "VBROADCASTSS" => {},
        "VCMPPD" => {},
        "VCMPPS" => {},
        "VCMPSD" => {},
        "VCMPSS" => {},
        "VCOMISD" => {},
        "VCOMISS" => {},
        "VCVTDQ2PD" => {},
        "VCVTDQ2PS" => {},
        "VCVTPD2DQ" => {},
        "VCVTPD2PS" => {},
        "VCVTPH2PS" => {},
        "VCVTPS2DQ" => {},
        "VCVTPS2PD" => {},
        "VCVTPS2PH" => {},
        "VCVTSD2SI" => {},
        "VCVTSD2SS" => {},
        "VCVTSI2SD" => {},
        "VCVTSI2SS" => {},
        "VCVTSS2SD" => {},
        "VCVTSS2SI" => {},
        "VCVTTPD2DQ" => {},
        "VCVTTPS2DQ" => {},
        "VCVTTSD2SI" => {},
        "VCVTTSS2SI" => {},
        "VDIVPD" => {},
        "VDIVPS" => {},
        "VDIVSD" => {},
        "VDIVSS" => {},
        "VDPPD" => {},
        "VDPPS" => {},
        "VEXTRACTF128" => {},
        "VEXTRACTI128" => {},
        "VEXTRACTPS" => {},
        "VFMADD132PD" => {},
        "VFMADD132PS" => {},
        "VFMADD132SD" => {},
        "VFMADD132SS" => {},
        "VFMADD213PD" => {},
        "VFMADD213PS" => {},
        "VFMADD213SD" => {},
        "VFMADD213SS" => {},
        "VFMADD231PD" => {},
        "VFMADD231PS" => {},
        "VFMADD231SD" => {},
        "VFMADD231SS" => {},
        "VFMADDPD" => {},
        "VFMADDPS" => {},
        "VFMADDSD" => {},
        "VFMADDSS" => {},
        "VFMADDSUB132PD" => {},
        "VFMADDSUB132PS" => {},
        "VFMADDSUB213PD" => {},
        "VFMADDSUB213PS" => {},
        "VFMADDSUB231PD" => {},
        "VFMADDSUB231PS" => {},
        "VFMADDSUBPD" => {},
        "VFMADDSUBPS" => {},
        "VFMSUB132PD" => {},
        "VFMSUB132PS" => {},
        "VFMSUB132SD" => {},
        "VFMSUB132SS" => {},
        "VFMSUB213PD" => {},
        "VFMSUB213PS" => {},
        "VFMSUB213SD" => {},
        "VFMSUB213SS" => {},
        "VFMSUB231PD" => {},
        "VFMSUB231PS" => {},
        "VFMSUB231SD" => {},
        "VFMSUB231SS" => {},
        "VFMSUBADD132PD" => {},
        "VFMSUBADD132PS" => {},
        "VFMSUBADD213PD" => {},
        "VFMSUBADD213PS" => {},
        "VFMSUBADD231PD" => {},
        "VFMSUBADD231PS" => {},
        "VFMSUBADDPD" => {},
        "VFMSUBADDPS" => {},
        "VFMSUBPD" => {},
        "VFMSUBPS" => {},
        "VFMSUBSD" => {},
        "VFMSUBSS" => {},
        "VFNMADD132PD" => {},
        "VFNMADD132PS" => {},
        "VFNMADD132SD" => {},
        "VFNMADD132SS" => {},
        "VFNMADD213PD" => {},
        "VFNMADD213PS" => {},
        "VFNMADD213SD" => {},
        "VFNMADD213SS" => {},
        "VFNMADD231PD" => {},
        "VFNMADD231PS" => {},
        "VFNMADD231SD" => {},
        "VFNMADD231SS" => {},
        "VFNMADDPD" => {},
        "VFNMADDPS" => {},
        "VFNMADDSD" => {},
        "VFNMADDSS" => {},
        "VFNMSUB132PD" => {},
        "VFNMSUB132PS" => {},
        "VFNMSUB132SD" => {},
        "VFNMSUB132SS" => {},
        "VFNMSUB213PD" => {},
        "VFNMSUB213PS" => {},
        "VFNMSUB213SD" => {},
        "VFNMSUB213SS" => {},
        "VFNMSUB231PD" => {},
        "VFNMSUB231PS" => {},
        "VFNMSUB231SD" => {},
        "VFNMSUB231SS" => {},
        "VFNMSUBPD" => {},
        "VFNMSUBPS" => {},
        "VFNMSUBSD" => {},
        "VFNMSUBSS" => {},
        "VFRCZPD" => {},
        "VFRCZPS" => {},
        "VFRCZSD" => {},
        "VFRCZSS" => {},
        "VGATHERDPD" => {},
        "VGATHERDPS" => {},
        "VGATHERQPD" => {},
        "VGATHERQPS" => {},
        "VHADDPD" => {},
        "VHADDPS" => {},
        "VHSUBPD" => {},
        "VHSUBPS" => {},
        "VINSERTF128" => {},
        "VINSERTI128" => {},
        "VINSERTPS" => {},
        "VLDDQU" => {},
        "VLDMXCSR" => {},
        "VMASKMOVDQU" => {},
        "VMASKMOVPD" => {},
        "VMASKMOVPS" => {},
        "VMAXPD" => {},
        "VMAXPS" => {},
        "VMAXSD" => {},
        "VMAXSS" => {},
        "VMINPD" => {},
        "VMINPS" => {},
        "VMINSD" => {},
        "VMINSS" => {},
        "VMOVAPD" => {},
        "VMOVAPS" => {},
        "VMOVD" => {},
        "VMOVDDUP" => {},
        "VMOVDQA" => {},
        "VMOVDQU" => {},
        "VMOVHLPS" => {},
        "VMOVHPD" => {},
        "VMOVHPS" => {},
        "VMOVLHPS" => {},
        "VMOVLPD" => {},
        "VMOVLPS" => {},
        "VPMOVMSKB" => {},
        "VMOVMSKPD" => {},
        "VMOVMSKPS" => {},
        "VMOVNTDQ" => {},
        "VMOVNTDQA" => {},
        "VMOVNTPD" => {},
        "VMOVNTPS" => {},
        "VMOVQ" => {},
        "VMOVSD" => {},
        "VMOVSHDUP" => {},
        "VMOVSLDUP" => {},
        "VMOVSS" => {},
        "VMOVUPD" => {},
        "VMOVUPS" => {},
        "VMPSADBW" => {},
        "VMULPD" => {},
        "VMULPS" => {},
        "VMULSD" => {},
        "VMULSS" => {},
        "VORPD" => {},
        "VORPS" => {},
        "VPABSB" => {},
        "VPABSD" => {},
        "VPABSW" => {},
        "VPACKSSDW" => {},
        "VPACKSSWB" => {},
        "VPACKUSDW" => {},
        "VPACKUSWB" => {},
        "VPADDB" => {},
        "VPADDD" => {},
        "VPADDQ" => {},
        "VPADDSB" => {},
        "VPADDSW" => {},
        "VPADDUSB" => {},
        "VPADDUSW" => {},
        "VPADDW" => {},
        "VPALIGNR" => {},
        "VPAND" => {},
        "VPANDN" => {},
        "VPAVGB" => {},
        "VPAVGW" => {},
        "VPBLENDD" => {},
        "VPBLENDVB" => {},
        "VPBLENDW" => {},
        "VPBROADCASTB" => {},
        "VPBROADCASTD" => {},
        "VPBROADCASTQ" => {},
        "VPBROADCASTW" => {},
        "VPCLMULQDQ" => {},
        "VPCMOV" => {},
        "VPCMPEQB" => {},
        "VPCMPEQD" => {},
        "VPCMPEQQ" => {},
        "VPCMPEQW" => {},
        "VPCMPESTRI" => {},
        "VPCMPESTRM" => {},
        "VPCMPGTB" => {},
        "VPCMPGTD" => {},
        "VPCMPGTQ" => {},
        "VPCMPGTW" => {},
        "VPCMPISTRI" => {},
        "VPCMPISTRM" => {},
        "VPCOMB" => {},
        "VPCOMD" => {},
        "VPCOMQ" => {},
        "VPCOMUB" => {},
        "VPCOMUD" => {},
        "VPCOMUQ" => {},
        "VPCOMUW" => {},
        "VPCOMW" => {},
        "VPERM2F128" => {},
        "VPERM2I128" => {},
        "VPERMD" => {},
        "VPERMIL2PD" => {},
        "VPERMIL2PS" => {},
        "VPERMILPD" => {},
        "VPERMILPS" => {},
        "VPERMPD" => {},
        "VPERMPS" => {},
        "VPERMQ" => {},
        "VPEXTRB" => {},
        "VPEXTRD" => {},
        "VPEXTRQ" => {},
        "VPEXTRW" => {},
        "VPGATHERDD" => {},
        "VPGATHERDQ" => {},
        "VPGATHERQD" => {},
        "VPGATHERQQ" => {},
        "VPHADDBD" => {},
        "VPHADDBQ" => {},
        "VPHADDBW" => {},
        "VPHADDD" => {},
        "VPHADDDQ" => {},
        "VPHADDSW" => {},
        "VPHADDUBD" => {},
        "VPHADDUBQ" => {},
        "VPHADDUBW" => {},
        "VPHADDUDQ" => {},
        "VPHADDUWD" => {},
        "VPHADDUWQ" => {},
        "VPHADDW" => {},
        "VPHADDWD" => {},
        "VPHADDWQ" => {},
        "VPHMINPOSUW" => {},
        "VPHSUBBW" => {},
        "VPHSUBD" => {},
        "VPHSUBDQ" => {},
        "VPHSUBSW" => {},
        "VPHSUBW" => {},
        "VPHSUBWD" => {},
        "VPINSRB" => {},
        "VPINSRD" => {},
        "VPINSRQ" => {},
        "VPINSRW" => {},
        "VPMACSDD" => {},
        "VPMACSDQH" => {},
        "VPMACSDQL" => {},
        "VPMACSSDD" => {},
        "VPMACSSDQH" => {},
        "VPMACSSDQL" => {},
        "VPMACSSWD" => {},
        "VPMACSSWW" => {},
        "VPMACSWD" => {},
        "VPMACSWW" => {},
        "VPMADCSSWD" => {},
        "VPMADDUBSW" => {},
        "VPMADDWD" => {},
        "VPMASKMOVD" => {},
        "VPMASKMOVQ" => {},
        "VPMAXSB" => {},
        "VPMAXSD" => {},
        "VPMAXSW" => {},
        "VPMAXUB" => {},
        "VPMAXUD" => {},
        "VPMAXUW" => {},
        "VPMINSB" => {},
        "VPMINSD" => {},
        "VPMINSW" => {},
        "VPMINUB" => {},
        "VPMINUD" => {},
        "VPMINUW" => {},
        "VPMOVSXBD" => {},
        "VPMOVSXBQ" => {},
        "VPMOVSXBW" => {},
        "VPMOVSXDQ" => {},
        "VPMOVSXWD" => {},
        "VPMOVSXWQ" => {},
        "VPMOVZXBD" => {},
        "VPMOVZXBQ" => {},
        "VPMOVZXBW" => {},
        "VPMOVZXDQ" => {},
        "VPMOVZXWD" => {},
        "VPMOVZXWQ" => {},
        "VPMULDQ" => {},
        "VPMULHRSW" => {},
        "VPMULHUW" => {},
        "VPMULHW" => {},
        "VPMULLD" => {},
        "VPMULLW" => {},
        "VPMULUDQ" => {},
        "VPOR" => {},
        "VPPERM" => {},
        "VPROTB" => {},
        "VPROTD" => {},
        "VPROTQ" => {},
        "VPROTW" => {},
        "VPSADBW" => {},
        "VPSHAB" => {},
        "VPSHAD" => {},
        "VPSHAQ" => {},
        "VPSHAW" => {},
        "VPSHLB" => {},
        "VPSHLD" => {},
        "VPSHLQ" => {},
        "VPSHLW" => {},
        "VPSHUFB" => {},
        "VPSHUFD" => {},
        "VPSHUFHW" => {},
        "VPSHUFLW" => {},
        "VPSIGNB" => {},
        "VPSIGND" => {},
        "VPSIGNW" => {},
        "VPSLLD" => {},
        "VPSLLDQ" => {},
        "VPSLLQ" => {},
        "VPSLLVD" => {},
        "VPSLLVQ" => {},
        "VPSLLW" => {},
        "VPSRAD" => {},
        "VPSRAVD" => {},
        "VPSRAW" => {},
        "VPSRLD" => {},
        "VPSRLDQ" => {},
        "VPSRLQ" => {},
        "VPSRLVD" => {},
        "VPSRLVQ" => {},
        "VPSRLW" => {},
        "VPSUBB" => {},
        "VPSUBD" => {},
        "VPSUBQ" => {},
        "VPSUBSB" => {},
        "VPSUBSW" => {},
        "VPSUBUSB" => {},
        "VPSUBUSW" => {},
        "VPSUBW" => {},
        "VPTEST" => {},
        "VPUNPCKHBW" => {},
        "VPUNPCKHDQ" => {},
        "VPUNPCKHQDQ" => {},
        "VPUNPCKHWD" => {},
        "VPUNPCKLBW" => {},
        "VPUNPCKLDQ" => {},
        "VPUNPCKLQDQ" => {},
        "VPUNPCKLWD" => {},
        "VPXOR" => {},
        "VRCPPS" => {},
        "VRCPSS" => {},
        "VROUNDPD" => {},
        "VROUNDPS" => {},
        "VROUNDSD" => {},
        "VROUNDSS" => {},
        "VRSQRTPS" => {},
        "VRSQRTSS" => {},
        "VSHUFPD" => {},
        "VSHUFPS" => {},
        "VSQRTPD" => {},
        "VSQRTPS" => {},
        "VSQRTSD" => {},
        "VSQRTSS" => {},
        "VSTMXCSR" => {},
        "VSUBPD" => {},
        "VSUBPS" => {},
        "VSUBSD" => {},
        "VSUBSS" => {},
        "VTESTPD" => {},
        "VTESTPS" => {},
        "VUCOMISD" => {},
        "VUCOMISS" => {},
        "VUNPCKHPD" => {},
        "VUNPCKHPS" => {},
        "VUNPCKLPD" => {},
        "VUNPCKLPS" => {},
        "VXORPD" => {},
        "VXORPS" => {},
        "VZEROALL" => {},
        "VZEROUPPER" => {},

        _ => return Err(Box::new(String::from("Illegal instruction!")))
    }

    Ok(Box::new(Vec::<u8>::new()))
}

/// Decode a single array of bytes to assembler instruction with operands
fn decode_instruction_amd64(code: Box<Vec<u8>>, flags: CpuFlags) -> Result<Box<String>, Box<String>>{
    Ok(Box::new(String::new()))
}