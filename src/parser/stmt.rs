use std::collections::HashMap;

use super::{expr::*, token::Literal};

#[derive(Debug)]
pub enum Stmt{
    Expr(Expr),
    Print(Expr),
    //Assign(Identifier, Expression)
    Assign(String, Expr),
    //Reassign(Identifier, Expression)
    Reassign(String, Expr),
    Block(Block),
}

impl Stmt{
    pub fn execute(&self, variables: &mut HashMap<String, Literal>){
        match self{
            Stmt::Print(expr) => {
                let res = expr.solve(&variables);
                match res{
                    Ok(literal) => println!("{}", literal.to_string()),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        return;
                    }
                }
            }
            Stmt::Assign(name, expr) => {
                let res = expr.solve(&variables);
                match res{
                    Ok(literal) => {
                        variables.insert(name.to_owned(), literal);
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        return;
                    }
                }
            }
            Stmt::Reassign(name, expr) => {
                let res = expr.solve(&variables);
                match res{
                    Ok(literal) => {
                        if variables.contains_key(name){
                            variables.insert(name.to_owned(), literal);
                        } else{
                            eprintln!("Variable {} does not exist!", name);
                            return;
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        return;
                    }
                }
            }
            Stmt::Expr(expr) => {
                let _ = expr.solve(&variables);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Block{
    pub stmts: Vec<Stmt>,
    //The list of variables in the scope of the current block
    pub vars: HashMap<String, Literal>,
}

impl Block{
    pub fn new(stmts: Vec<Stmt>) -> Self{
        Self{
            stmts,
            vars: HashMap::new(),
        }
    }

    pub fn execute(&mut self){
        for stmt in self.stmts.iter(){
            stmt.execute(&mut self.vars);
        }
    }
}