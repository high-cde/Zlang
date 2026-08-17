use std::collections::{BTreeSet, HashMap};

use crate::error::{ZlangError, ZlangResult};
use crate::vm::{
    BytecodeModule, Capability, Instruction, Register, BYTECODE_VERSION, REGISTER_COUNT,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
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
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Ident(String),
    Neg(Box<Expr>),
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Statement {
    Let { name: String, value: Expr },
    Print(Expr),
}

pub struct Compiler {
    registers: HashMap<String, Register>,
    next_register: u8,
    instructions: Vec<Instruction>,
    capabilities: BTreeSet<Capability>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            next_register: 0,
            instructions: Vec::new(),
            capabilities: BTreeSet::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> ZlangResult<BytecodeModule> {
        self.reset();
        let tokens = tokenize(source)?;
        let statements = Parser::new(tokens).parse_program()?;

        for statement in statements {
            self.compile_statement(statement)?;
        }
        self.instructions.push(Instruction::Halt);

        BytecodeModule::new(
            BYTECODE_VERSION,
            REGISTER_COUNT as u8,
            self.capabilities.iter().copied().collect(),
            self.instructions.clone(),
        )
    }

    fn reset(&mut self) {
        self.registers.clear();
        self.next_register = 0;
        self.instructions.clear();
        self.capabilities.clear();
    }

    fn compile_statement(&mut self, statement: Statement) -> ZlangResult<()> {
        match statement {
            Statement::Let { name, value } => {
                let register = self.compile_expression(value)?;
                self.registers.insert(name, register);
            }
            Statement::Print(expression) => {
                let source = self.compile_expression(expression)?;
                self.capabilities.insert(Capability::ConsoleWrite);
                self.instructions.push(Instruction::Emit { source });
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: Expr) -> ZlangResult<Register> {
        match expression {
            Expr::Number(value) => {
                let destination = self.allocate_register()?;
                self.instructions
                    .push(Instruction::LoadImm { destination, value });
                Ok(destination)
            }
            Expr::Ident(name) => self
                .registers
                .get(&name)
                .copied()
                .ok_or_else(|| ZlangError::Compile(format!("undefined variable `{name}`"))),
            Expr::Neg(expression) => {
                let source = self.compile_expression(*expression)?;
                let destination = self.allocate_register()?;
                self.instructions.push(Instruction::Neg {
                    destination,
                    source,
                });
                Ok(destination)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.compile_expression(*left)?;
                let right = self.compile_expression(*right)?;
                let destination = self.allocate_register()?;
                let instruction = match operator {
                    BinaryOperator::Add => Instruction::Add {
                        destination,
                        left,
                        right,
                    },
                    BinaryOperator::Sub => Instruction::Sub {
                        destination,
                        left,
                        right,
                    },
                    BinaryOperator::Mul => Instruction::Mul {
                        destination,
                        left,
                        right,
                    },
                    BinaryOperator::Div => Instruction::Div {
                        destination,
                        left,
                        right,
                    },
                };
                self.instructions.push(instruction);
                Ok(destination)
            }
        }
    }

    fn allocate_register(&mut self) -> ZlangResult<Register> {
        if usize::from(self.next_register) >= REGISTER_COUNT {
            return Err(ZlangError::Compile(format!(
                "register budget exhausted (maximum {REGISTER_COUNT}); split the expression or program"
            )));
        }
        let register = Register(self.next_register);
        self.next_register += 1;
        Ok(register)
    }
}

fn tokenize(source: &str) -> ZlangResult<Vec<Token>> {
    let mut tokens = Vec::new();
    for (line_index, raw_line) in source.lines().enumerate() {
        let line = line_index + 1;
        let mut characters = raw_line.char_indices().peekable();
        while let Some((offset, character)) = characters.next() {
            let column = offset + 1;
            match character {
                '#' => break,
                ' ' | '\t' | '\r' => {}
                '0'..='9' => {
                    let mut literal = character.to_string();
                    while let Some((_, next)) = characters.peek() {
                        if next.is_ascii_digit() {
                            literal.push(*next);
                            characters.next();
                        } else {
                            break;
                        }
                    }
                    let value = literal.parse::<i64>().map_err(|_| {
                        ZlangError::source(line, column, "integer literal is out of range")
                    })?;
                    tokens.push(Token {
                        kind: TokenKind::Number(value),
                        line,
                        column,
                    });
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = character.to_string();
                    while let Some((_, next)) = characters.peek() {
                        if next.is_ascii_alphanumeric() || *next == '_' {
                            identifier.push(*next);
                            characters.next();
                        } else {
                            break;
                        }
                    }
                    let kind = match identifier.as_str() {
                        "let" => TokenKind::Let,
                        "print" => TokenKind::Print,
                        _ => TokenKind::Ident(identifier),
                    };
                    tokens.push(Token { kind, line, column });
                }
                '+' => tokens.push(simple(TokenKind::Plus, line, column)),
                '-' => tokens.push(simple(TokenKind::Minus, line, column)),
                '*' => tokens.push(simple(TokenKind::Star, line, column)),
                '/' => tokens.push(simple(TokenKind::Slash, line, column)),
                '=' => tokens.push(simple(TokenKind::Equal, line, column)),
                '(' => tokens.push(simple(TokenKind::LParen, line, column)),
                ')' => tokens.push(simple(TokenKind::RParen, line, column)),
                ';' => tokens.push(simple(TokenKind::Newline, line, column)),
                _ => {
                    return Err(ZlangError::source(
                        line,
                        column,
                        format!("unsupported character `{character}`"),
                    ));
                }
            }
        }
        tokens.push(simple(TokenKind::Newline, line, raw_line.len() + 1));
    }
    tokens.push(simple(TokenKind::End, source.lines().count() + 1, 1));
    Ok(tokens)
}

fn simple(kind: TokenKind, line: usize, column: usize) -> Token {
    Token { kind, line, column }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse_program(&mut self) -> ZlangResult<Vec<Statement>> {
        let mut statements = Vec::new();
        self.skip_newlines();
        while !matches!(self.current().kind, TokenKind::End) {
            statements.push(self.parse_statement()?);
            if !matches!(self.current().kind, TokenKind::Newline | TokenKind::End) {
                return Err(self.error_here("expected a newline after the statement"));
            }
            self.skip_newlines();
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> ZlangResult<Statement> {
        match &self.current().kind {
            TokenKind::Let => {
                self.advance();
                let name = match self.advance().kind.clone() {
                    TokenKind::Ident(name) => name,
                    _ => return Err(self.error_here("expected an identifier after `let`")),
                };
                self.expect(TokenKind::Equal, "expected `=` after variable name")?;
                Ok(Statement::Let {
                    name,
                    value: self.parse_expression()?,
                })
            }
            TokenKind::Print => {
                self.advance();
                Ok(Statement::Print(self.parse_expression()?))
            }
            _ => Err(self.error_here("expected `let` or `print`")),
        }
    }

    fn parse_expression(&mut self) -> ZlangResult<Expr> {
        let mut expression = self.parse_term()?;
        while matches!(self.current().kind, TokenKind::Plus | TokenKind::Minus) {
            let operator = match self.advance().kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Sub,
                _ => unreachable!("operator is guarded by the loop condition"),
            };
            let right = self.parse_term()?;
            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_term(&mut self) -> ZlangResult<Expr> {
        let mut expression = self.parse_factor()?;
        while matches!(self.current().kind, TokenKind::Star | TokenKind::Slash) {
            let operator = match self.advance().kind {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                _ => unreachable!("operator is guarded by the loop condition"),
            };
            let right = self.parse_factor()?;
            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_factor(&mut self) -> ZlangResult<Expr> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Number(value) => Ok(Expr::Number(value)),
            TokenKind::Ident(name) => Ok(Expr::Ident(name)),
            TokenKind::Minus => Ok(Expr::Neg(Box::new(self.parse_factor()?))),
            TokenKind::LParen => {
                let expression = self.parse_expression()?;
                self.expect(TokenKind::RParen, "expected `)` to close expression")?;
                Ok(expression)
            }
            _ => Err(ZlangError::source(
                token.line,
                token.column,
                "expected integer, identifier, unary `-`, or parenthesized expression",
            )),
        }
    }

    fn expect(&mut self, expected: TokenKind, message: &str) -> ZlangResult<()> {
        if same_variant(&self.current().kind, &expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_here(message))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current().kind, TokenKind::Newline) {
            self.advance();
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> &Token {
        let current = &self.tokens[self.position];
        if !matches!(current.kind, TokenKind::End) {
            self.position += 1;
        }
        current
    }

    fn error_here(&self, message: impl Into<String>) -> ZlangError {
        ZlangError::source(self.current().line, self.current().column, message)
    }
}

fn same_variant(left: &TokenKind, right: &TokenKind) -> bool {
    matches!(
        (left, right),
        (TokenKind::Let, TokenKind::Let)
            | (TokenKind::Print, TokenKind::Print)
            | (TokenKind::Plus, TokenKind::Plus)
            | (TokenKind::Minus, TokenKind::Minus)
            | (TokenKind::Star, TokenKind::Star)
            | (TokenKind::Slash, TokenKind::Slash)
            | (TokenKind::Equal, TokenKind::Equal)
            | (TokenKind::LParen, TokenKind::LParen)
            | (TokenKind::RParen, TokenKind::RParen)
            | (TokenKind::Newline, TokenKind::Newline)
            | (TokenKind::End, TokenKind::End)
    )
}
