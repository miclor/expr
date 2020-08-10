
use regex::Regex;

#[derive(Debug, PartialEq)]
enum TokenType {
    OpenParen,
    CloseParen,
    Atom,
    Whitespace
}


#[derive(Debug, PartialEq)]
struct Token<'a> {
    value: &'a str,
    token_type: TokenType,
}


impl<'a> Token<'a> {
   pub fn new (val: &'a str, t_type: TokenType) -> Token<'a> {
        Token { value: val, token_type: t_type }
    }
}


#[derive(Debug)]
struct Lexer<'a> {
    input_string: &'a str,
    tokens: Vec<Token<'a>>,
    state: usize,
}


impl<'a> Lexer<'a> {
    pub fn new(input_string: &'a str) -> Lexer<'a> {
        Lexer { tokens: Vec::new(), input_string: input_string, state: 0}
    }

    fn lex(&mut self) {
        while self.state < self.input_string.len() {
            self.match_whitespace();
            self.match_open_paren();
            self.match_close_paren();
            self.match_atom();
        }
    }

    fn match_open_paren(& mut self)  {
        let re_open_paren = Regex::new("^\\(").unwrap();
        let mat = re_open_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            let length = mat.unwrap().end();
            self.state += length;
            self.tokens.push(Token { value: "(", token_type: TokenType::OpenParen });
        }

    }

    fn match_close_paren(&mut self) {
        let re_close_paren = Regex::new("^\\)").unwrap();
        let mat = re_close_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            let length = mat.unwrap().end();
            self.state += length;
            self.tokens.push(Token { value: ")", token_type: TokenType::CloseParen });
        }

    }

    fn match_atom(&mut self) {
        let re_atom = Regex::new("^\\w+").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            let length = mat.unwrap().end();
            self.state += length;
            self.tokens.push(Token { value: mat.unwrap().as_str(), token_type: TokenType::Atom });
        }

    }

    fn match_whitespace(&mut self) {
        let re_atom = Regex::new(r"^\s").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            let length = mat.unwrap().end();
            self.state += length;
            //self.tokens.push(Token { value: mat.unwrap().as_str(), token_type: TokenType::Whitespace });
        }
    }

    fn match_token(&mut self, reg_ex: &str, token_type: TokenType) {
        let re_atom = Regex::new(reg_ex).unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            let length = mat.unwrap().end();
            self.state += length;
            self.tokens.push(Token { value: mat.unwrap().as_str(), token_type: token_type });
        }
    }
}

#[derive(Debug)]
struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    state: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a std::vec::Vec<Token<'a>>) -> Parser<'a> {
        Parser { tokens: &tokens, state: 0}
    }

    pub fn parse_expr(&mut self) {
        
        if *&self.tokens[self.state].token_type == TokenType::Atom {
            println!("found an atom: {:?}", self.state);
            self.state += 1;
        } else if *&self.tokens[self.state].token_type == TokenType::OpenParen {
            println!("found an open paren: {:?}", self.state);
            self.state += 1;
        }

        if *&self.tokens[self.state].token_type == TokenType::CloseParen {
            println!("found an close paren: {:?}", self.state);
            self.state += 1;
        }

    }

}

fn main() {
    let mut l = Lexer::new(" test ");
    l.lex();

    for i in &l.tokens {
        println!("Token: {:?}", i);
    }

    let mut p = Parser::new(&l.tokens);

    p.parse_expr();

}


