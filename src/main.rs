
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

fn match_open_paren(input_string: &str) -> Result<usize, &str>{
    let re_open_paren = Regex::new(r"^[(]").unwrap();
    let mat = re_open_paren.find(input_string);

    if mat.is_some() {
        let length = mat.unwrap().end();
        return Ok(length);
    }
    return Err("No match");
}

fn match_close_paren() {

}

fn match_atom() {

}


impl<'a> Lexer<'a> {
    pub fn new(input_string: &'a str) -> Lexer<'a> {
        Lexer { tokens: Vec::new(), input_string: input_string}
    }

    fn lex(&self, source_text: &'a str)  {
        // let re_open_paren = Regex::new(r"^[(]").unwrap();
        // let re_close_paren = Regex::new(r"&^[)]").unwrap();
        // let re_atom = Regex::new(r"^[0-9A-za-z].*").unwrap();
        // let re_whitespace = Regex::new(r"^[\s]").unwrap();

        let mut n = 0;

        while n < source_text.len() {



        }


    }
}


fn main() {
    let l = Lexer::new("( test )");
    l.lex("( atom )");
    println!("ugh")
}


