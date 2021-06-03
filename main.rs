//import `one_plus_one.rs`

fn main() {
  // get a character stream of `one_plus_one.rs`
  let charstream = "";

  // parse first char
  mut current_chars = charstream[0];

  // all words/characters/symbols in `one_plus_one.rs`
  let FN = "fn";
  let MAIN = "main";
  let SPACE = " ";
  let NEWLINE = "\n";
  let OPEN_PARENS = "(";
  let CLOSE_PARENS = ")";  
  let OPEN_CURLY = "{";
  let CLOSE_CURLY = "}";  
  let ONE = "1";
  let PLUS = "+";  
  let PRINTLN = "println!";

  // build dictionary
  let dictionary = Vector<String>;

  // add words to dictionary
  dictionary.push(FN)
  dictionary.push(MAIN)
  dictionary.push(SPACE)
  dictionary.push(NEWLINE)
  dictionary.push(OPEN_PARENS)
  dictionary.push(CLOSE_PARENS)
  dictionary.push(CLOSE_CURLY)
  dictionary.push(OPEN_CURLY)
  dictionary.push(ONE)
  dictionary.push(PLUS)
  dictionary.push(PRINTLN)
  
  // TODO
  // loop through dictionary and compare/check for equality w `current_chars`
    // if there's a match, make note & increment `current_chars` in the charstream
    // if there's not a match, add one more char to `current_chars`.

}




// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/struct.Item.html
pub struct Item<K = ItemKind> {
  pub attrs: Vec<Attribute>,
  pub id: NodeId,
  pub span: Span,
  pub vis: Visibility,
  pub ident: Ident,
  pub kind: K,
  pub tokens: Option<LazyTokenStream>,
}

// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/enum.ItemKind.html
pub enum ItemKind {
  // ...
}