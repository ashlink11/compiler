Lecture notes: [Rust: A Language for the Next 40 Years - Carol Nichols](https://www.youtube.com/watch?v=A3AdN7U24iU)

memory safety problems C
use after free
double free
memory leaks
buffer overreads/overwrites
null pointers
data races

C++
there's still a lot of undefined behavior
there's still a lot of pointer problems you can get into
multi-threaded programming is still a big problem in C++

Portability

>Rust compiles to LLVM intermediate representation so you can write Rust and compile it for any platform that LLVM can. There are C compilers that target platforms that LLVM doesn't. There isn't a non-LLVM backend yet for rust. (2019)

Rust v1.00 came out May 2015. They ship code every 6 weeks with strong backwards compatibility guarantees.

Breaking compilation (what guarantee is about) vs breaking runtime behavior

Rust 2015 Edition
Rust 2018 Edition - add new keywords, e.g. async; opt-in

Terrible for compiler maintenance?

Rust source code --> high level intermediate representation (HIR) --> mid level intermediate representation (MIR) --> LLVM IR --> Machine code

Rust compiler does borrow checking, optimizations, code generation in MIR
 - Editions are different HIR but compile to same MIR, so compiler is same MIR and down

2019 Rust Todo:
 - ISO/ECMA standard
 - Compiler certification
 - LTS release (security fixes)
 - cargo build tool only; prevents companies who use other build systems to integrate
 - open-source package registry (no private packages)
 - not tons of Rust libraries yet

CSS rendering in Firefox Quantum renders all the CSS properties in your page in parallel (used to be written in C++; Rust would have prevented 75% of security bugs); 2x speed increase too

Rust use:
- Swift is similar to Rust -- used at Apple
- Amazon - Firecracker MicroVM
- Google - Fuchsia OS project (open source)
- FaceBook - Mononoke mercurial server to manage mercurial mono repo
- Microsoft - IOT Edge analytics

Memory unsafety
Demand that our dependencies are safer
Rust is a better, safer C

Language server protocol: standardize way languages talk
code completion, in-line errors, jump to definition
IDE compatibility: VSCode (best-maintained plugin that uses language server protocol), intelliJ (pretty good rust support)



What is event-driven?
Event driven means when you make a call, your node server catches the request and registers a function. Let me repeat, Nodejs registers a function instead of the actual data. Why though? Here is why. Instead of having the event-cycle to wait for the data while it’s fetching (which could take sometime), node simply registers an event that will be called when the fetching is done and frees the event-cycle to take the next request.

Node.js = glue v8 and LibUV (event I/O support across OS) to create a very good framework to develop critical I/O server applications.

N-API - binding API
libuv - ben noordhuis and bert belder


But, I think Node is quite nice.
JavaScript is really quite nice.
In particular, dynamic languages are very nice.
dynamic languages are good for scientific computing and prototyping

promises introduced an extra object into every callback
promises are the necessary abstraction for async / await
many async APIs are aging badly due to this

JavaScript is a very secure sandbox unlike Python (JS is best dynamic language)

v8 is a very good secuity sandbox
ideal: cannot write to disc or access the network

build system (gyp)
very very difficult
If you're writing a module that links to a C library, you use this thing called jip to compile that C library and link it into Node, right? Jip is this thing that Chrome used to use, but Chrome, like abandoned jip for this other tool called GN several years later.
Node is sole user of GYP
v8 has a GYP wrapper to support Node

There's a lot of software to support, but I do think that I kind of made a kind of rash decision in thinking that everybody should compile their extension modules. We could have gone with a foreign function interface, which doesn't require any compiling necessarily, although you have to have your dependencies there, but I think this could have been a much more natural and easy interface for people who want to link to system libraries, and many people suggested this to me, namely Bryan Cantrell, but I totally ignored them.

Instead of guiding users to write C++ bindings to V8, I should have provided a core foreign function interface (FFI).

Displeased that libuv adopted autotools.

package.json Issac invented it
require() inspects package.json for "main" files, which means package.json is necessary
--unnecessary noise in it

npm - centralized privately controlled repository for modules

module package - unnecessary abstraction (rather than just JS files and link to a library)

node modules
- install locally: multiple projects, multiple node modules files, it gets really big
- vendoring-by-default semantics: using NODE_PATH env var wouldve precluded it; deviates greatly with browser semantics

things that unnecessary but simply cute to add - always regret

Node problems are not around the IO
I like programming in it. It's Unix-y.
Nerding out hardcore on epoll
module system was an afterthought as users needed it
how does Node manage user code?

DENO 2018 -
lldb
LLDB is a next generation, high-performance debugger. It is built as a set of reusable components which highly leverage existing libraries in the larger LLVM Project, such as the Clang expression parser and LLVM disassembler.

LLDB is the default debugger in Xcode on macOS and supports debugging C, Objective-C and C++ on the desktop and iOS devices and simulator.


The LLVM compiler infrastructure project is a set of compiler and toolchain technologies,[3] which can be used to develop a front end for any programming language and a back end for any instruction set architecture. LLVM is designed around a language-independent intermediate representation (IR) that serves as a portable, high-level assembly language that can be optimized with a variety of transformations over multiple passes.[4]

LLVM is written in C++ and is designed for compile-time, link-time, run-time, and "idle-time" optimization. Originally implemented for C and C++, the language-agnostic design of LLVM has since spawned a wide variety of front ends: languages with compilers that use LLVM include ActionScript, Ada, C#,[5][6][7] Common Lisp, Crystal, CUDA, D, Delphi, Dylan, Fortran, Graphical G Programming Language,[8] Halide, Haskell, Java bytecode, Julia, Kotlin, Lua, Objective-C, OpenGL Shading Language, Ruby,[9] Rust, Scala,[10] Swift, and Xojo.

In computer science, runtime, run time or execution time is the time when the CPU is executing the machine code. It is the last stage of a program's lifecycle.

18 min - more notes needed
next: <https://www.youtube.com/watch?v=1gIiZfSbEAE>
