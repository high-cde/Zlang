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
