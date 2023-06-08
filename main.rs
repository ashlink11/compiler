use std::str;

fn main() {
  //&str rather than &String and String::from()
  const ILLEGAL: &str = "ILLEGAL";
  #[allow(dead_code)]
  const EOF: &str = "EOF";

  // Identifiers + literals
  #[allow(dead_code)]
  const IDENT: &str = "IDENT";
  #[allow(dead_code)]
  const INT: &str = "INT";

  // operators
  #[allow(dead_code)]
  const ASSIGN: &str = "=";
  #[allow(dead_code)]
  const PLUS: &str = "+";
  #[allow(dead_code)]
  const MINUS: &str = "-";
  #[allow(dead_code)]
  const BANG: &str = "!";
  #[allow(dead_code)]
  const ASTERISK: &str = "*";
  #[allow(dead_code)]
  const SLASH: &str = "/";

  #[allow(dead_code)]
  const LT: &str = "<";
  #[allow(dead_code)]
  const GT: &str = ">";

  #[allow(dead_code)]
  const EQ: &str = "==";
  #[allow(dead_code)]
  const NOT_EQ: &str = "!=";

  // delimiters
  #[allow(dead_code)]
	const COMMA: &str = ",";
  #[allow(dead_code)]
	const SEMICOLON: &str = ";";
  #[allow(dead_code)]	
  const LPAREN: &str = "(";
  #[allow(dead_code)]
	const RPAREN: &str = ")";
  #[allow(dead_code)]
	const LBRACE: &str = "{";
  #[allow(dead_code)]
	const RBRACE: &str = "}";

  // keywords
  #[allow(dead_code)]
	const FUNCTION: &str = "FUNCTION";
	#[allow(dead_code)]
  const LET: &str = "LET";
  #[allow(dead_code)]
	const TRUE: &str = "TRUE";
  #[allow(dead_code)]
	const FALSE: &str = "FALSE";
  #[allow(dead_code)]
	const IF: &str = "IF";
  #[allow(dead_code)]
	const ELSE: &str = "ELSE";
  #[allow(dead_code)]
	const RETURN: &str = "RETURN";

  #[allow(dead_code)]
  pub fn next_token(a: i32, b: i32) -> i32 {
    a + b
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        assert_eq!(next_token(1, 2), 3);
    }
  }

  println!("The first const to print is {}", ILLEGAL);


}

