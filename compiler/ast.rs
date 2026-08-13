#[derive(Debug, Clone)]
pub enum Literal { Int(i64), Str(String) }

#[derive(Debug, Clone)]
pub enum BinaryOp { Add, Sub, Less, Greater, EqEq }

#[derive(Debug, Clone)]
pub enum Expr { 
    Literal(Literal), Identifier(String), Binary(Box<Expr>, BinaryOp, Box<Expr>) 
}

#[derive(Debug, Clone)]
pub enum Stmt { 
    VarDecl(String, Expr), 
    Assign(String, Expr), // <-- IL NUOVO COMANDO
    Print(Expr), Hash(Expr), Broadcast(Expr), Exec(Expr), Expr(Expr),
    Block(Vec<Stmt>), If(Expr, Box<Stmt>, Option<Box<Stmt>>), While(Expr, Box<Stmt>)
}
