# Building a C++ compiler with Rust

# Why this project?

Learning from first principles is important for me. There's a difference between knowing that I could possibly write a compiler if I tried and actually writing it. Right now, my understanding of computer architecture and low-level languages is based on reasoning by analogy, rather than from first principles.

> Analogies can’t replace understanding. While it’s easier on your brain to reason by analogy, you’re more likely to come up with better answers when you reason by first principles.
- [First Principles: The Building Blocks of True Knowledge](https://fs.blog/2018/04/first-principles/)

> Reasoning by first principles is useful when you are (1) doing something for the first time, (2) dealing with complexity, and (3) trying to understand a situation that you’re having problems with.

The whole system of technology, from computers, to the Internet, to the Web, is very, very complex. I hope this project helped me with the "building blocks of true knowledge".

# Technology Choices

### C++

Google's V8 JavaScript engine is written in C++, so learning C++ will help me understand V8 source code.

Node.js is written in C++ and part of its code ecosystem are C++ APIs.

From [Wikipedia](https://en.wikipedia.org/wiki/C%2B%2B):
> C++ (/ˌsiːˌplʌsˈplʌs/) is a general-purpose programming language created by Bjarne Stroustrup as an extension of the C programming language, or "C with Classes". The language has expanded significantly over time, and modern C++ now has object-oriented, generic, and functional features in addition to facilities for low-level memory manipulation. It is almost always implemented as a compiled language, and many vendors provide C++ compilers, including the Free Software Foundation, LLVM, Microsoft, Intel, Oracle, and IBM, so it is available on many platforms.

### Rust

From [Wikipedia](https://en.wikipedia.org/wiki/Rust_%28programming_language%29):
> Rust is a multi-paradigm programming language focused on performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but provides memory safety without using garbage collection.

> Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others. The designers refined the language while writing the Servo layout or browser engine, and the Rust compiler. The compiler is free and open-source software dual-licensed under the MIT License and Apache License 2.0.

> Rust has been the "most loved programming language" in the Stack Overflow Developer Survey every year since 2016.

> You can implement the compiler in whatever language you like, but I’d recommend using a language with sum types and pattern matching, like OCaml, Haskell, or Rust.
- [Nora Sandler, Writing a C Compiler, Part 1](https://norasandler.com/2017/11/29/Write-a-Compiler.html)

I admire [Ryan Dahl's ideal to build high-quality software systems](https://tinyclouds.org/rant.html) and I'm interested in [his new secure JavaScript and TypeScript runtime called Deno](https://deno.land/). It uses V8 and is built in Rust. I want to understand his design choices and want to read through the Deno source code.

### 64-bit architecture

Writing a compiler includes hardware considerations.

The CPU itself will process the compiler program in binary format.

The CPU is made of:
- control unit (CU)
- arithmetic logic unit (ALU)
- registers
- cache
- buses
- clock
[Source](https://www.bbc.co.uk/bitesize/guides/zhppfcw/revision/2)

> Registers are small amounts of high-speed memory contained within the CPU. They are used by the processor to store small amounts of data that are needed during processing, such as:
- the address of the next instruction to be executed
- the current instruction being decoded
- the results of calculations
[Source](https://www.bbc.co.uk/bitesize/guides/zhppfcw/revision/2)

The registers are small, and each register has a certain number of bits. In the 1990s and early 2000s, many processors had 32-bit architecture.

>A 32-bit system can access 232 memory addresses, i.e 4 GB of RAM or physical memory.
A 64-bit system can access 264 memory addresses, i.e actually 18-Quintillion GB of RAM. In short, any amount of memory greater than 4 GB can be easily handled by it.

> One bit in the register can reference an individual byte in memory, so a 32-bit system can address a maximum of 4 GB (4,294,967,296 bytes) of RAM.

> A 64-bit register can theoretically reference 18,446,744,073,709,551,616 bytes, or 17,179,869,184 GB (16 exabytes) of memory. This is several million times more than an average workstation would need to access. What’s important is that a 64-bit computer (which means it has a 64-bit processor) can access more than 4 GB of RAM. If a computer has 8 GB of RAM, it better have a 64-bit processor. Otherwise, at least 4 GB of the memory will be inaccessible by the CPU.

> A major difference between 32-bit processors and 64-bit processors is the number of calculations per second they can perform, which affects the speed at which they can complete tasks. 64-bit processors can come in dual core, quad core, six core, and eight core versions for home computing.

- [Geeks for Geeks](https://www.geeksforgeeks.org/difference-32-bit-64-bit-operating-systems/)

My computer has 8 cores and 16GB of RAM (very thankful for that), so therefore my processor must have 64-bit Operating System. In fact, I am running macOS Catalina (10.15) and [it does not support 32-bit software](https://www.macworld.com/article/3393161/how-to-check-if-mac-software-is-32-or-64-bit.html). Previous versions of macOS, such as Mojave (10.14) did support 32-bit software. This makes my choice easy--my compiler must be 64-bit.

Building a compiler means compiling and running binary files and the OS itself must have the structure available to run it.

In the root directory `/` of my computer, there is a `bin` folder, and that's where "essential command binaries" live. Binaries are executable code, as opposed to data. The file is stored in binary format.

> Binary files are usually thought of as being a sequence of bytes, which means the binary digits (bits) are grouped in eights. Binary files typically contain bytes that are intended to be interpreted as something other than text characters. Compiled computer programs are typical examples; indeed, compiled applications are sometimes referred to, particularly by programmers, as binaries.

- [Binary file](https://en.wikipedia.org/wiki/Binary_file)

When code is executed, the CPU processes it, which means the instructions get put into the registers, so my binary must be compatible with my CPU, and my CPU enforces that its binary files conform to 64-bit architecture.
