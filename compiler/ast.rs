#[derive(Debug, Clone)]
pub enum Literal { Int(i64), Str(String) }
#[derive(Debug, Clone)]
pub enum BinaryOp { Add, Sub }
#[derive(Debug, Clone)]
pub enum Expr { Literal(Literal), Identifier(String), Binary(Box<Expr>, BinaryOp, Box<Expr>) }
#[derive(Debug, Clone)]
pub enum Stmt { VarDecl(String, Expr), Print(Expr), Hash(Expr), Broadcast(Expr), Exec(Expr), Expr(Expr) }
