use std::fs;
use std::process::Command;

use zdos_zlang::compiler::Compiler;
use zdos_zlang::lexer::{tokenize, Token};
use zdos_zlang::parser::{parse, AST};

#[test]
fn lexer_recognizes_basic_arithmetic_program() {
    let tokens = tokenize("let x = 2 + 3\nprint x");
    assert!(matches!(tokens[0], Token::Let));
    assert!(matches!(tokens[1], Token::Ident(ref name) if name == "x"));
    assert!(matches!(tokens[2], Token::Equal));
    assert!(matches!(tokens[3], Token::Number(2)));
    assert!(matches!(tokens[4], Token::Plus));
    assert!(matches!(tokens[5], Token::Number(3)));
}

#[test]
fn parser_builds_program_ast() {
    let ast = parse(tokenize("let answer = (2 + 3) * 4"));
    assert!(matches!(ast, AST::Program(statements) if statements.len() == 1));
}

#[test]
fn compiler_emits_supported_runtime_instructions() {
    let mut compiler = Compiler::new();
    let bytecode = compiler.compile("emit hello\norbit_sync\nzchain tx-1");
    assert_eq!(
        bytecode,
        vec![
            "PRINT_STDOUT hello".to_string(),
            "SPACEX_LEO_HANDSHAKE".to_string(),
            "ZCHAIN_SIGN tx-1".to_string(),
        ]
    );
}

#[test]
fn cli_executes_smoke_script() {
    let root = std::env::temp_dir().join(format!("zlang-smoke-{}.zl", std::process::id()));
    fs::write(&root, "emit smoke-test\norbit_sync\n").expect("write smoke script");

    let output = Command::new(env!("CARGO_BIN_EXE_zlang"))
        .arg(&root)
        .output()
        .expect("run zlang binary");

    fs::remove_file(&root).ok();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("smoke-test"));
    assert!(stdout.contains("Handshake LEO completato"));
}
