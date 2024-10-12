use super::expr::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    //Assign(Identifier, Expression)
    Assign(String, Expr),
    //Reassign(Identifier, Expression)
    //Only assign if the variable exists in scope
    Reassign(String, Expr),
    //Block of statements
    Block(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}
