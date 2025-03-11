use super::expr::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    //Assign(Identifier, Expression)
    Assign(String, Expr),
    //Reassign(Identifier, Expression)
    //Only assign if the variable exists in scope
    Reassign(String, Expr),
    //Loop statement
    //While(Condition, Statements)
    While(Expr, Vec<Stmt>),
    //Block of statements
    Block(Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}
