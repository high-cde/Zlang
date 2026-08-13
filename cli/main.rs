use std::{env, fs};
use zlang_core::compiler::{codegen::Codegen, lexer::Lexer, parser::Parser};
use zlang_core::vm::vm::VM;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("ZDOS Z-Lang v0.2.0"); return; }
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut parser = Parser::new(Lexer::new(&input));
    let stmts = parser.parse_program();
    let mut codegen = Codegen::new();
    codegen.compile(&stmts);
    VM::new(codegen.code, codegen.constants).run();
}
