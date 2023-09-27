use super::token::*;
use super::expr::*;

pub struct Parser<'a>{
    tokens: &'a Vec<Token>,
    pos: u32,
}

impl<'a> Parser<'a>{
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a>{
        Self { tokens, pos: 0}
    }

    //parse the tokens into an expression
    pub fn parse(&mut self) -> Option<Expr>{
        self.expr()
    }

    //recursive function to create the ast
    fn expr(&mut self) -> Option<Expr>{
        println!("{:?}", self.get_current_token().class);
        match self.get_current_token().class{
            TokenType::Literal(_) => {
                Some(
                    {
                        let expr = self.literal_start();
                        //do not consume if we only get a literal
                        if let Expr::Literal(_) = expr{
                            expr
                        } else {
                            self.consume();
                            self.merge_next_expr(expr)
                        }
                    }
                )
            }
            TokenType::Lparen => {
                self.consume();
                let paren_expr = self.expr();
                match paren_expr{
                    None => None,
                    Some(expr) =>{
                        let expr = Expr::new_paren(expr);
                        Some(self.merge_next_expr(expr))
                    }
                }
            }
            //If the current token is an Rparen we place None on the binary's left side
            //Example (5 + 5) * 3 -> returns Paren(5+5).merge(None * 3) 
            TokenType::Rparen => {
                let left = Expr::None;
                self.consume();
                if let TokenType::Operator(opr) = &self.get_current_token().class{
                    //Check if right is an Lparen such as in case of (4) * (5)
                    let right = match &self.peek().class{
                        TokenType::Lparen => Expr::None,
                        TokenType::Literal(literal) => Expr::new_literal(&literal),
                        _ => return None,
                    };
                    Some(
                        {
                            let expr = Expr::new_binary_op(left, right, opr);
                            self.consume();
                            self.merge_next_expr(expr)
                        }
                    )
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    //Takes the current expression and checks for the next one
    //If an expression is found, merge the two expressions
    fn merge_next_expr(&mut self, expr: Expr) -> Expr{
        if let Some(next) = self.expr(){

            println!("{:?}\t{:?}", &expr, &next);
            println!("{:?}", expr.clone().merge(next.clone()));

            expr.merge(next)
        } else {
            expr
        }
    }

    //Handles when the expression starts with a literal
    //Return a binary expression if the next token is an operator else literal expression
    //In case of an Rparen, set the binary's right side to None
    fn literal_start(&mut self) -> Expr{
        self.consume();
        let left = self.get_previous_token();
        if let TokenType::Operator(opr) = &self.get_current_token().class{
            let right = self.peek();
            let right_expr = match &right.class{
                TokenType::Rparen => Expr::None,
                TokenType::Literal(literal) => Expr::new_literal(&literal),
                _ => Expr::None,
            };
            Expr::new_binary_op(
                Expr::new_literal(left.class.get_literal().unwrap()), 
                right_expr, 
                opr
            )
        } else {
            Expr::new_literal(left.class.get_literal().unwrap())
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

    #[test]
    pub fn parse_binary_ops(){
        let src = vec!("4+5", "7-2", "8*2", "5/5");
        let expected = vec!(
            Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(5)),
            Expr::new_sub(Expr::new_num_literal(7), Expr::new_num_literal(2)), 
            Expr::new_mul(Expr::new_num_literal(8), Expr::new_num_literal(2)),
            Expr::new_div(Expr::new_num_literal(5), Expr::new_num_literal(5)),
        );
        for (line, expect) in src.iter().zip(expected){
            let mut lexer = Lexer::new(line);
            let parse_result = Parser::new(&lexer.lex()).parse();
            assert_eq!(parse_result.unwrap(), expect);
        }
    }

    #[test]
    pub fn parse_complex_numeric_ops(){
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
        for (line, expect) in src.iter().zip(expected){
            let mut lexer = Lexer::new(line);
            let parse_result = Parser::new(&lexer.lex()).parse();
            assert_eq!(parse_result.unwrap(), expect);
        }
    }
}