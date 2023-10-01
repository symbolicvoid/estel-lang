use super::token::*;
use super::expr::*;
use super::stmt::*;
use super::errors::{ ExprError, ExpectType, StmtError, StmtErrors };

pub struct Parser<'a>{
    tokens: &'a Vec<Token>,
    pos: u32,
}

impl<'a> Parser<'a>{
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a>{
        Self { tokens, pos: 0}
    }

    //parse the tokens into an expression
    //can take in global scope variables
    pub fn parse(&mut self, global: Option<&'a mut Block<'a>>) -> Result<Block<'a>, StmtErrors>{
        let mut stmts = Vec::new();
        let mut errs: Vec<StmtError> = Vec::new();
        while !self.is_eof(self.pos as usize){
            if self.get_current_token().class == TokenType::Eof{ break; }
            let stmt = self.make_statement();

            match stmt{
                Ok(stmt) => {
                    //consume the StmtEnd token
                    self.consume();
                    stmts.push(stmt)
                },
                Err(err) => {
                    errs.push(err);
                    self.synchronize();
                }
            }
        }
        //check if errors occured
        if errs.len() > 0{
            return Err(StmtErrors{errors: errs});
        } else {
            Ok(Block::new(stmts, global))
        }
    }

    pub fn make_statement(&mut self) -> Result<Stmt, StmtError>{
        match &self.get_current_token().class{

            TokenType::Keyword(Keyword::Print) => {
                self.consume();
                let expr = self.expr();
                //Error if no expr or invalid expr after print
                Ok(Stmt::Print(self.check_expression(expr)?))
            }

            TokenType::Keyword(Keyword::Let) => {
                self.consume();
                let name = match &self.get_current_token().class{
                    TokenType::Ident(name) => name.to_owned(),
                    _ => return Err(
                        StmtError::ExpectToken(TokenType::Ident(String::new()), self.get_current_token().to_owned())
                    ),
                };
                self.consume();
                if let TokenType::Assign = self.get_current_token().class{
                    //Consume the assignment operator
                    self.consume();
                    let expr = self.expr();
                    Ok(Stmt::Assign(name, self.check_expression(expr)?))
                } else {
                    //Expect an assignment operator after identifier
                    Err(
                        StmtError::ExpectToken(TokenType::Assign, self.get_current_token().to_owned())    
                    )
                }
            }
            TokenType::Ident(name/*Borrow checker won't let me use the name here so ¯\_(ツ)_/¯*/) => {
                let name = name.clone();
                match &self.peek().class{
                    TokenType::Assign => {
                        self.consume();
                        self.consume();
                        let expr = self.expr();
                        Ok(Stmt::Reassign(name, self.check_expression(expr)?))
                    }
                    TokenType::Operator(_) => {
                        let expr = self.expr();
                        Ok(Stmt::Expr(self.check_expression(expr)?))
                    }
                    _ => {
                        Err(
                            StmtError::ExpectToken(TokenType::Assign, self.get_current_token().to_owned())
                        )
                    }
                }
            }
            TokenType::Literal(_) | TokenType::Lparen => {
                let expr = self.expr();
                Ok(Stmt::Expr(self.check_expression(expr)?))
            }
            _ => Err(StmtError::InvalidStartToken(self.get_current_token().to_owned())),
        }
    }

    //Checks the expression, if missing or invalid return a StmtError else return the unwrapped Expr
    fn check_expression(&mut self, expr: Result<Option<Expr>, ExprError>) -> Result<Expr, StmtError>{
        match expr{
            Ok(expr) => 
                {
                    match expr{
                        None => Err(
                            //Return a missing expression error if no expression was found
                            StmtError::InvalidExpression(
                                ExprError::ExpectTokenError(
                                    ExpectType::Expression, self.get_current_token().to_owned())
                            )
                        ),
                        Some(expr) => Ok(expr),
                    }
            }
            Err(err) => {
                //Return an invalid expression error if the expression is invalid 
                //using the expression's error
                Err(StmtError::InvalidExpression(err))
            }
        }
    }

    //recursive function to create the expression tree
    //Result: used to return an error if the expression is invalid
    //Option: used to return None if there is no next expression
    fn expr(&mut self) -> Result<Option<Expr>, ExprError>{
        match self.get_current_token().class{
            TokenType::Literal(_) | TokenType::Ident(_) => {
                Ok(
                    Some(
                        {
                            let expr = self.parse_binary_opr()?;
                            //do not consume if we only get a literal or identifier
                            match expr{
                                Expr::Literal(_) | Expr::Ident(_) => expr,
                                _ => {
                                    self.consume();
                                    self.merge_next_expr(expr)?
                                }
                            }
                        }
                    )
                )
            }
            TokenType::Lparen => {
                self.consume();
                //handle empty parenthesis
                if let TokenType::Rparen = self.get_current_token().class{
                    return Err(
                        ExprError::ExpectTokenError(
                            ExpectType::Expression, self.get_current_token().to_owned()
                        )
                    );
                }
                let paren_expr = self.expr()?;
                match paren_expr{
                    None => Err(
                        ExprError::ExpectTokenError(
                            ExpectType::Expression, self.get_current_token().to_owned()
                        )
                    ),
                    Some(expr) =>{
                        let expr = Expr::new_paren(expr);
                        Ok(Some(self.merge_next_expr(expr)?))
                    }
                }
            }
            //If the current token is an Rparen we place None on the binary expression's left side
            //Example (5 + 5) * 3 -> returns Paren(5+5).merge(None * 3) 
            TokenType::Rparen => {
                let left = Expr::None;
                self.consume();
                if let TokenType::Operator(opr) = &self.get_current_token().class{
                    //Check if right is an Lparen such as in case of (4) * (5)
                    let right = match &self.peek().class{
                        TokenType::Lparen => Expr::None,
                        TokenType::Literal(literal) => Expr::new_literal(&literal),
                        TokenType::Ident(name) => Expr::new_ident(&name),
                        _ => 
                        return Err(ExprError::ExpectTokenError(ExpectType::Operand, 
                            self.get_current_token().to_owned()))
                    };
                    Ok(
                        Some(
                            {
                                let expr = Expr::new_binary_op(left, right, opr);
                                self.consume();
                                self.merge_next_expr(expr)?
                            }
                        )
                    )
                } else {
                    self.pos -= 1;
                    Ok(None)
                }
            }
            _ => {
                Ok(None)
            },
        }
    }

    //Takes the current expression and checks for the next one
    //If an expression is found, merge the two expressions
    fn merge_next_expr(&mut self, expr: Expr) -> Result<Expr, ExprError>{
        let next = self.expr()?;
        match next{
            None => Ok(expr),
            Some(next) => {
                Ok(expr.merge(next))
            }
        }
    }

    //Handles when the expression starts with a literal or an identifier
    //Return a binary expression if the next token is an operator else literal expression
    //In case of an Rparen, set the binary's right side to None
    fn parse_binary_opr(&mut self) -> Result<Expr, ExprError>{
        self.consume();
        let left = self.get_previous_token();
        if let TokenType::Operator(opr) = &self.get_current_token().class{
            let right = self.peek();
            let right_expr = match &right.class{
                TokenType::Lparen => Expr::None,
                TokenType::Literal(literal) => Expr::new_literal(&literal),
                TokenType::Ident(name) => Expr::new_ident(&name),
                _ => return Err(ExprError::ExpectTokenError(ExpectType::Operand, right.to_owned())),
            };
            let left_expr = match &left.class{
                TokenType::Literal(literal) => Expr::new_literal(&literal),
                TokenType::Ident(name) => Expr::new_ident(&name),
                _ => return Err(ExprError::ExpectTokenError(ExpectType::Operand, left.to_owned())),
            };
            Ok(
                Expr::new_binary_op(
                    left_expr, 
                    right_expr, 
                    opr
                )
            )
        } else {
            match &left.class{
                TokenType::Literal(literal) => Ok(Expr::new_literal(&literal)),
                TokenType::Ident(name) => Ok(Expr::new_ident(&name)),
                _ => Err(ExprError::ExpectTokenError(ExpectType::Operand, left.to_owned())),
            }
        }
    }

    //Move the current position to the next statement
    //Used when errors occur
    fn synchronize(&mut self){
        self.consume();
        while !self.is_eof(self.pos as usize){
            match self.get_current_token().class{
                TokenType::StmtEnd => {
                    self.consume();
                    return;
                }
                _ => {
                    self.consume();
                }
            }
        }

    }

    fn peek(&self) -> &Token{
        let pos = self.pos as usize + 1;
        if self.is_eof(pos){
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[pos]
        }
    }

    //advances the position
    fn consume(&mut self){
        self.pos += 1;
    }


    //return the token at the current pos
    //return the last EOF otherwise
    fn get_current_token(&self) -> &Token{
        let pos = self.pos as usize;
        if self.is_eof(pos){
            return &self.tokens[self.tokens.len() - 1];
        } 
        &self.tokens[pos]
    }

    fn get_previous_token(&self) -> &Token{
        if self.pos == 0{
            return self.get_current_token()
        }
        let pos = self.pos as usize - 1;
        if self.is_eof(pos){
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[pos]
    }

    fn is_eof(&self, pos: usize) -> bool{
        pos >= self.tokens.len()
    }
}


#[cfg(test)]
mod tests{
    use super::super::lexer::*;
    use super::*;

    fn compare_results(src: Vec<&str>, expected: Vec<Expr>){
        for (line, expect) in src.iter().zip(expected){

            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();

            let parse_result = Parser::new(&tokens).parse(None);
            println!("{:?}", parse_result);
            match &parse_result.unwrap().stmts[0]{
                Stmt::Expr(expr) => assert_eq!(expr.to_owned(), expect),
                _ => panic!("Stmt is not an expr statement"),
            }
        }
    }

    #[test]
    fn parse_binary_ops(){
        let src = vec!("4+5", "7-2", "8*2", "5/5");
        let expected = vec!(
            Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(5)),
            Expr::new_sub(Expr::new_num_literal(7), Expr::new_num_literal(2)), 
            Expr::new_mul(Expr::new_num_literal(8), Expr::new_num_literal(2)),
            Expr::new_div(Expr::new_num_literal(5), Expr::new_num_literal(5)),
        );
        compare_results(src, expected);
    }

    #[test]
    fn parse_complex_numeric_ops(){
        let src = vec!(
            "5 * 5 + 3",
            "(4) * (5)",
            "5 * (5 + 3)",
            "5 * (5 + 3) * 2",
            "(4-2) * 7 / (4+3)",
            "(5-5) * (2+8)",
        );
        let expected = vec!(
            Expr::new_add(
                Expr::new_mul(Expr::new_num_literal(5), Expr::new_num_literal(5)), 
                Expr::new_num_literal(3)
            ),
            Expr::new_mul(
                Expr::new_paren(Expr::new_num_literal(4)), 
                Expr::new_paren(Expr::new_num_literal(5))
            ),
            Expr::new_mul(
                Expr::new_num_literal(5), 
                Expr::new_paren(
                    Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3))
                )
            ),
            Expr::new_mul(
                Expr::new_mul(
                    Expr::new_num_literal(5), 
                    Expr::new_paren(
                        Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3))
                    )
                ),
                Expr::new_num_literal(2)
            ),
            Expr::new_mul(
                Expr::new_paren(
                    Expr::new_sub(Expr::new_num_literal(4), Expr::new_num_literal(2))
                ),
                Expr::new_div(
                    Expr::new_num_literal(7), 
                    Expr::new_paren(
                        Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(3))
                    )
                )
            ),
            Expr::new_mul(
                Expr::new_paren(
                    Expr::new_sub(Expr::new_num_literal(5), Expr::new_num_literal(5))
                ),
                Expr::new_paren(
                    Expr::new_add(Expr::new_num_literal(2), Expr::new_num_literal(8))
                )
            ),
        );
        compare_results(src, expected);
    }

    #[test]
    fn parse_identifier_ops(){
        let src = vec![
            "a + 5",
            "a + b",
            "a + b + c",
            "a + b * c",
            "a * (b + c)",
            "(a + b) * c",
            "(a) * (b)"
        ];
        let expected = vec![
            Expr::new_add(Expr::new_ident("a"), Expr::new_num_literal(5)),
            Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b")),
            Expr::new_add(
                Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b")), 
                Expr::new_ident("c")
            ),
            Expr::new_add(
                Expr::new_ident("a"), 
                Expr::new_mul(Expr::new_ident("b"), Expr::new_ident("c"))
            ),
            Expr::new_mul(
                Expr::new_ident("a"), 
                Expr::new_paren(Expr::new_add(Expr::new_ident("b"), Expr::new_ident("c")))
            ),
            Expr::new_mul(
                Expr::new_paren(Expr::new_add(Expr::new_ident("a"), Expr::new_ident("b"))), 
                Expr::new_ident("c")
            ),
            Expr::new_mul(
                Expr::new_paren(Expr::new_ident("a")), 
                Expr::new_paren(Expr::new_ident("b"))
            ),
        ];
        compare_results(src, expected);
    }

    #[test]
    fn test_expr_errors(){
        let src = vec![
            "5 + ;",
            "5 + 5 + \n",
            "5 + 5 + *",
            "5 + =",
        ];

        let error = vec![
            ExprError::ExpectTokenError(ExpectType::Operand, Token{
                class: TokenType::StmtEnd, line: 1,  start: 4}
            ),
            ExprError::ExpectTokenError(ExpectType::Operand, Token{
                class: TokenType::StmtEnd, line: 1,  start: 8}
            ),
            ExprError::ExpectTokenError(ExpectType::Operand, Token{
                class: TokenType::Operator(Operator::Mul), line: 1,  start: 8}
            ),
            ExprError::ExpectTokenError(ExpectType::Operand, Token{
                class: TokenType::Assign, line: 1,  start: 4}
            ),
        ];

        for (line, expect) in src.iter().zip(error){
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();
            let parse_result = Parser::new(&tokens).parse(None);
            if let Err(errors) = parse_result{
                if let StmtError::InvalidExpression(err) = &errors.errors[0]{
                    assert_eq!(err, &expect);
                } else {
                    panic!("Expected an invalid expression error but got {:?}", errors.errors[0]);
                }
            } else {
                panic!("Expected an error but got none");
            }
        }
    }

    #[test]
    fn test_stmt_errors(){
        let src = vec![
            "let",
            "let a",
            "let = 5"
        ];
        let expecte = vec![
            StmtError::ExpectToken(TokenType::Ident(String::new()), Token{
                class: TokenType::Eof, line: 1,  start: 3}
            ),
            StmtError::ExpectToken(TokenType::Assign, Token{
                class: TokenType::Eof, line: 1,  start: 5}
            ),
            StmtError::ExpectToken(TokenType::Ident(String::new()), Token{
                class: TokenType::Assign, line: 1,  start: 4}
            ),
        ];
        for (line, err) in src.iter().zip(expecte){
            let mut lexer = Lexer::new(line);
            let tokens = lexer.lex();
            let parse_result = Parser::new(&tokens).parse(None);
            if let Err(errors) = parse_result{
                //make sure only 1 error occured
                assert!(errors.errors.len() == 1);
                assert_eq!(errors.errors[0], err);
            } else {
                panic!("Expected an error but got none");
            }
        }
    }
}