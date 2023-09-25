use super::token::*;
use super::expr::*;

pub struct Parser{
    tokens: Vec<Token>,
    pos: u32,
}

#[allow(unused)]
impl Parser{
    pub fn new(tokens: Vec<Token>) -> Parser{
        Self { tokens, pos: 0 }
    }

    //parse the tokens into an expression
    pub fn parse(&mut self) -> Option<Expr>{
        self.expr()
    }

    fn expr(&mut self) -> Option<Expr>{
        let expr: Expr;
        match &self.get_current_token().class{
            TokenType::Literal(left) => {
                //match the next token
                match &self.get_current_token().class{
                    TokenType::Operator(opr) => {
                        let right = match &self.peek().class{
                            TokenType::Literal(right) => {
                                right
                            },
                            _ => {
                                return None;
                            }
                        };
                        expr = Expr::new_binary_op(left, right, opr);
                        if let Some(next_expr) = self.expr(){
                            return Some(expr.merge(next_expr));
                        } else {
                            return Some(expr);
                        }
                    },
                    _ => {
                        None
                    }
                }
            }
            _ => {
                None
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

    //advances the position and returns the consumed token
    fn consume(&mut self) -> &Token{
        let consumed = &self.tokens[self.pos as usize];
        self.pos += 1;
        consumed
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

    fn is_eof(&self, pos: usize) -> bool{
        pos>= self.tokens.len()
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
            let parse_result = Parser::new(lexer.lex()).parse();
            assert_eq!(parse_result.clone().unwrap(), expect, "Expected expression: {:?}, Got: {:?}", expect, parse_result);
        }
    }
}