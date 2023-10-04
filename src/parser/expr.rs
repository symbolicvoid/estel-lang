use super::errors::LiteralOpError;
use super::{stmt::Block, token::*};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr {
    //--------------------
    //Constructor funtions
    //--------------------
    pub fn new_add(left: Expr, right: Expr) -> Expr {
        Expr::Add(Box::new(left), Box::new(right))
    }
    pub fn new_sub(left: Expr, right: Expr) -> Expr {
        Expr::Sub(Box::new(left), Box::new(right))
    }
    pub fn new_mul(left: Expr, right: Expr) -> Expr {
        Expr::Mul(Box::new(left), Box::new(right))
    }
    pub fn new_div(left: Expr, right: Expr) -> Expr {
        Expr::Div(Box::new(left), Box::new(right))
    }
    pub fn new_literal(literal: &Literal) -> Expr {
        Expr::Literal(literal.to_owned())
    }
    pub fn new_ident(ident: &str) -> Expr {
        Expr::Ident(ident.to_owned())
    }
    #[allow(dead_code)]
    pub fn new_num_literal(num: i32) -> Expr {
        Expr::Literal(Literal::Number(num))
    }
    pub fn new_binary_op(left: Expr, right: Expr, opr: &Operator) -> Expr {
        match opr {
            Operator::Add => Expr::new_add(left, right),
            Operator::Sub => Expr::new_sub(left, right),
            Operator::Mul => Expr::new_mul(left, right),
            Operator::Div => Expr::new_div(left, right),
        }
    }

    pub fn solve(&self, block: &Block) -> Result<Literal, LiteralOpError> {
        match self {
            //Division operation can only be done between two numbers
            Expr::Div(left, right) => {
                let left = left.solve(block)?;
                let right = right.solve(block)?;
                left.div(right)
            }
            //Multiplication can be done between two numbers, and a string and a number
            //"Hello" * 2  => "HelloHello"
            Expr::Mul(left, right) => {
                let left = left.solve(block)?;
                let right = right.solve(block)?;
                left.mul(right)
            }
            //Can add both Strings and Numbers
            Expr::Add(left, right) => {
                let left = left.solve(block)?;
                let right = right.solve(block)?;
                left.add(right)
            }
            //Can only subtract numbers
            Expr::Sub(left, right) => {
                let left = left.solve(block)?;
                let right = right.solve(block)?;
                left.sub(right)
            }
            Expr::Literal(literal) => Ok(literal.to_owned()),
            Expr::Ident(name) => match block.get_var(name) {
                Some(literal) => Ok(literal.to_owned()),
                None => Err(LiteralOpError::UndefinedVariableError),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpectType {
    Operand,
    Operator,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn make_num_literal() {
        assert_eq!(Expr::Literal(Literal::Number(8)), Expr::new_num_literal(8));
    }

    #[test]
    fn make_basic_exprs() {
        let literal = Literal::Number(8);

        assert_eq!(
            Expr::Add(
                Box::new(Expr::Literal(literal.clone())),
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_add(
                Expr::Literal(literal.clone()),
                Expr::Literal(literal.clone())
            )
        );
        assert_eq!(
            Expr::Sub(
                Box::new(Expr::Literal(literal.clone())),
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_sub(
                Expr::Literal(literal.clone()),
                Expr::Literal(literal.clone())
            )
        );
        assert_eq!(
            Expr::Mul(
                Box::new(Expr::Literal(literal.clone())),
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_mul(
                Expr::Literal(literal.clone()),
                Expr::Literal(literal.clone())
            )
        );
        assert_eq!(
            Expr::Div(
                Box::new(Expr::Literal(literal.clone())),
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_div(
                Expr::Literal(literal.clone()),
                Expr::Literal(literal.clone())
            )
        );
    }

    #[test]
    fn solve_numeric_exprs() {
        let exprs = vec![
            //5*5+3
            Expr::new_add(
                Expr::new_mul(Expr::new_num_literal(5), Expr::new_num_literal(5)),
                Expr::new_num_literal(3),
            ),
            //(4)*(5)
            Expr::new_mul(Expr::new_num_literal(4), Expr::new_num_literal(5)),
            //5*(5+3)
            Expr::new_mul(
                Expr::new_num_literal(5),
                Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3)),
            ),
            //5*(5+3)*2
            Expr::new_mul(
                Expr::new_mul(
                    Expr::new_num_literal(5),
                    Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3)),
                ),
                Expr::new_num_literal(2),
            ),
            //(4-2)*7/(4+3)
            Expr::new_mul(
                Expr::new_sub(Expr::new_num_literal(4), Expr::new_num_literal(2)),
                Expr::new_div(
                    Expr::new_num_literal(7),
                    Expr::new_add(Expr::new_num_literal(4), Expr::new_num_literal(3)),
                ),
            ),
            //(5-5)*(2+8)
            Expr::new_mul(
                Expr::new_sub(Expr::new_num_literal(5), Expr::new_num_literal(5)),
                Expr::new_add(Expr::new_num_literal(2), Expr::new_num_literal(8)),
            ),
        ];
        let solns = vec![
            Literal::Number(28),
            Literal::Number(20),
            Literal::Number(40),
            Literal::Number(80),
            Literal::Float(2.0),
            Literal::Number(0),
        ];
        for (expr, soln) in exprs.iter().zip(solns.iter()) {
            assert_eq!(expr.solve(&Block::new(Vec::new(), None)).unwrap(), *soln);
        }
    }
}
