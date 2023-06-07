use std::str;

fn main() {
  //&str rather than &String and String::from()
  const ILLEGAL: &str = "ILLEGAL";
  const EOF: &str = "EOF";

  // Identifiers + literals
	const IDENT: &str = "IDENT";
	const INT: &str = "INT";

  // operators
	const ASSIGN: &str = "=";
	const PLUS: &str = "+";
	const MINUS: &str = "-";
	const BANG: &str = "!";
	const ASTERISK: &str = "*";
	const SLASH: &str = "/";

	const LT: &str = "<";
	const GT: &str = ">";

	const EQ: &str = "==";
	const NOT_EQ: &str = "!=";

  // delimiters
	const COMMA: &str = ",";
	const SEMICOLON: &str = ";";
	
  const LPAREN: &str = "(";
	const RPAREN: &str = ")";
	const LBRACE: &str = "{";
	const RBRACE: &str = "}";

  // keywords
	const FUNCTION: &str = "FUNCTION";
	const LET: &str = "LET";
	const TRUE: &str = "TRUE";
	const FALSE: &str = "FALSE";
	const IF: &str = "IF";
	const ELSE: &str = "ELSE";
	const RETURN: &str = "RETURN";

  println!("The first const to print is {}", ILLEGAL);


}

