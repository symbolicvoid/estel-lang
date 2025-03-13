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
        let mut errors: Vec<StmtError> = Vec::new();
        while self.get_current_token().class != TokenType::Eof {
            let stmt = self.make_block();
            match stmt {
                Ok(option_stmt) => {
                    if let Some(stmt) = option_stmt {
                        stmts.push(stmt);
                    }
                }
                Err(mut errs) => errors.append(&mut errs.errors),
            }
        }
        //check if errors occured
        if !errors.is_empty() {
            Err(StmtErrors { errors })
        } else {
            Ok(Block::new(stmts))
        }
    }

    fn make_block(&mut self) -> Result<Option<Stmt>, StmtErrors> {
        //check for tokens that specify a block: {
        match self.get_current_token().class {
            //Left brace starts a block that ends with a right brace
            TokenType::Lbrace => {
                //store starting token in case of error
                let start = self.get_current_token().to_owned();
                //consume the starting brace
                self.consume();
                let mut stmts = Vec::new();
                let mut errors = Vec::new();
                while self.get_current_token().class != TokenType::Rbrace {
                    if self.get_current_token().class == TokenType::Eof {
                        //return unterminated block error
                        errors.push(StmtError::UnterminatedBlock(start));
                        break;
                    }
                    let stmt = self.make_block();
                    match stmt {
                        Ok(option_stmt) => {
                            if let Some(stmt) = option_stmt {
                                stmts.push(stmt);
                            }
                        }
                        Err(mut errs) => errors.append(&mut errs.errors),
                    }
                }
                //consume the right brace
                self.consume();
                if errors.is_empty() {
                    Ok(Some(Stmt::Block(stmts)))
                } else {
                    Err(StmtErrors { errors })
                }
            }
            //handle while loop
            TokenType::Keyword(Keyword::While) => {
                self.consume();
                //look for parenthesis that specify the condition
                if self.get_current_token().class != TokenType::Lparen {
                    //error if no parenthesis
                    return Err(StmtErrors {
                        errors: vec![StmtError::ExpectToken(
                            TokenType::Lparen,
                            self.get_current_token().to_owned(),
                        )],
                    });
                }
                //save the parenthesis token incase of error
                let paren_start = self.get_current_token().to_owned();
                let mut errors = Vec::new();
                self.consume();
                //tokens of the condition expression
                let mut condition_tokens = Vec::new();
                //expression begins after left parenthesis and ends at a right parenthesis
                while self.get_current_token().class != TokenType::Rparen {
                    if self.get_current_token().class == TokenType::Eof
                        || self.get_current_token().class == TokenType::Lbrace
                        || self.get_current_token().class == TokenType::Rbrace
                    {
                        errors.push(StmtError::UnterminatedParenthesis(paren_start));
                        return Err(StmtErrors { errors });
                    }
                    condition_tokens.push(self.get_current_token().to_owned());
                    self.consume();
                }
                //create an expression for the loop condition
                let expr = self.make_expr(condition_tokens);
                let cond = match expr {
                    Ok(expr) => {
                        match expr {
                            Some(expr) => expr,
                            //error for missing expression
                            None => {
                                errors.push(StmtError::ExpectedExpression(paren_start));
                                //consume the right parenthesis
                                self.consume();
                                return Err(StmtErrors { errors });
                            }
                        }
                    }
                    //error for invalid expressions
                    Err(expr_error) => {
                        errors.push(StmtError::InvalidExpression(expr_error));
                        //consume the right parenthesis
                        self.consume();
                        return Err(StmtErrors { errors });
                    }
                };
                //consume the right parenthesis
                self.consume();
                //parse the body of the loop
                let stmts = self.make_block();
                match stmts {
                    Ok(option_stmt) => match option_stmt {
                        Some(body) => {
                            Ok(Some(Stmt::While(cond, Box::new(body))))
                        }
                        None => {
                            Ok(Some(Stmt::While(cond, Box::new(Stmt::None))))
                        }
                    },
                    Err(mut errs) => {
                        errors.append(&mut errs.errors);
                        Err(StmtErrors { errors })
                    }
                }
            }
            //handle right braces with no corresponding left brace
            TokenType::Rbrace => {
                let right_brace = self.get_current_token().to_owned();
                self.consume();
                //synchronize the position to the next line
                while self.get_current_token().class != TokenType::StmtEnd {
                    if self.get_current_token().class == TokenType::Eof {
                        break;
                    }
                    self.consume();
                }
                Err(StmtErrors {
                    errors: vec![StmtError::UnexpectedBlockClose(right_brace)],
                })
            }
            //other tokens means there is a non-block statement
            _ => {
                //find the stmtend token and save all tokens before it
                let mut stmt_tokens = Vec::new();
                while self.get_current_token().class != TokenType::StmtEnd {
                    stmt_tokens.push(self.get_current_token().to_owned());
                    self.consume();
                    if self.get_current_token().class == TokenType::Eof
                        || self.get_current_token().class == TokenType::Rbrace
                        || self.get_current_token().class == TokenType::Lbrace
                    {
                        break;
                    }
                }
                if stmt_tokens.is_empty() {
                    self.consume();
                    return Ok(None);
                }
                let stmt = self.make_statement(stmt_tokens);
                match stmt {
                    Ok(stmt) => Ok(Some(stmt)),
                    Err(err) => Err(StmtErrors { errors: vec![err] }),
                }
            }
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
                            //redo the entire loop if a unary operator was found
                            tokens.push(token);
                            continue;
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
            match &parse_result.unwrap().stmts[0] {
                Stmt::Expr(expr) => assert_eq!(expr, expect),
                _ => panic!("Stmt is not an expr statement"),
            }
        }
    }

    fn compare_parse_results(src: &[&str], expected: &[Block]) {
        for (line, expect) in src.iter().zip(expected) {
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();

            let parse_result = Parser::new(&tokens).parse();
            assert_eq!(&parse_result.unwrap(), expect);
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
            "5 - 6 + 3",
            "5 + 3 - 6",
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
            Expr::new_add(
                Expr::new_sub(Expr::new_num_literal(5), Expr::new_num_literal(6)),
                Expr::new_num_literal(3),
            ),
            Expr::new_sub(
                Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3)),
                Expr::new_num_literal(6),
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
    fn parse_complex_unary_ops() {
        let src = [
            "5 - -5",
            "6 + -5 * 3",
            "5 * -5 + 3",
            "-5 * -5",
            "6 - -5 + 25 * 16",
        ];
        let expected = [
            Expr::new_sub(
                Expr::new_num_literal(5),
                Expr::Negate(Box::new(Expr::new_num_literal(5))),
            ),
            Expr::new_add(
                Expr::new_num_literal(6),
                Expr::new_mul(
                    Expr::Negate(Box::new(Expr::new_num_literal(5))),
                    Expr::new_num_literal(3),
                ),
            ),
            Expr::new_add(
                Expr::new_mul(
                    Expr::new_num_literal(5),
                    Expr::Negate(Box::new(Expr::new_num_literal(5))),
                ),
                Expr::new_num_literal(3),
            ),
            Expr::new_mul(
                Expr::Negate(Box::new(Expr::new_num_literal(5))),
                Expr::Negate(Box::new(Expr::new_num_literal(5))),
            ),
            Expr::new_add(
                Expr::new_sub(
                    Expr::new_num_literal(6),
                    Expr::Negate(Box::new(Expr::new_num_literal(5))),
                ),
                Expr::new_mul(Expr::new_num_literal(25), Expr::new_num_literal(16)),
            ),
        ];

        compare_expr_parse_results(&src, &expected);
    }

    #[test]
    fn parse_block() {
        let src = vec![
            "
                {
                    print a;
                }
            ",
            "
                let a = 5;
                {
                    let b = 4;
                    a = 3;
                }

            ",
            "
                {
                    {
                        print 5;
                        {
                            {
                            }
                        }
                    }
                    print \"Hi\";
                }
            ",
        ];
        let expected = vec![
            Block::new(vec![Stmt::Block(vec![Stmt::Print(Expr::Ident(
                String::from("a"),
            ))])]),
            Block::new(vec![
                Stmt::Assign(String::from('a'), Expr::new_num_literal(5)),
                Stmt::Block(vec![
                    Stmt::Assign(String::from('b'), Expr::new_num_literal(4)),
                    Stmt::Reassign(String::from('a'), Expr::new_num_literal(3)),
                ]),
            ]),
            Block::new(vec![Stmt::Block(vec![
                Stmt::Block(vec![
                    Stmt::Print(Expr::new_num_literal(5)),
                    Stmt::Block(vec![Stmt::Block(Vec::new())]),
                ]),
                Stmt::Print(Expr::new_string_literal("Hi")),
            ])]),
        ];
        compare_parse_results(&src, &expected);
    }

    #[test]
    fn parse_while() {
        let src = vec![
            "
                while (a < 5){
                    print a;
                    a = a + 1;
                }
            ",
            "
                while (a < 5){
                    while (b < 5){
                        print b;
                        b = b + 1;
                    }
                    print a;
                    a = a + 1;
                }
            ",
            "
                {while (true){
                    let a = 5;
                    {
                        let b = 7;
                        a = a + b;
                    }
                    print a;
                    a = a + 1;
                }}
            ",
        ];
        let expected = vec![
            Block::new(vec![Stmt::While(
                Expr::new_less(Expr::new_ident("a"), Expr::new_num_literal(5)),
                Box::new(Stmt::Block(vec![
                    Stmt::Print(Expr::Ident(String::from("a"))),
                    Stmt::Reassign(
                        String::from("a"),
                        Expr::new_add(Expr::Ident(String::from("a")), Expr::new_num_literal(1)),
                    ),
                ])),
            )]),
            Block::new(vec![Stmt::While(
                Expr::new_less(Expr::new_ident("a"), Expr::new_num_literal(5)),
                Box::new(Stmt::Block(vec![
                    Stmt::While(
                        Expr::new_less(Expr::new_ident("b"), Expr::new_num_literal(5)),
                        Box::new(Stmt::Block(vec![
                            Stmt::Print(Expr::Ident(String::from("b"))),
                            Stmt::Reassign(
                                String::from("b"),
                                Expr::new_add(
                                    Expr::Ident(String::from("b")),
                                    Expr::new_num_literal(1),
                                ),
                            ),
                        ])),
                    ),
                    Stmt::Print(Expr::Ident(String::from("a"))),
                    Stmt::Reassign(
                        String::from("a"),
                        Expr::new_add(Expr::Ident(String::from("a")), Expr::new_num_literal(1)),
                    ),
                ])),
            )]),
            Block::new(vec![Stmt::Block(vec![Stmt::While(
                Expr::new_bool_literal(true),
                Box::new(Stmt::Block(vec![
                    Stmt::Assign(String::from("a"), Expr::new_num_literal(5)),
                    Stmt::Block(vec![
                        Stmt::Assign(String::from("b"), Expr::new_num_literal(7)),
                        Stmt::Reassign(
                            String::from("a"),
                            Expr::new_add(
                                Expr::Ident(String::from("a")),
                                Expr::Ident(String::from("b")),
                            ),
                        ),
                    ]),
                    Stmt::Print(Expr::Ident(String::from("a"))),
                    Stmt::Reassign(
                        String::from("a"),
                        Expr::new_add(Expr::Ident(String::from("a")), Expr::new_num_literal(1)),
                    ),
                ])),
            )])]),
        ];
        compare_parse_results(&src, &expected);
    }

    fn compare_stmt_errors(src: &[&str], expected: &[StmtErrors]) {
        for (code, err) in src.iter().zip(expected) {
            let mut lexer = Lexer::new(code);
            let tokens = lexer.lex();
            let parse_result = Parser::new(&tokens).parse();
            if let Err(errors) = parse_result {
                assert_eq!(&errors, err);
            } else {
                panic!("Expected errors but got none");
            }
        }
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

    #[test]
    fn test_block_errors() {
        let src = [
            "
                let a = 5; }
            ",
            "
                let a = 5;
                {}{}}
                print \"Hello\"; {
                print \"Hi\";
            ",
            "
                {
            ",
        ];
        let expected = [
            StmtErrors {
                errors: vec![StmtError::UnexpectedBlockClose(Token {
                    class: TokenType::Rbrace,
                    line: 2,
                    start: 27,
                })],
            },
            StmtErrors {
                errors: vec![
                    StmtError::UnexpectedBlockClose(Token {
                        class: TokenType::Rbrace,
                        line: 3,
                        start: 20,
                    }),
                    StmtError::UnterminatedBlock(Token {
                        class: TokenType::Lbrace,
                        line: 4,
                        start: 31,
                    }),
                ],
            },
            StmtErrors {
                errors: vec![StmtError::UnterminatedBlock(Token {
                    class: TokenType::Lbrace,
                    line: 2,
                    start: 16,
                })],
            },
        ];
        compare_stmt_errors(&src, &expected);
    }

    #[test]
    fn test_while_errors() {
        let src = [
            "while(){print a;}",
            "while(a){print b;",
            "while(a +) print c;",
        ];
        let expected = [
            StmtErrors {
                errors: vec![StmtError::ExpectedExpression(Token {
                    class: TokenType::Lparen,
                    line: 1,
                    start: 5,
                })],
            },
            StmtErrors {
                errors: vec![StmtError::UnterminatedBlock(Token {
                    class: TokenType::Lbrace,
                    line: 1,
                    start: 8,
                })],
            },
            StmtErrors {
                errors: vec![StmtError::InvalidExpression(ExprError::ExpectTokenError(
                    ExpectType::Operand,
                    Token {
                        class: TokenType::Rparen,
                        line: 1,
                        start: 9,
                    },
                ))],
            },
        ];
        compare_stmt_errors(&src, &expected);
    }
}
