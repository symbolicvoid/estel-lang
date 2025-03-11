use super::errors::LiteralOpError;
use super::executor::Executor;
use super::token::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Negate(Box<Expr>),
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
    pub fn new_greater(left: Expr, right: Expr) -> Expr {
        Expr::Greater(Box::new(left), Box::new(right))
    }
    pub fn new_less(left: Expr, right: Expr) -> Expr {
        Expr::Less(Box::new(left), Box::new(right))
    }
    pub fn new_greater_equal(left: Expr, right: Expr) -> Expr {
        Expr::GreaterEqual(Box::new(left), Box::new(right))
    }
    pub fn new_less_equal(left: Expr, right: Expr) -> Expr {
        Expr::LessEqual(Box::new(left), Box::new(right))
    }
    pub fn new_equal(left: Expr, right: Expr) -> Expr {
        Expr::Equal(Box::new(left), Box::new(right))
    }
    pub fn new_not_equal(left: Expr, right: Expr) -> Expr {
        Expr::NotEqual(Box::new(left), Box::new(right))
    }
    pub fn new_and(left: Expr, right: Expr) -> Expr {
        Expr::And(Box::new(left), Box::new(right))
    }
    pub fn new_or(left: Expr, right: Expr) -> Expr {
        Expr::Or(Box::new(left), Box::new(right))
    }
    pub fn new_literal(literal: &Literal) -> Expr {
        Expr::Literal(literal.to_owned())
    }
    pub fn new_ident(ident: &str) -> Expr {
        Expr::Ident(ident.to_owned())
    }

    //functions used to simplify writing tests
    #[allow(dead_code)]
    pub fn new_num_literal(num: i32) -> Expr {
        Expr::Literal(Literal::Number(num))
    }

    #[allow(dead_code)]
    pub fn new_string_literal(string: &str) -> Expr {
        Expr::Literal(Literal::String(string.to_owned()))
    }

    #[allow(dead_code)]
    pub fn new_bool_literal(boolean: bool) -> Expr {
        Expr::Literal(Literal::Bool(boolean))
    }
    //---------------------------------------------

    pub fn new_binary_op(left: Expr, right: Expr, opr: &Operator) -> Expr {
        match opr {
            Operator::Add => Expr::new_add(left, right),
            Operator::Sub => Expr::new_sub(left, right),
            Operator::Mul => Expr::new_mul(left, right),
            Operator::Div => Expr::new_div(left, right),
            Operator::Greater => Expr::new_greater(left, right),
            Operator::Less => Expr::new_less(left, right),
            Operator::GreaterEqual => Expr::new_greater_equal(left, right),
            Operator::LessEqual => Expr::new_less_equal(left, right),
            Operator::Equal => Expr::new_equal(left, right),
            Operator::NotEqual => Expr::new_not_equal(left, right),
            Operator::And => Expr::new_and(left, right),
            Operator::Or => Expr::new_or(left, right),
        }
    }

    pub fn new_unary_op(expr: Expr, opr: &Unary) -> Expr {
        match opr {
            Unary::Not => Expr::Not(Box::new(expr)),
            Unary::Neg => Expr::Negate(Box::new(expr)),
        }
    }

    pub fn solve(&self, executor: &Executor) -> Result<Literal, LiteralOpError> {
        match self {
            //Division operation can only be done between two numbers
            Expr::Div(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.div(right)
            }
            //Multiplication can be done between two numbers, and a string and a number
            //"Hello" * 2  => "HelloHello"
            Expr::Mul(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.mul(right)
            }
            //Can add both Strings and Numbers
            Expr::Add(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.add(right)
            }
            //Can only subtract numbers
            Expr::Sub(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.sub(right)
            }
            Expr::Greater(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.greater(right)
            }
            Expr::Less(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.less(right)
            }
            Expr::GreaterEqual(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.greater_equal(right)
            }
            Expr::LessEqual(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                left.less_equal(right)
            }
            Expr::Equal(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                Ok(left.equal(right))
            }
            Expr::NotEqual(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                Ok(left.not_equal(right))
            }
            Expr::And(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                Ok(left.and(right))
            }
            Expr::Or(left, right) => {
                let left = left.solve(executor)?;
                let right = right.solve(executor)?;
                Ok(left.or(right))
            }
            Expr::Not(expr) => {
                let expr = expr.solve(executor)?;
                Ok(expr.not())
            }
            Expr::Negate(expr) => {
                let expr = expr.solve(executor)?;
                expr.negate()
            }
            Expr::Ident(name) => match executor.get_var(name) {
                Some(literal) => Ok(literal),
                None => Err(LiteralOpError::UndefinedVariableError),
            },
            Expr::Literal(literal) => Ok(literal.to_owned()),
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

    use crate::parser::executor::Scope;

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
        let exprs = [
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
        let solns = [
            Literal::Number(28),
            Literal::Number(20),
            Literal::Number(40),
            Literal::Number(80),
            Literal::Float(2.0),
            Literal::Number(0),
        ];
        for (expr, soln) in exprs.iter().zip(solns.iter()) {
            assert_eq!(
                expr.solve(&Executor::new(false, Scope::new())).unwrap(),
                *soln
            );
        }
    }

    #[test]
    fn solve_relational_ops() {
        let exprs = [
            //9>9
            Expr::new_greater(Expr::new_num_literal(9), Expr::new_num_literal(9)),
            //6<8
            Expr::new_less(Expr::new_num_literal(6), Expr::new_num_literal(8)),
            //5>=5
            Expr::new_greater_equal(Expr::new_num_literal(5), Expr::new_num_literal(5)),
            //12<=8
            Expr::new_less_equal(Expr::new_num_literal(12), Expr::new_num_literal(8)),
            //5==5
            Expr::new_equal(Expr::new_num_literal(5), Expr::new_num_literal(5)),
            //7!=7
            Expr::new_not_equal(Expr::new_num_literal(7), Expr::new_num_literal(7)),
            //7==7 and 8!=8
            Expr::new_and(
                Expr::new_equal(Expr::new_num_literal(7), Expr::new_num_literal(7)),
                Expr::new_not_equal(Expr::new_num_literal(8), Expr::new_num_literal(8)),
            ),
            //7==7 or 8!=8
            Expr::new_or(
                Expr::new_equal(Expr::new_num_literal(7), Expr::new_num_literal(7)),
                Expr::new_not_equal(Expr::new_num_literal(8), Expr::new_num_literal(8)),
            ),
            //"" or 1
            Expr::new_or(
                Expr::new_literal(&Literal::String("".to_owned())),
                Expr::new_literal(&Literal::Number(1)),
            ),
            //"" and true
            Expr::new_and(
                Expr::new_literal(&Literal::String("".to_owned())),
                Expr::new_literal(&Literal::Bool(true)),
            ),
        ];
        let solns = [
            Literal::Bool(false),
            Literal::Bool(true),
            Literal::Bool(true),
            Literal::Bool(false),
            Literal::Bool(true),
            Literal::Bool(false),
            Literal::Bool(false),
            Literal::Bool(true),
            Literal::Bool(true),
            Literal::Bool(false),
        ];
        for (expr, soln) in exprs.iter().zip(solns.iter()) {
            assert_eq!(
                expr.solve(&Executor::new(false, Scope::new())).unwrap(),
                *soln
            );
        }
    }
}
