
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
    state: usize,
}



impl<'a> Lexer<'a> {
    pub fn new(input_string: &'a str) -> Lexer<'a> {
        Lexer { tokens: Vec::new(), input_string: input_string, state: 0}
    }

    fn lex(&mut self, source_text: &'a str)  {

        // let mut n: usize = 0;

        while self.state < source_text.len() {

            println!("checking whitespace, {:?}", self.state);
            self.state += self.match_whitespace();
            println!("checking open paren, {:?}", self.state);
            self.state += self.match_open_paren();
            println!("checking close paren: {:?}", self.state);
            self.state += self.match_close_paren();
            println!("checking atom: {:?}", self.state);
            self.state += self.match_atom();

        }

        println!("n: {:?}", self.state);

    }

    fn match_open_paren(&self) -> usize {
        let re_open_paren = Regex::new("^\\(").unwrap();
        let mat = re_open_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for open paren at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            return length;
        }
        return 0;
    }

    fn match_close_paren(&self) -> usize {
        let re_close_paren = Regex::new("^\\)").unwrap();
        let mat = re_close_paren.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for close paren at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            return length;
        }
        return 0;
    }

    fn match_atom(&self) -> usize {
        let re_atom = Regex::new("^\\w+").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for atom at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            return length;
        }
        return 0;
    }

    fn match_whitespace(&self) -> usize {
        let re_atom = Regex::new(r"^\s").unwrap();
        let mat = re_atom.find(&self.input_string[self.state.. self.input_string.len()]);

        if mat.is_some() {
            println!("  found match for whitespace at {:?}", &self.input_string[self.state..]);
            let length = mat.unwrap().end();
            return length;
        }
        println!("  did not find match for whitespace at{:?}", self.state);
        return 0;
    }
}


fn main() {
    let mut l = Lexer::new("( test )");
    l.lex("( atom )");
    println!("yay")
}


