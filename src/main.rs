
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
    tokens: Vec<Token<'a>>,
}

fn match_number(source_text: &str)  {

    let re = Regex::new(r"\D").unwrap();
    assert!(re.is_match("2%%014a01a11"));
}


fn main() {
    println!("Hello, world!");
    match_number("( 232 )");
}


