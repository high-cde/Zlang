#[derive(Debug, Clone, PartialEq)]
pub enum Token { 
    Let, Print, Hash, Broadcast, Exec, If, Else, While,
    Identifier(String), Int(i64), Str(String), 
    Plus, Minus, Eq, EqEq, Less, Greater, LBrace, RBrace, EOF 
}

pub struct Lexer { input: Vec<char>, pos: usize }

impl Lexer {
    pub fn new(input: &str) -> Self { Lexer { input: input.chars().collect(), pos: 0 } }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() { return Token::EOF; }
        let ch = self.input[self.pos];
        
        match ch {
            '+' => { self.pos += 1; Token::Plus }
            '-' => { self.pos += 1; Token::Minus }
            '{' => { self.pos += 1; Token::LBrace }
            '}' => { self.pos += 1; Token::RBrace }
            '<' => { self.pos += 1; Token::Less }
            '>' => { self.pos += 1; Token::Greater }
            '=' => { 
                self.pos += 1; 
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1; Token::EqEq
                } else {
                    Token::Eq
                }
            }
            '"' => self.read_string(),
            _ => { 
                if ch.is_alphabetic() { self.read_identifier() } 
                else if ch.is_numeric() { self.read_number() } 
                else { self.pos += 1; self.next_token() } 
            }
        }
    }
    
    fn skip_whitespace(&mut self) { while self.pos < self.input.len() && self.input[self.pos].is_whitespace() { self.pos += 1; } }
    
    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') { self.pos += 1; }
        let s: String = self.input[start..self.pos].iter().collect();
        match s.as_str() { 
            "let" => Token::Let, "print" => Token::Print, "hash" => Token::Hash, 
            "broadcast" => Token::Broadcast, "exec" => Token::Exec, 
            "if" => Token::If, "else" => Token::Else, "while" => Token::While,
            _ => Token::Identifier(s) 
        }
    }
    
    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_numeric() { self.pos += 1; }
        let s: String = self.input[start..self.pos].iter().collect();
        Token::Int(s.parse().unwrap())
    }
    
    fn read_string(&mut self) -> Token {
        self.pos += 1; let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' { self.pos += 1; }
        let s: String = self.input[start..self.pos].iter().collect();
        self.pos += 1; Token::Str(s)
    }
}
