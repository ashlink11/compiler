The interpreter we’re going to build in this book will implement all these features. It will tokenize and parse Monkey source code in a REPL, building up an internal representation of the code called abstract syntax tree and then evaluate this tree. It will have a few major parts:
• the lexer
• the parser
• the Abstract Syntax Tree (AST) • the internal object system
• the evaluator
We’re going to build these parts in exactly this order, from the bottom up. Or better put: starting with the source code and ending with the output.

We won’t be using any other tools in this book other than the ones provided by the Go programming language.

package token

type TokenType string

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers + literals
	IDENT = "IDENT" // add, foobar, x, y, ...
	INT   = "INT"   // 1343456

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"

	LT = "<"
	GT = ">"

	EQ     = "=="
	NOT_EQ = "!="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"

	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"

	// Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
	IF       = "IF"
	ELSE     = "ELSE"
	RETURN   = "RETURN"
)

type Token struct {
	Type    TokenType
	Literal string
}

var keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"true":   TRUE,
	"false":  FALSE,
	"if":     IF,
	"else":   ELSE,
	"return": RETURN,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}

go
statically typed 
compiled
2007
dependency analysis innovations
pointers but no pointer arithmetic
parallelism via routines 

Lexer

It doesn’t need to buffer or save tokens, since there will only be one method called NextToken(), which will output the next token.

That means, we’ll initialize the lexer with our source code and then repeatedly call NextToken() on it to go through the source code, token by token, character by character.

So let’s create a new package and add a first test that we can continuously run to get feedback about the working state of the lexer.

we successfully turned the small subset of the Monkey language we used in the our test case into tokens!

The lexer’s job is not to tell us whether code makes sense, works or contains errors. That comes in a later stage. The lexer should only turn this input into tokens. For that reason the test cases I write for lexers cover all tokens and also try to provoke off-by-one errors, edge cases at end-of-file, newline handling, multi-digit number parsing and so on.

// lexer/lexer_test.go
```go
func TestNextToken(t *testing.T) { input := `let five = 5;
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
   let result = add(five, ten);
   !-/*5;
   5 < 10 > 5;
   if (5 < 10) {
       return true;
   } else {
       return false;

  10 == 10; 10 != 9;
}`
// [...]
}
```

```go
var keywords = map[string]TokenType{ 
"fn": FUNCTION,
"let": LET,
"true": TRUE,
"false":  FALSE,
"if":     IF,
"else":   ELSE,
"return": RETURN,
}
```

p.25