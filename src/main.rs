
use regex::Regex;

#[derive(Debug)]
enum TokenType {
    OpenParen,
    CloseParen,
    Atom,
    Whitespace
}

#[derive(Debug)]
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

            println!("checking whitespace, {:?}", self.state);
            self.tokens.push(self.match_whitespace().unwrap());
            println!("checking open paren, {:?}", self.state);
            self.tokens.push(self.match_open_paren().unwrap());
            println!("checking close paren: {:?}", self.state);
            self.tokens.push(self.match_close_paren().unwrap());
            println!("checking atom: {:?}", self.state);
            self.tokens.push(self.match_atom().unwrap());

        }

        println!("state: {:?}", self.state);

    }

    fn match_open_paren(&'a mut self) -> Option<Token<'a>> {
        let re_open_paren = Regex::new("^\\(").unwrap();
        let mat = re_open_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for open paren at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            self.state += length;
            // return length;
            return Some(Token { value: "(", token_type: TokenType::OpenParen });
        }
        return None;
    }

    fn match_close_paren(&mut self) -> Option<Token> {
        let re_close_paren = Regex::new("^\\)").unwrap();
        let mat = re_close_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for close paren at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            self.state += length;
            return Some(Token { value: ")", token_type: TokenType::CloseParen });
        }
        return None;
    }

    fn match_atom(&mut self) -> Option<Token> {
        let re_atom = Regex::new("^\\w+").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for atom at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            self.state += length;
            return Some(Token { value: mat.unwrap().as_str(), token_type: TokenType::Atom });
        }
        return None;
    }

    fn match_whitespace(&mut self) -> Option<Token> {
        let re_atom = Regex::new(r"^\s").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for whitespace at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            self.state += length;
            return Some(Token { value: mat.unwrap().as_str(), token_type: TokenType::Whitespace });
        }
        println!("  did not find match for whitespace at{:?}", self.state);
        return None;
    }
}


fn main() {
    let mut l = Lexer::new("( test )");
    l.lex();
    println!("yay")
}


