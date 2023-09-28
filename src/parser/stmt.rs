use super::expr::*;

#[derive(Debug)]
pub enum Stmt{
    Expr(Expr),
    Print(Expr),
    Block(Block),
}

impl Stmt{
    pub fn execute(&self){
        match self{
            Stmt::Print(expr) => {
                if let Some(literal) = expr.solve(){
                    println!("{}", literal.to_string());
                }
            }
            Stmt::Expr(expr) => {
                expr.solve();
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Block{
    pub stmts: Vec<Stmt>,
}

impl Block{
    pub fn execute(&self){
        for stmt in self.stmts.iter(){
            stmt.execute();
        }
    }
}