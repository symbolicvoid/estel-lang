use super::errors::{ExprError, StmtError, StmtErrors};
use super::expr::*;
use super::stmt::*;
use super::token::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: u32,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Self { tokens, pos: 0 }
    }

    //parse the tokens into an expression
    //can take in global scope variables
    pub fn parse(&mut self) -> Result<Block, StmtErrors> {
        let mut stmts = Vec::new();
        let mut errs: Vec<StmtError> = Vec::new();
        while self.get_current_token().class != TokenType::Eof {
            //find the stmtend token and save all tokens before it
            let mut stmt_tokens = Vec::new();
            while self.get_current_token().class != TokenType::StmtEnd {
                stmt_tokens.push(self.get_current_token().to_owned());
                self.consume();
                if self.get_current_token().class == TokenType::Eof {
                    break;
                }
            }
            if stmt_tokens.is_empty() {
                self.consume();
                continue;
            }
            let stmt = self.make_statement(stmt_tokens);
            match stmt {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    errs.push(err);
                    self.consume();
                }
            }
        }
        //check if errors occured0
        if !errs.is_empty() {
            Err(StmtErrors { errors: errs })
        } else {
            Ok(Block::new(stmts))
        }
    }

    //function to create a stmt from a vector of tokens
    fn make_statement(&mut self, mut stmt_tokens: Vec<Token>) -> Result<Stmt, StmtError> {
        match &stmt_tokens[0].class {
            TokenType::Keyword(Keyword::Let) => self.make_let_stmt(stmt_tokens),
            TokenType::Keyword(Keyword::Print) => self.make_print_stmt(stmt_tokens),
            TokenType::Ident(_) => self.make_ident_stmt(stmt_tokens),
            TokenType::Literal(_) | TokenType::Lparen | TokenType::Unary(_) => {
                self.make_expr_stmt(stmt_tokens)
            }
            //use swap remove since we dont care about the vector anymore
            _ => Err(StmtError::InvalidStartToken(stmt_tokens.swap_remove(0))),
        }
    }

    fn make_let_stmt(&mut self, mut tokens: Vec<Token>) -> Result<Stmt, StmtError> {
        let ident;
        if tokens.len() < 3 {
            return Err(StmtError::IncompleteStatement(tokens.swap_remove(0)));
        }
        //check for identifier after the let keyword
        match &tokens[1].class {
            TokenType::Ident(name) => {
                ident = name.to_owned();
            }
            _ => {
                return Err(StmtError::ExpectToken(
                    TokenType::Ident(String::new()),
                    tokens.swap_remove(1),
                ))
            }
        };
        //check for assign token after the identifier
        match &tokens[2].class {
            TokenType::Assign => {}
            _ => {
                return Err(StmtError::ExpectToken(
                    TokenType::Assign,
                    tokens.swap_remove(2),
                ))
            }
        };

        let expr = self.make_expr(tokens[3..].to_vec());
        Ok(Stmt::Assign(ident, self.check_expression(expr)?))
    }

    fn make_print_stmt(&mut self, tokens: Vec<Token>) -> Result<Stmt, StmtError> {
        let expr = self.make_expr(tokens[1..].to_vec());
        Ok(Stmt::Print(self.check_expression(expr)?))
    }

    fn make_ident_stmt(&mut self, mut tokens: Vec<Token>) -> Result<Stmt, StmtError> {
        //check the length of the vector, if only one its an expression statement

        if tokens.len() == 1 {
            let expr = self.make_expr(tokens);
            return Ok(Stmt::Expr(self.check_expression(expr)?));
        }

        //check for assignment operator after the identifier
        //if there is no assignment operator, return an expression statement
        if let TokenType::Assign = &tokens[1].class {
            let expr = self.make_expr(tokens[2..].to_vec());
            Ok(Stmt::Reassign(
                match tokens.swap_remove(0).class {
                    TokenType::Ident(name) => name,
                    _ => panic!(),
                },
                self.check_expression(expr)?,
            ))
        } else {
            let expr = self.make_expr(tokens);
            Ok(Stmt::Expr(self.check_expression(expr)?))
        }
    }

    fn make_expr_stmt(&mut self, tokens: Vec<Token>) -> Result<Stmt, StmtError> {
        let expr = self.make_expr(tokens);
        Ok(Stmt::Expr(self.check_expression(expr)?))
    }

    //Create an expression tree using shunting yard algorithm
    fn make_expr(&mut self, mut tokens: Vec<Token>) -> Result<Option<Expr>, ExprError> {
        let mut operands: Vec<Expr> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();
        //Holds the currently expected token, eg- expecting an operator after operand
        let mut expect = ExpectType::Operand;
        tokens.reverse();

        //check for empty list of tokens
        if tokens.is_empty() {
            return Ok(None);
        }

        while let Some(token) = tokens.pop() {
            match &token.class {
                TokenType::Literal(lit) => {
                    if expect == ExpectType::Operator {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    operands.push(Expr::new_literal(lit));
                    expect = ExpectType::Operator;
                }
                TokenType::Ident(name) => {
                    if expect == ExpectType::Operator {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    operands.push(Expr::new_ident(name));
                    expect = ExpectType::Operator;
                }
                TokenType::Operator(op) => {
                    if expect == ExpectType::Operand {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    match operators.last().map(|t| &t.class) {
                        Some(TokenType::Operator(top)) => {
                            if top.precedence() >= op.precedence() {
                                let right = operands.pop().unwrap();
                                let expr = Expr::new_binary_op(operands.pop().unwrap(), right, top);
                                operands.push(expr);
                                operators.pop();
                                operators.push(token);
                            } else {
                                operators.push(token);
                            }
                        }
                        Some(TokenType::Unary(top)) => {
                            let right = operands.pop().unwrap();
                            let expr = Expr::new_unary_op(right, top);
                            operands.push(expr);
                            operators.pop();
                            operators.push(token);
                        }
                        _ => {
                            operators.push(token);
                        }
                    }
                    expect = ExpectType::Operand;
                }
                TokenType::Unary(_) => {
                    if expect == ExpectType::Operator {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    operators.push(token);
                }
                TokenType::Lparen => {
                    //expect parenthesis only after an operand or at the start
                    if expect == ExpectType::Operator {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    operators.push(token);
                }
                TokenType::Rparen => {
                    //Expect Rparen after an operand
                    if expect == ExpectType::Operand {
                        return Err(ExprError::ExpectTokenError(expect, token));
                    }
                    while let Some(top) = operators.last() {
                        if let TokenType::Lparen = top.class {
                            operators.pop();
                            break;
                        } else {
                            let right = operands.pop().unwrap();
                            if let TokenType::Operator(opr) = &top.class {
                                let expr = Expr::new_binary_op(operands.pop().unwrap(), right, opr);
                                operands.push(expr);
                                operators.pop();
                            }
                        }
                    }
                }
                _ => return Err(ExprError::ExpectTokenError(ExpectType::Operand, token)),
            }
        }

        //If the expression ended while expecting an operand, the expression is imcomplete
        if expect == ExpectType::Operand {
            return Err(ExprError::ExpectTokenError(
                expect,
                self.get_current_token().clone(),
            ));
        }

        //Pop the remaining operators
        while let Some(top) = operators.last() {
            match &top.class {
                TokenType::Lparen => {
                    return Err(ExprError::UnterminatedParenthesis(top.clone()));
                }
                TokenType::Operator(opr) => {
                    let right = operands.pop().unwrap();
                    let expr = Expr::new_binary_op(operands.pop().unwrap(), right, opr);
                    operands.push(expr);
                    operators.pop();
                }
                TokenType::Unary(unr) => {
                    let expr = Expr::new_unary_op(operands.pop().unwrap(), unr);
                    operands.push(expr);
                    operators.pop();
                }
                _ => {}
            }
        }
        //return the last operand
        Ok(Some(operands.pop().unwrap()))
    }

    //Checks the expression, if invalid return a StmtError else return the unwrapped Expr
    fn check_expression(
        &mut self,
        expr: Result<Option<Expr>, ExprError>,
    ) -> Result<Expr, StmtError> {
        match expr {
            Ok(expr) => {
                if let Some(expr) = expr {
                    Ok(expr)
                } else {
                    Err(StmtError::ExpectedExpression(
                        self.get_current_token().clone(),
                    ))
                }
            }
            Err(err) => {
                //Return an invalid expression error if the expression is invalid
                //using the expression's error
                Err(StmtError::InvalidExpression(err))
            }
        }
    }

    //advances the position
    fn consume(&mut self) {
        self.pos += 1;
    }

    //return the token at the current pos
    //return the last EOF otherwise
    fn get_current_token(&self) -> &Token {
        let pos = self.pos as usize;
        if self.is_eof(pos) {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[pos]
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer::*;
    use super::*;

    fn compare_expr_parse_results(src: &[&str], expected: &[Expr]) {
        for (line, expect) in src.iter().zip(expected) {
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();

            let parse_result = Parser::new(&tokens).parse();
            println!("{:?}", parse_result);
            match &parse_result.unwrap().stmts[0] {
                Stmt::Expr(expr) => assert_eq!(expr, expect),
                _ => panic!("Stmt is not an expr statement"),
            }
        }
    }

    #[test]
    fn parse_binary_ops() {
        let src = ["4+5", "7-2", "8*2", "5/5"];
        let expected = [
            Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(5)),
            Expr::new_sub(Expr::new_num_literal(7), Expr::new_num_literal(2)),
            Expr::new_mul(Expr::new_num_literal(8), Expr::new_num_literal(2)),
            Expr::new_div(Expr::new_num_literal(5), Expr::new_num_literal(5)),
        ];
        compare_expr_parse_results(&src, &expected);
    }

    #[test]
    fn parse_complex_numeric_ops() {
        let src = [
            "5 * 5 + 3",
            "(4) * (5)",
            "5 * (5 + 3)",
            "5 * (5 + 3) * 2",
            "(4-2) * 7 / (4+3)",
            "(5-5) * (2+8)",
        ];
        let expected = [
            Expr::new_add(
                Expr::new_mul(Expr::new_num_literal(5), Expr::new_num_literal(5)),
                Expr::new_num_literal(3),
            ),
            Expr::new_mul(Expr::new_num_literal(4), Expr::new_num_literal(5)),
            Expr::new_mul(
                Expr::new_num_literal(5),
                Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3)),
            ),
            Expr::new_mul(
                Expr::new_mul(
                    Expr::new_num_literal(5),
                    Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3)),
                ),
                Expr::new_num_literal(2),
            ),
            Expr::new_div(
                Expr::new_mul(
                    Expr::new_sub(Expr::new_num_literal(4), Expr::new_num_literal(2)),
                    Expr::new_num_literal(7),
                ),
                Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(3)),
            ),
            Expr::new_mul(
                Expr::new_sub(Expr::new_num_literal(5), Expr::new_num_literal(5)),
                Expr::new_add(Expr::new_num_literal(2), Expr::new_num_literal(8)),
            ),
        ];
        compare_expr_parse_results(&src, &expected);
    }

    #[test]
    fn parse_identifier_ops() {
        let src = [
            "a + 5",
            "a + b",
            "a + b + c",
            "a + b * c",
            "a * (b + c)",
            "(a + b) * c",
            "(a) * (b)",
        ];
        let expected = [
            Expr::new_add(Expr::new_ident("a"), Expr::new_num_literal(5)),
            Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b")),
            Expr::new_add(
                Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b")),
                Expr::new_ident("c"),
            ),
            Expr::new_add(
                Expr::new_ident("a"),
                Expr::new_mul(Expr::new_ident("b"), Expr::new_ident("c")),
            ),
            Expr::new_mul(
                Expr::new_ident("a"),
                Expr::new_add(Expr::new_ident("b"), Expr::new_ident("c")),
            ),
            Expr::new_mul(
                Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b")),
                Expr::new_ident("c"),
            ),
            Expr::new_mul(Expr::new_ident("a"), Expr::new_ident("b")),
        ];
        compare_expr_parse_results(&src, &expected);
    }

    #[test]
    fn parse_unary_ops() {
        let src = [
            "-5",
            "-a",
            "-(5+5)",
            "-(a+b)",
            "!a",
            "!(a or b)",
            "!a and b",
        ];
        let expected = [
            Expr::Negate(Box::new(Expr::new_num_literal(5))),
            Expr::Negate(Box::new(Expr::new_ident("a"))),
            Expr::Negate(Box::new(Expr::new_add(
                Expr::new_num_literal(5),
                Expr::new_num_literal(5),
            ))),
            Expr::Negate(Box::new(Expr::new_add(
                Expr::new_ident("a"),
                Expr::new_ident("b"),
            ))),
            Expr::Not(Box::new(Expr::new_ident("a"))),
            Expr::Not(Box::new(Expr::Or(
                Box::new(Expr::new_ident("a")),
                Box::new(Expr::new_ident("b")),
            ))),
            Expr::And(
                Box::new(Expr::Not(Box::new(Expr::new_ident("a")))),
                Box::new(Expr::new_ident("b")),
            ),
        ];
        compare_expr_parse_results(&src, &expected);
    }

    #[test]
    fn test_expr_errors() {
        let src = vec!["5 + ;", "5 + 5 + \n", "5 + 5 + *", "5 + ="];
        let error = vec![
            ExprError::ExpectTokenError(
                ExpectType::Operand,
                Token {
                    class: TokenType::StmtEnd,
                    line: 1,
                    start: 4,
                },
            ),
            ExprError::ExpectTokenError(
                ExpectType::Operand,
                Token {
                    class: TokenType::StmtEnd,
                    line: 1,
                    start: 8,
                },
            ),
            ExprError::ExpectTokenError(
                ExpectType::Operand,
                Token {
                    class: TokenType::Operator(Operator::Mul),
                    line: 1,
                    start: 8,
                },
            ),
            ExprError::ExpectTokenError(
                ExpectType::Operand,
                Token {
                    class: TokenType::Assign,
                    line: 1,
                    start: 4,
                },
            ),
        ];

        for (line, expect) in src.iter().zip(error) {
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();
            let parse_result = Parser::new(&tokens).parse();
            if let Err(errors) = parse_result {
                if let StmtError::InvalidExpression(err) = &errors.errors[0] {
                    assert_eq!(err, &expect);
                } else {
                    panic!(
                        "Expected an invalid expression error but got {:?}",
                        errors.errors[0]
                    );
                }
            } else {
                panic!("Expected an error but got none");
            }
        }
    }

    #[test]
    fn test_stmt_errors() {
        let src = vec!["let", "let a", "let = 5"];
        let expected = vec![
            StmtError::IncompleteStatement(Token {
                class: TokenType::Keyword(Keyword::Let),
                line: 1,
                start: 0,
            }),
            StmtError::IncompleteStatement(Token {
                class: TokenType::Keyword(Keyword::Let),
                line: 1,
                start: 0,
            }),
            StmtError::ExpectToken(
                TokenType::Ident(String::new()),
                Token {
                    class: TokenType::Assign,
                    line: 1,
                    start: 4,
                },
            ),
        ];
        for (line, err) in src.iter().zip(expected) {
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();
            let parse_result = Parser::new(&tokens).parse();
            if let Err(errors) = parse_result {
                //make sure only 1 error occured
                assert!(errors.errors.len() == 1);
                assert_eq!(errors.errors[0], err);
            } else {
                panic!("Expected an error but got none");
            }
        }
    }
}
