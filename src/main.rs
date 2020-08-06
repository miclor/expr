
use regex::Regex;

#[derive(Debug)]
enum TokenType {
    OpenParen,
    CloseParen,
    Atom,
}

#[derive(Debug)]
struct Token<'a> {
    value: &'a str,
    token_type: TokenType,
}

#[derive(Debug)]
struct Lexer<'a> {
    input_string: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_string: &'a str) -> Lexer<'a> {
        Lexer { tokens: Vec::new(), input_string: input_string}
    }

    fn lex(&self, source_text: &'a str)  {
        let re_open_paren = Regex::new(r"[(]{1}").unwrap();
        let re_close_paren = Regex::new(r"[)]{1}").unwrap();
        let re_atom = Regex::new(r"[0-9A-za-z].*").unwrap();

        let n = 0;

        while (n < source_text.length()) {

            n =
        }

        assert!(re_open_paren.is_match("("));
        assert!(re_close_paren.is_match(")"));
        assert!(re_atom.is_match("Atom123"));
    }
}


fn main() {
    let l = Lexer::new("( test )");
    l.lex("( atom )");
    println!("ugh")
}


