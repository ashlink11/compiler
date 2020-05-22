## Research on assembly, machine code, binaries and binary format

## Assembly language

>x86 Assembly Language is a family of backward-compatible assembly languages, which provide some level of compatibility all the way back to the Intel 8008 introduced in April 1972. x86 assembly languages are used to produce object code for the x86 class of processors. Like all assembly languages, it uses short mnemonics to represent the fundamental instructions that the CPU in a computer can understand and follow. Compilers sometimes produce assembly code as an intermediate step when translating a high level program into machine code. Regarded as a programming language, assembly coding is machine-specific and low level. Assembly languages are more typically used for detailed and time critical applications such as small real-time embedded systems or operating system kernels and device drivers.
-[x86 Assembly Language](https://en.wikipedia.org/wiki/X86_assembly_language)

>x86-64 (also known as x64, x86_64, AMD64 and Intel 64)[note 1] is the 64-bit version of the x86 instruction set.

>Intel 64 is Intel's implementation of x86-64, used and implemented in various processors made by Intel.

>Intel's processors implementing the Intel64 architecture include the Pentium 4 F-series/5x1 series, 506, and 516, Celeron D models 3x1, 3x6, 355, 347, 352, 360, and 365 and all later Celerons, all models of Xeon since "Nocona", all models of Pentium Dual-Core processors since "Merom-2M", the Atom 230, 330, D410, D425, D510, D525, N450, N455, N470, N475, N550, N570, N2600 and N2800, all versions of the Pentium D, Pentium Extreme Edition, Core 2, Core i9, Core i7, Core i5, and Core i3 processors, and the Xeon Phi 7200 series processors.
-[x86-64](https://en.wikipedia.org/wiki/X86-64)

MacBook Pros of this generation often use Intel Core i5, i7 & i9 chips.

>Differences between AMD64 and Intel 64
Although nearly identical, there are some differences between the two instruction sets in the semantics of a few seldom used machine instructions (or situations), which are mainly used for system programming.[38] Compilers generally produce executables (i.e. machine code) that avoid any differences, at least for ordinary application programs. This is therefore of interest mainly to developers of compilers, operating systems and similar, which must deal with individual and special system instructions.

>Recent implementations
>Intel 64's BSF and BSR instructions act differently than AMD64's when the source is zero and the operand size is 32 bits. The processor sets the zero flag and leaves the upper 32 bits of the destination undefined.
AMD64 requires a different microcode update format and control MSRs (model-specific registers) while Intel 64 implements microcode update unchanged from their 32-bit only processors.
Intel 64 lacks some MSRs that are considered architectural in AMD64. These include SYSCFG, TOP_MEM, and TOP_MEM2.
Intel 64 allows SYSCALL/SYSRET only in 64-bit mode (not in compatibility mode),[39] and allows SYSENTER/SYSEXIT in both modes.[40] AMD64 lacks SYSENTER/SYSEXIT in both sub-modes of long mode.[11]:33
In 64-bit mode, near branches with the 66H (operand size override) prefix behave differently. Intel 64 ignores this prefix: the instruction has 32-bit sign extended offset, and instruction pointer is not truncated. AMD64 uses 16-bit offset field in the instruction, and clears the top 48 bits of instruction pointer.
AMD processors raise a floating point Invalid Exception when performing an FLD or FSTP of an 80-bit signalling NaN, while Intel processors do not.
Intel 64 lacks the ability to save and restore a reduced (and thus faster) version of the floating-point state (involving the FXSAVE and FXRSTOR instructions).
AMD processors ever since Opteron Rev. E and Athlon 64 Rev. D have reintroduced limited support for segmentation, via the Long Mode Segment Limit Enable (LMSLE) bit, to ease virtualization of 64-bit guests.[41][42]
When returning to a non-canonical address using SYSRET, AMD64 processors execute the general protection fault handler in privilege level 3,[43] while on Intel 64 processors it is executed in privilege level 0.[44]

>macOS 10.15 includes only the 64-bit kernel and no longer supports 32-bit applications.

>The 64-bit kernel does not support 32-bit kernel extensions, and the 32-bit kernel does not support 64-bit kernel extensions.

>macOS uses the universal binary format to package 32- and 64-bit versions of application and library code into a single file; the most appropriate version is automatically selected at load time. In Mac OS X 10.6, the universal binary format is also used for the kernel and for those kernel extensions that support both 32-bit and 64-bit kernels.

>Apple macOS refers to 64-bit architecture as "x86-64" or "x86_64", as seen in the Terminal command arch[3] and in their developer documentation.[2][4]

Additional resources:
 - [Apple Developer Docs x86-64 Code Model](https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/MachOTopics/1-Articles/x86_64_code.html)

## Machine code

Machine code is machine-dependent, i.e. depends on the architecture of the processor.

## Binaries

In the root directory `/` of my computer, there is a `bin` folder, and that's where "essential command binaries" live. Binaries are executable code, as opposed to data.

> Binary files are usually thought of as being a sequence of bytes, which means the binary digits (bits) are grouped in eights. Binary files typically contain bytes that are intended to be interpreted as something other than text characters. Compiled computer programs are typical examples; indeed, compiled applications are sometimes referred to, particularly by programmers, as binaries.
- [Binary file](https://en.wikipedia.org/wiki/Binary_file)

## Execution of a program

When assembly code is executed, its turned into machine code, then bytecode, so the CPU can process it. Instructions are loaded into the RAM, get put into the registers, and are processed by the ALU and CU, while the clock ticks along and the instruction counter increments.
