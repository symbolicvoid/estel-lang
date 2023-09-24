use super::token::*;
use super::expr::*;

pub struct Parser{
    tokens: Vec<Token>,
    pos: u32,
}

impl Parser{
    pub fn new(tokens: Vec<Token>) -> Parser{
        Self { tokens, pos: 0 }
    }


    //parse the tokens into an expression
    pub fn parse(&mut self) -> Expr{
        Expr { expr: ExprType::new_num_literal(8) }
    }

    fn peek(&self) -> &TokenType{
        let pos = self.pos as usize + 1;
        if self.is_eof(pos){
            &TokenType::EOF
        } else {
            &self.tokens[pos].class
        }
    }

    fn advance(&mut self) ->  &TokenType{
        let pos = self.pos as usize;
        if self.is_eof(pos){
            return &TokenType::EOF;
        } 
        self.pos += 1;
        &self.tokens[pos].class
    }

    fn is_eof(&self, pos: usize) -> bool{
        if pos>= self.tokens.len(){
            true
        } else {
            false
        }
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
            Expr{
                expr: ExprType::new_add(ExprType::new_num_literal(4), ExprType::new_num_literal(5))
            },
            Expr{
                expr: ExprType::new_sub(ExprType::new_num_literal(7), ExprType::new_num_literal(2))
            },
            Expr{
                expr: ExprType::new_mul(ExprType::new_num_literal(8), ExprType::new_num_literal(2))
            },
            Expr{
                expr: ExprType::new_div(ExprType::new_num_literal(5), ExprType::new_num_literal(5))
            },
        );
        for (line, expect) in src.iter().zip(expected){
            let mut lexer = Lexer::new(line);
            let parse_result = Parser::new(lexer.lex()).parse();
            assert_eq!(parse_result, expect, "Expected expression: {:?}, Got: {:?}", expect, parse_result);
        }
    }
}