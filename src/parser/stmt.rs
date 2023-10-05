use std::collections::HashMap;

use super::{expr::*, token::*};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    //Assign(Identifier, Expression)
    Assign(String, Expr),
    //Reassign(Identifier, Expression)
    //Only assign if the variable exists in scope
    Reassign(String, Expr),
}

impl Stmt {
    //variables: contains the variables in the current scope
    //print_expr_result: whether to print the result of an an Expr statement (printed in prompt mode)
    pub fn execute(&self, block: &mut Block, print_expr_result: bool) {
        match self {
            Stmt::Print(expr) => {
                let res = expr.solve(block);
                match res {
                    Ok(literal) => println!("{}", literal.to_string()),
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::Assign(name, expr) => {
                let res = expr.solve(block);
                match res {
                    Ok(value) => block.insert_var(name, value),
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            //Reassign only if the current variable exists in scope
            Stmt::Reassign(name, expr) => {
                let res = expr.solve(block);
                match res {
                    Ok(value) => {
                        if !block.insert_if_exists(name, value) {
                            eprintln!("Error: Variable {} does not exist in scope", name);
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::Expr(expr) => {
                let res = expr.solve(block);
                match res {
                    Ok(literal) => {
                        if print_expr_result {
                            println!("{}", literal.to_string());
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub stmts: Vec<Stmt>,
    //The list of variables in the scope of the current block
    pub vars: HashMap<String, Literal>,
    pub parent: Option<Box<&'a mut Block<'a>>>,
}

impl<'a> Block<'a> {
    pub fn new(stmts: Vec<Stmt>, parent: Option<&'a mut Block<'a>>) -> Self {
        let parent = parent.map(Box::new);
        Self {
            stmts,
            vars: HashMap::new(),
            parent,
        }
    }

    pub fn execute(&mut self, print_expr_result: bool) {
        let stmts = &self.stmts.clone();
        for stmt in stmts.iter() {
            stmt.execute(self, print_expr_result);
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Literal> {
        if self.vars.contains_key(name) {
            return self.vars.get(name);
        }
        match &self.parent {
            Some(parent) => parent.get_var(name),
            None => None,
        }
    }

    pub fn insert_var(&mut self, name: &str, value: Literal) {
        self.vars.insert(name.to_owned(), value);
    }

    //Insert a variable into the block's map only if it exists
    //Also checks the parent scope and modifies them if it exists in parent scope
    //Return true if the variable was found and modified
    pub fn insert_if_exists(&mut self, name: &str, value: Literal) -> bool {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_owned(), value);
            true
        } else if let Some(ref mut parent) = self.parent {
            parent.insert_if_exists(name, value)
        } else {
            false
        }
    }
}
