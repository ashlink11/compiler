The interpreter we’re going to build in this book will implement all these features. It will tokenize and parse Monkey source code in a REPL, building up an internal representation of the code called abstract syntax tree and then evaluate this tree. It will have a few major parts:
• the lexer
• the parser
• the Abstract Syntax Tree (AST) • the internal object system
• the evaluator
We’re going to build these parts in exactly this order, from the bottom up. Or better put: starting with the source code and ending with the output.