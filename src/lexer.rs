#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Ident(String),
    Let,
    Print,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    Newline,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut n = c.to_string();
                while let Some(p) = chars.peek() {
                    if p.is_numeric() {
                        n.push(*p);
                        chars.next();
                    } else { break; }
                }
                tokens.push(Token::Number(n.parse().unwrap()));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '=' => tokens.push(Token::Equal),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '\n' | ';' => tokens.push(Token::Newline),
            c if c.is_whitespace() => {}
            c if c.is_alphabetic() => {
                let mut ident = c.to_string();
                while let Some(p) = chars.peek() {
                    if p.is_alphanumeric() || *p == '_' {
                        ident.push(*p);
                        chars.next();
                    } else { break; }
                }
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "print" => tokens.push(Token::Print),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => {}
        }
    }

    tokens
}
