use std::collections::HashMap;

use super::{stmt::*, token::*};

//struct that executes the program
pub struct Executor {
    //vector of all scopes
    //children scopes get added to the end of the vector
    pub scopes: Vec<Scope>,
    //whether to print result of Expr statements, as done in prompt mode
    print_expr_result: bool,
}

impl Executor {
    //global: global variables to be loaded loaded before the program executes
    pub fn new(print_expr_result: bool, global: Scope) -> Self {
        //create a vector with the global scope at 0
        Self {
            scopes: vec![global],
            print_expr_result,
        }
    }

    //function to execute an entire program
    //main program also uses the global scope
    pub fn execute_code(&mut self, program: Block) {
        for stmt in &program.stmts {
            self.execute_statement(stmt);
        }
    }

    //function to execute blocks within the program
    fn execute_block(&mut self, block: Block) {
        //create a new scope for all blocks
        let scope = Scope::new();
        //load the scope to the executor
        self.scopes.push(scope);

        for stmt in &block.stmts {
            self.execute_statement(stmt);
        }

        //remove the block's scope after it is done executing
        //the variables of this block are no longer needed
        //since all children of this block have been executed beforehand, this block's scope will be at the end
        self.scopes.pop();
    }

    //function to execute a statement
    fn execute_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let res = expr.solve(self);
                match res {
                    Ok(literal) => println!("{}", literal.to_string()),
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::Assign(name, expr) => {
                let res = expr.solve(self);
                match res {
                    Ok(value) => self.insert_var(name.to_owned(), value),
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            //Reassign only if the current variable exists in scope
            Stmt::Reassign(name, expr) => {
                let res = expr.solve(self);
                match res {
                    Ok(value) => {
                        if !self.insert_if_exists(name.to_owned(), value) {
                            eprintln!("Error: Variable {} does not exist in scope", name);
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::Expr(expr) => {
                let res = expr.solve(self);
                match res {
                    Ok(literal) => {
                        if self.print_expr_result {
                            println!("{}", literal.to_string());
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::While(expr, stmts) => {
                let res = expr.solve(self);
                match res {
                    Ok(mut cond) => {
                        while cond.is_truthy() {
                            let block = Block::new(stmts.to_owned());
                            self.execute_block(block);
                            cond = match expr.solve(self) {
                                Ok(res) => res,
                                Err(err) => {
                                    eprintln!("{:?}", err);
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Stmt::Block(stmts) => {
                let block = Block::new(stmts.to_owned());
                self.execute_block(block);
            }
        }
    }

    //insert a variable to the current block's scope
    //cannot insert to parent scope
    fn insert_var(&mut self, name: String, value: Literal) {
        //insert the variable into the current active scope, which is the last element
        self.scopes.last_mut().unwrap().insert_var(name, value);
    }

    //insert a variable if it already exists in the current or parent scopes
    //return true if successful, false if not
    fn insert_if_exists(&mut self, name: String, value: Literal) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.exists(&name) {
                scope.insert_var(name, value);
                return true;
            }
        }
        false
    }

    //get a variable from either the current scope, or the nearest parent scope
    pub fn get_var(&self, name: &String) -> Option<Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(literal) = scope.get_var(name) {
                return Some(literal.to_owned());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Scope {
    vars: HashMap<String, Literal>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from_hashmap(vars: HashMap<String, Literal>) -> Self {
        Self { vars }
    }

    pub fn insert_var(&mut self, name: String, value: Literal) {
        self.vars.insert(name, value);
    }

    //check if a variable exists in this scope
    pub fn exists(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }

    pub fn get_var(&self, name: &String) -> Option<&Literal> {
        self.vars.get(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::expr::Expr;

    use super::*;

    fn compare_scopes(blocks: Vec<Block>, expected: Vec<Scope>) {
        let mut got = vec![];

        for block in blocks {
            let mut executor = Executor::new(false, Scope::new());
            executor.execute_code(block);
            //use the executor's global scope (first element) to compare results
            got.push(executor.scopes.pop().unwrap());
        }
        let got_scopes = got;
        got_scopes
            .iter()
            .zip(expected.iter())
            .for_each(|(got, expected)| {
                assert_eq!(got.vars, expected.vars);
            });
    }

    #[test]
    fn execute_basic_blocks() {
        let blocks = vec![
            /*
            let a = 3+3
            let b = "hello"
            let c = a*b
            b = 8
            */
            Block::new(vec![
                Stmt::Assign(
                    String::from("a"),
                    Expr::new_add(Expr::new_num_literal(3), Expr::new_num_literal(3)),
                ),
                Stmt::Assign(String::from("b"), Expr::new_string_literal("hello")),
                Stmt::Assign(
                    String::from("c"),
                    Expr::new_mul(
                        Expr::Ident(String::from("a")),
                        Expr::Ident(String::from("b")),
                    ),
                ),
                Stmt::Reassign(String::from("b"), Expr::new_num_literal(8)),
            ]),
            /*
            let a = true
            let b = 5
            let c = a * b
            */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_bool_literal(true)),
                Stmt::Assign(String::from("b"), Expr::new_num_literal(5)),
                Stmt::Assign(
                    String::from("c"),
                    Expr::new_mul(
                        Expr::Ident(String::from("a")),
                        Expr::Ident(String::from("b")),
                    ),
                ),
            ]),
        ];
        let expected_scopes = vec![
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Number(6)),
                    (String::from("b"), Literal::Number(8)),
                    (
                        String::from("c"),
                        Literal::String(String::from("hellohellohellohellohellohello")),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            //Last statement is an invalid operation, thus c should not be stored
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Bool(true)),
                    (String::from("b"), Literal::Number(5)),
                ]
                .into_iter()
                .collect(),
            ),
        ];

        compare_scopes(blocks, expected_scopes);
    }

    #[test]
    fn execute_nested_blocks() {
        let blocks = vec![
            /*
            let a = 3
            {
                let a = 5
                let b = a
            }
            */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_num_literal(3)),
                Stmt::Block(vec![
                    Stmt::Assign(String::from("a"), Expr::new_num_literal(5)),
                    Stmt::Assign(String::from("b"), Expr::Ident(String::from("a"))),
                ]),
            ]),
            /*
            let a = 9
            let c = ""
            {
                let a = 5
                let b = a
                {
                    let a = 8
                    b = a
                }
                c = a * b
            }
            */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_num_literal(9)),
                Stmt::Assign(String::from("c"), Expr::new_string_literal("")),
                Stmt::Block(vec![
                    Stmt::Assign(String::from("a"), Expr::new_num_literal(5)),
                    Stmt::Assign(String::from("b"), Expr::Ident(String::from("a"))),
                    Stmt::Block(vec![
                        Stmt::Assign(String::from("a"), Expr::new_num_literal(8)),
                        Stmt::Reassign(String::from("b"), Expr::Ident(String::from("a"))),
                    ]),
                    Stmt::Reassign(
                        String::from("c"),
                        Expr::new_mul(
                            Expr::Ident(String::from("a")),
                            Expr::Ident(String::from("b")),
                        ),
                    ),
                ]),
            ]),
        ];
        let expected_scopes = vec![
            Scope::from_hashmap(
                vec![(String::from("a"), Literal::Number(3))]
                    .into_iter()
                    .collect(),
            ),
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Number(9)),
                    (String::from("c"), Literal::Number(40)),
                ]
                .into_iter()
                .collect(),
            ),
        ];

        compare_scopes(blocks, expected_scopes);
    }

    #[test]
    fn execute_basic_while_loop() {
        let blocks = vec![
            /*
            let a = 5;
            let b = 3;
            while(a != 0){
                b=b+1;
                a=a-1;
            }
             */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_num_literal(5)),
                Stmt::Assign(String::from("b"), Expr::new_num_literal(3)),
                Stmt::While(
                    Expr::new_not_equal(Expr::new_ident("a"), Expr::new_num_literal(0)),
                    vec![
                        Stmt::Reassign(
                            String::from("b"),
                            Expr::new_add(Expr::new_ident("b"), Expr::new_num_literal(1)),
                        ),
                        Stmt::Reassign(
                            String::from("a"),
                            Expr::new_sub(Expr::new_ident("a"), Expr::new_num_literal(1)),
                        ),
                    ],
                ),
            ]),
            /*
            let a = "Hello";
            let b = "";
            let i = 3;
            while(i != 0){
                b = b + a;
                i = i - 1;
             */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_string_literal("Hello")),
                Stmt::Assign(String::from("b"), Expr::new_string_literal("")),
                Stmt::Assign(String::from("i"), Expr::new_num_literal(3)),
                Stmt::While(
                    Expr::new_not_equal(Expr::new_ident("i"), Expr::new_num_literal(0)),
                    vec![
                        Stmt::Reassign(
                            String::from("b"),
                            Expr::new_add(Expr::new_ident("b"), Expr::new_ident("a")),
                        ),
                        Stmt::Reassign(
                            String::from("i"),
                            Expr::new_sub(Expr::new_ident("i"), Expr::new_num_literal(1)),
                        ),
                    ],
                ),
            ]),
            /*
            let a = true;
            let num = 4;
            let i = 30;
            while(i != 0 and a){
                num = num + 1;
                a = num - 8 != 0;
                i = i - 1;
            }
             */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_bool_literal(true)),
                Stmt::Assign(String::from("num"), Expr::new_num_literal(4)),
                Stmt::Assign(String::from("i"), Expr::new_num_literal(30)),
                Stmt::While(
                    Expr::new_and(
                        Expr::new_not_equal(Expr::new_ident("i"), Expr::new_num_literal(0)),
                        Expr::new_ident("a"),
                    ),
                    vec![
                        Stmt::Reassign(
                            String::from("num"),
                            Expr::new_add(Expr::new_ident("num"), Expr::new_num_literal(1)),
                        ),
                        Stmt::Reassign(
                            String::from("a"),
                            Expr::new_not_equal(
                                Expr::new_sub(Expr::new_ident("num"), Expr::new_num_literal(8)),
                                Expr::new_num_literal(0),
                            ),
                        ),
                        Stmt::Reassign(
                            String::from("i"),
                            Expr::new_sub(Expr::new_ident("i"), Expr::new_num_literal(1)),
                        ),
                    ],
                ),
            ]),
        ];
        let expected_scope = vec![
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Number(0)),
                    (String::from("b"), Literal::Number(8)),
                ]
                .into_iter()
                .collect(),
            ),
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::String(String::from("Hello"))),
                    (
                        String::from("b"),
                        Literal::String(String::from("HelloHelloHello")),
                    ),
                    (String::from("i"), Literal::Number(0)),
                ]
                .into_iter()
                .collect(),
            ),
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Bool(false)),
                    (String::from("num"), Literal::Number(8)),
                    (String::from("i"), Literal::Number(26)),
                ]
                .into_iter()
                .collect(),
            ),
        ];
        compare_scopes(blocks, expected_scope);
    }

    #[test]
    fn execute_nested_while_loop() {
        let blocks = vec![
            /*
            let a = 0;
            let i = 5;
            let j = 5;
            while(i != 0){
                while(j != 0){
                    a = a + 1;
                    j = j - 1;
                }
                i = i - 1;
                j = 5;
             }
             */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_num_literal(0)),
                Stmt::Assign(String::from("i"), Expr::new_num_literal(5)),
                Stmt::Assign(String::from("j"), Expr::new_num_literal(5)),
                Stmt::While(
                    Expr::new_not_equal(Expr::new_ident("i"), Expr::new_num_literal(0)),
                    vec![
                        Stmt::While(
                            Expr::new_not_equal(Expr::new_ident("j"), Expr::new_num_literal(0)),
                            vec![
                                Stmt::Reassign(
                                    String::from("a"),
                                    Expr::new_add(Expr::new_ident("a"), Expr::new_num_literal(1)),
                                ),
                                Stmt::Reassign(
                                    String::from("j"),
                                    Expr::new_sub(Expr::new_ident("j"), Expr::new_num_literal(1)),
                                ),
                            ],
                        ),
                        Stmt::Reassign(
                            String::from("i"),
                            Expr::new_sub(Expr::new_ident("i"), Expr::new_num_literal(1)),
                        ),
                        Stmt::Reassign(String::from("j"), Expr::new_num_literal(5)),
                    ],
                ),
            ]),
            /*
            let a = 0;
            let i = 5;
            while(i != 0){
                while(a%10 != 2 or a < 10){
                    a = a + 1;
                }
                a = a + 1;
                i = i - 1;
            }
             */
            Block::new(vec![
                Stmt::Assign(String::from("a"), Expr::new_num_literal(0)),
                Stmt::Assign(String::from("i"), Expr::new_num_literal(5)),
                Stmt::While(
                    Expr::new_not_equal(Expr::new_ident("i"), Expr::new_num_literal(0)),
                    vec![
                        Stmt::While(
                            Expr::new_or(
                                Expr::new_not_equal(
                                    Expr::new_mod(Expr::new_ident("a"), Expr::new_num_literal(10)),
                                    Expr::new_num_literal(2),
                                ),
                                Expr::new_less(Expr::new_ident("a"), Expr::new_num_literal(10)),
                            ),
                            vec![Stmt::Reassign(
                                String::from("a"),
                                Expr::new_add(Expr::new_ident("a"), Expr::new_num_literal(1)),
                            )],
                        ),
                        Stmt::Reassign(
                            String::from("a"),
                            Expr::new_add(Expr::new_ident("a"), Expr::new_num_literal(1)),
                        ),
                        Stmt::Reassign(
                            String::from("i"),
                            Expr::new_sub(Expr::new_ident("i"), Expr::new_num_literal(1)),
                        ),
                    ],
                ),
            ]),
        ];
        let expected_scope = vec![
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Number(25)),
                    (String::from("i"), Literal::Number(0)),
                    (String::from("j"), Literal::Number(5)),
                ]
                .into_iter()
                .collect(),
            ),
            Scope::from_hashmap(
                vec![
                    (String::from("a"), Literal::Number(53)),
                    (String::from("i"), Literal::Number(0)),
                ]
                .into_iter()
                .collect(),
            ),
        ];
        compare_scopes(blocks, expected_scope);
    }
}
