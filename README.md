# Building a C++ compiler with Rust

## What is a compiler

> A compiler is a computer program that translates computer code written in one programming language (the source language) into another language (the target language). The name compiler is primarily used for programs that translate source code from a high-level programming language to a lower level language (e.g., assembly language, object code, or machine code) to create an executable program.
-[Wikipedia](https://en.wikipedia.org/wiki/Compiler)

In this case: translate C++ source code into x86 assembly code via a program written in Rust.

## Why this project?

Learning from first principles is important for me. There's a difference between knowing that I could possibly write a compiler if I tried and actually writing it. Right now, my understanding of computer architecture and low-level languages is based on reasoning by analogy, rather than from first principles.

> Analogies can’t replace understanding. While it’s easier on your brain to reason by analogy, you’re more likely to come up with better answers when you reason by first principles.
-[First Principles: The Building Blocks of True Knowledge](https://fs.blog/2018/04/first-principles/)

> Reasoning by first principles is useful when you are (1) doing something for the first time, (2) dealing with complexity, and (3) trying to understand a situation that you’re having problems with.

The whole system of technology, from computers, to the Internet, to the Web, is very, very complex. I hope this project helped me with the "building blocks of true knowledge".

[Nora Sandler's reasons for writing a compiler](https://norasandler.com/2017/11/29/Write-a-Compiler.html):
>1. You’ll learn about abstract syntax trees (ASTs) and how programs can represent and manipulate other programs. Handy for working with linters, static analyzers, and metaprogramming of all sorts.
>2. You’ll learn about assembly, calling conventions, and all the gritty, low-level details of how computers, like, do stuff.
>3. It seems like an impossibly hard project (but isn’t!), so writing one will make you feel like a badass.

Grace Hopper wrote the [first compiler](https://en.wikibooks.org/wiki/Introduction_to_Software_Engineering/Tools/Compiler#History) in the early 1952, which compiled A-0 source code into assembly language.

# Language Choices

## C++

Google's V8 JavaScript engine is written in C++, so learning C++ will help me understand V8 source code.

Node.js is written in C++ and part of its code ecosystem are C++ APIs.

> C++ (/ˌsiːˌplʌsˈplʌs/) is a general-purpose programming language created by Bjarne Stroustrup as an extension of the C programming language, or "C with Classes". The language has expanded significantly over time, and modern C++ now has object-oriented, generic, and functional features in addition to facilities for low-level memory manipulation. It is almost always implemented as a compiled language, and many vendors provide C++ compilers, including the Free Software Foundation, LLVM, Microsoft, Intel, Oracle, and IBM, so it is available on many platforms.
-[Wikipedia](https://en.wikipedia.org/wiki/C%2B%2B):

## Rust

> Rust is a multi-paradigm programming language focused on performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but provides memory safety without using garbage collection.

> Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others. The designers refined the language while writing the Servo layout or browser engine, and the Rust compiler. The compiler is free and open-source software dual-licensed under the MIT License and Apache License 2.0.

> Rust has been the "most loved programming language" in the Stack Overflow Developer Survey every year since 2016.
-[Wikipedia](https://en.wikipedia.org/wiki/Rust_%28programming_language%29):

> You can implement the compiler in whatever language you like, but I’d recommend using a language with sum types and pattern matching, like OCaml, Haskell, or Rust.
- [Nora Sandler, Writing a C Compiler, Part 1](https://norasandler.com/2017/11/29/Write-a-Compiler.html)

I admire [Ryan Dahl's ideal to build high-quality software systems](https://tinyclouds.org/rant.html) and I'm interested in [his new secure JavaScript and TypeScript runtime called Deno](https://deno.land/). It uses V8 and is built in Rust. I want to understand his design choices and want to read through the Deno source code.

[more on Rust](./rust.md)

# Hardware Considerations

Writing a compiler includes hardware considerations. Writing all sorts of software used to require hardware considerations, especially during the FORTRAN era.

> According to the Concise Encyclopedia of Computer Science, programming languages back [before the 1960s] such as FORTRAN were difficult to understand and dependent on the underlying hardware, with different computers like the UNIVAC I and IBM’s scientific computers unable to run the same programs.
-[Article on COBOL History](https://builtin.com/software-engineering-perspectives/why-cobol-is-still-used)

>Towards the end of the 1950s, machine-independent programming languages were first proposed. Subsequently, several experimental compilers were developed. The first compiler was written by Grace Hopper, in 1952, for the A-0 programming language. The FORTRAN team led by John Backus at IBM is generally credited as having introduced the first complete compiler in 1957. COBOL was an early language to be compiled on multiple architectures, in 1960.
-[Wikibooks Introduction to Software Engineering/Tools/Compiler](https://en.wikibooks.org/wiki/Introduction_to_Software_Engineering/Tools/Compiler#History)

## 64-bit architecture

The CPU itself needs instructions in assembly language, which it can translate to binary instructions.

The [CPU](https://www.bbc.co.uk/bitesize/guides/zhppfcw/revision/2) is made of:
- control unit (CU)
- arithmetic logic unit (ALU)
- registers
- cache
- buses
- clock

> [Registers](https://www.bbc.co.uk/bitesize/guides/zhppfcw/revision/2) are small amounts of high-speed memory contained within the CPU. They are used by the processor to store small amounts of data that are needed during processing, such as:
- the address of the next instruction to be executed
- the current instruction being decoded
- the results of calculations

The registers are small, and each register has a certain number of bits. In the 1990s and early 2000s, many processors had 32-bit architecture.

>A 32-bit system can access 232 memory addresses, i.e 4 GB of RAM or physical memory.
A 64-bit system can access 264 memory addresses, i.e actually 18-Quintillion GB of RAM. In short, any amount of memory greater than 4 GB can be easily handled by it.

>One bit in the register can reference an individual byte in memory, so a 32-bit system can address a maximum of 4 GB (4,294,967,296 bytes) of RAM.

>A 64-bit register can theoretically reference 18,446,744,073,709,551,616 bytes, or 17,179,869,184 GB (16 exabytes) of memory. This is several million times more than an average workstation would need to access. What’s important is that a 64-bit computer (which means it has a 64-bit processor) can access more than 4 GB of RAM. If a computer has 8 GB of RAM, it better have a 64-bit processor. Otherwise, at least 4 GB of the memory will be inaccessible by the CPU.

>A major difference between 32-bit processors and 64-bit processors is the number of calculations per second they can perform, which affects the speed at which they can complete tasks. 64-bit processors can come in dual core, quad core, six core, and eight core versions for home computing.
-[Geeks for Geeks](https://www.geeksforgeeks.org/difference-32-bit-64-bit-operating-systems/)

I am running macOS Catalina (10.15) and [it does not support 32-bit software](https://www.macworld.com/article/3393161/how-to-check-if-mac-software-is-32-or-64-bit.html). Previous versions of macOS, such as Mojave (10.14) did support 32-bit software. This makes my choice easy--my compiler must be 64-bit.

[More research on assembly, machine code and binary](./binaries.md)

# Architecture Overview

A typical compiler does the following steps:

>Parsing: the source text is converted to an abstract syntax tree (AST).
Resolution of references to other modules (C postpones this step till linking).
Semantic validation: weeding out syntactically correct statements that make no sense, e.g. unreachable code or duplicate declarations.
Equivalent transformations and high-level optimization: the AST is transformed to represent a more efficient computation with the same semantics. This includes e.g. early calculation of common subexpressions and constant expressions, eliminating excessive local assignments (see also SSA), etc.
Code generation: the AST is transformed into linear low-level code, with jumps, register allocation and the like. Some function calls can be inlined at this stage, some loops unrolled, etc.
Peephole optimization: the low-level code is scanned for simple local inefficiencies which are eliminated.

>Most modern compilers (for instance, gcc and clang) repeat the last two steps once more. They use an intermediate low-level but platform-independent language for initial code generation. Then that language is converted into platform-specific code (x86, ARM, etc) doing roughly the same thing in a platform-optimized way. This includes e.g. the use of vector instructions when possible, instruction reordering to increase branch prediction efficiency, and so on.
-[How to write a very basic compiler](https://softwareengineering.stackexchange.com/questions/165543/how-to-write-a-very-basic-compiler)
