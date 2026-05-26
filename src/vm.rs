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
