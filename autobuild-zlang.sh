#!/data/data/com.termux/files/usr/bin/bash

set -e

echo "[ZLANG] Ricostruzione progetto…"

# 1. Pulizia e struttura
rm -rf src
mkdir -p src

# 2. LEXER
cat > src/lexer.rs << 'EOF'
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
EOF

# 3. PARSER
cat > src/parser.rs << 'EOF'
use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum AST {
    Program(Vec<AST>),
    Let(String, Box<AST>),
    Number(i64),
    Ident(String),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
    Print(Box<AST>),
}

fn panic_msg(s: &str) -> ! {
    eprintln!("PANIC: {}", s);
    std::process::exit(1);
}

pub fn parse(tokens: Vec<Token>) -> AST {
    let mut q: VecDeque<Token> = tokens.into();
    let mut stmts = Vec::new();

    while q.front().is_some() {
        while matches!(q.front(), Some(Token::Newline)) { q.pop_front(); }
        if q.front().is_none() { break; }
        stmts.push(parse_stmt(&mut q));
    }

    AST::Program(stmts)
}

fn parse_stmt(q: &mut VecDeque<Token>) -> AST {
    match q.front() {
        Some(Token::Let) => parse_let(q),
        Some(Token::Print) => parse_print(q),
        _ => parse_expr(q),
    }
}

fn parse_let(q: &mut VecDeque<Token>) -> AST {
    q.pop_front();
    let name = match q.pop_front() {
        Some(Token::Ident(s)) => s,
        _ => panic_msg("Expected identifier"),
    };
    match q.pop_front() {
        Some(Token::Equal) => {}
        _ => panic_msg("Expected ="),
    }
    AST::Let(name, Box::new(parse_expr(q)))
}

fn parse_print(q: &mut VecDeque<Token>) -> AST {
    q.pop_front();
    AST::Print(Box::new(parse_expr(q)))
}

fn parse_expr(q: &mut VecDeque<Token>) -> AST {
    let mut node = parse_term(q);
    loop {
        match q.front() {
            Some(Token::Plus) => { q.pop_front(); node = AST::Add(Box::new(node), Box::new(parse_term(q))); }
            Some(Token::Minus) => { q.pop_front(); node = AST::Sub(Box::new(node), Box::new(parse_term(q))); }
            _ => break,
        }
    }
    node
}

fn parse_term(q: &mut VecDeque<Token>) -> AST {
    let mut node = parse_factor(q);
    loop {
        match q.front() {
            Some(Token::Star) => { q.pop_front(); node = AST::Mul(Box::new(node), Box::new(parse_factor(q))); }
            Some(Token::Slash) => { q.pop_front(); node = AST::Div(Box::new(node), Box::new(parse_factor(q))); }
            _ => break,
        }
    }
    node
}

fn parse_factor(q: &mut VecDeque<Token>) -> AST {
    match q.pop_front() {
        Some(Token::Number(n)) => AST::Number(n),
        Some(Token::Ident(s)) => AST::Ident(s),
        Some(Token::LParen) => {
            let e = parse_expr(q);
            match q.pop_front() {
                Some(Token::RParen) => e,
                _ => panic_msg("Expected )"),
            }
        }
        other => panic_msg(&format!("Unexpected token: {:?}", other)),
    }
}
EOF

# 4. VM
cat > src/vm.rs << 'EOF'
use crate::parser::AST;
use std::collections::HashMap;

fn print_output(s: &str) {
    print!("{}\n", s);
}

pub fn run_code(code: &str) {
    let tokens = crate::lexer::tokenize(code);
    let ast = crate::parser::parse(tokens);
    execute(ast);
}

pub fn execute(ast: AST) {
    let mut env = HashMap::new();
    eval(ast, &mut env);
}

fn eval(ast: AST, env: &mut HashMap<String, String>) -> Option<String> {
    match ast {
        AST::Program(stmts) => { for s in stmts { eval(s, env); } None }
        AST::Let(name, expr) => { let v = eval(*expr, env).unwrap(); env.insert(name, v); None }
        AST::Number(n) => Some(n.to_string()),
        AST::Ident(name) => env.get(&name).cloned(),
        AST::Add(a, b) => Some((eval(*a, env).unwrap().parse::<i64>().unwrap() + eval(*b, env).unwrap().parse::<i64>().unwrap()).to_string()),
        AST::Sub(a, b) => Some((eval(*a, env).unwrap().parse::<i64>().unwrap() - eval(*b, env).unwrap().parse::<i64>().unwrap()).to_string()),
        AST::Mul(a, b) => Some((eval(*a, env).unwrap().parse::<i64>().unwrap() * eval(*b, env).unwrap().parse::<i64>().unwrap()).to_string()),
        AST::Div(a, b) => Some((eval(*a, env).unwrap().parse::<i64>().unwrap() / eval(*b, env).unwrap().parse::<i64>().unwrap()).to_string()),
        AST::Print(expr) => { let v = eval(*expr, env).unwrap(); print_output(&v); None }
    }
}
EOF

# 5. API
cat > src/api.rs << 'EOF'
pub fn http_get(_url: &str) -> String {
    "http disabled".to_string()
}
EOF

# 6. GIT + PUSH
git init
git branch -M main
git remote remove origin 2>/dev/null || true
git remote add origin https://github.com/high-cde/Zlang.git
git add .
git commit -m "ZLang autobuild"
git push -u origin main

echo "[ZLANG] Autobuild completato."
