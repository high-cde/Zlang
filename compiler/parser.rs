use crate::compiler::ast::{BinaryOp, Expr, Literal, Stmt};
use crate::compiler::lexer::{Lexer, Token};

pub struct Parser { lexer: Lexer, current_token: Token }

impl Parser {
    pub fn new(lexer: Lexer) -> Self { let mut p = Parser { lexer, current_token: Token::EOF }; p.advance(); p }
    fn advance(&mut self) { self.current_token = self.lexer.next_token(); }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current_token != Token::EOF { stmts.push(self.parse_statement()); }
        stmts
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current_token.clone() {
            Token::Let => self.parse_var_decl(),
            Token::Print => { self.advance(); Stmt::Print(self.parse_expression()) },
            Token::Hash => { self.advance(); Stmt::Hash(self.parse_expression()) },
            Token::Broadcast => { self.advance(); Stmt::Broadcast(self.parse_expression()) },
            Token::Exec => { self.advance(); Stmt::Exec(self.parse_expression()) },
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::LBrace => self.parse_block(),
            Token::Identifier(name) => {
                self.advance(); // Consuma l'identificatore
                if self.current_token == Token::Eq {
                    self.advance(); // Consuma l'uguale '='
                    Stmt::Assign(name, self.parse_expression())
                } else {
                    panic!("Errore sintassi: atteso '=' dopo l'identificatore");
                }
            },
            _ => Stmt::Expr(self.parse_expression()),
        }
    }

    fn parse_block(&mut self) -> Stmt {
        self.advance(); // Salta '{'
        let mut stmts = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            stmts.push(self.parse_statement());
        }
        self.advance(); // Salta '}'
        Stmt::Block(stmts)
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance(); // Salta 'if'
        let cond = self.parse_expression();
        let cons = Box::new(self.parse_block());
        let alt = if self.current_token == Token::Else {
            self.advance();
            Some(Box::new(if self.current_token == Token::If { self.parse_if() } else { self.parse_block() }))
        } else { None };
        Stmt::If(cond, cons, alt)
    }

    fn parse_while(&mut self) -> Stmt {
        self.advance(); // Salta 'while'
        let cond = self.parse_expression();
        let body = Box::new(self.parse_block());
        Stmt::While(cond, body)
    }

    fn parse_var_decl(&mut self) -> Stmt {
        self.advance();
        if let Token::Identifier(name) = self.current_token.clone() {
            self.advance(); self.advance();
            return Stmt::VarDecl(name, self.parse_expression());
        }
        panic!("Errore di sintassi in dichiarazione variabile");
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_additive();
        while matches!(self.current_token, Token::Less | Token::Greater | Token::EqEq) {
            let op = match self.current_token {
                Token::Less => BinaryOp::Less,
                Token::Greater => BinaryOp::Greater,
                Token::EqEq => BinaryOp::EqEq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = if self.current_token == Token::Plus { BinaryOp::Add } else { BinaryOp::Sub };
            self.advance();
            let right = self.parse_primary();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token.clone() {
            Token::Int(i) => { self.advance(); Expr::Literal(Literal::Int(i)) }
            Token::Str(s) => { self.advance(); Expr::Literal(Literal::Str(s)) }
            Token::Identifier(id) => { self.advance(); Expr::Identifier(id) }
            _ => panic!("Errore sintassi: Token inatteso {:?}", self.current_token),
        }
    }
}
