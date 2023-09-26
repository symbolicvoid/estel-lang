use super::token::*;

//using PartialOrd so we can compare the enum to get its precedence
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expr{
    Literal(Literal),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr{
    //--------------------
    //Constructor funtions
    //--------------------
    pub fn new_add(left: Expr, right: Expr) -> Expr{
        Expr::Add(Box::new(left), Box::new(right))
    }
    pub fn new_sub(left: Expr, right: Expr) -> Expr{
        Expr::Sub(Box::new(left), Box::new(right))
    }
    pub fn new_mul(left: Expr, right: Expr) -> Expr{
        Expr::Mul(Box::new(left), Box::new(right))
    }
    pub fn new_div(left: Expr, right: Expr) -> Expr{
        Expr::Div(Box::new(left), Box::new(right))
    }
    pub fn new_literal(literal: &Literal) -> Expr{
        Expr::Literal(literal.to_owned())
    }
    pub fn new_num_literal(num: i32) -> Expr{
        Expr::Literal(Literal::Number(num))
    }
    pub fn new_binary_op(left: &Literal, right: &Literal, opr: &Operator) -> Expr{
        match opr{
            Operator::Add => {
                Expr::new_add(Expr::Literal(left.clone()), Expr::Literal(right.clone()))
            }
            Operator::Sub => {
                Expr::new_sub(Expr::Literal(left.clone()), Expr::Literal(right.clone()))
            }
            Operator::Mul => {
                Expr::new_mul(Expr::Literal(left.clone()), Expr::Literal(right.clone()))
            }
            Operator::Div => {
                Expr::new_div(Expr::Literal(left.clone()), Expr::Literal(right.clone()))
            }
        }
    }

    //Merges two expressions together based on the order of their precedence
    //For example: Merge(Add(8, 4), Mul(4, 6)) becomes:
    // Add(8, Mul(4, 6))
    pub fn merge(self, other: Expr) -> Expr{
        //return other if self is a literal
        if let Expr::Literal(_) = self{
            return other;
        } else if self < other { //Compare the enums based on their order using PartialOrd
            return other.inverse().merge(self).inverse();
        }
        //Bring the enums into scope for ease in reading
        use Expr::*;
        match self{
            Div(left, right ) => {
                Div(left, Box::new(right.merge(other)))
            } 
            Mul(left, right ) => {
                Mul(left, Box::new(right.merge(other)))
            }
            Add(left, right ) => {
                Add(left, Box::new(right.merge(other)))
            } 
            Sub(left, right ) => {
                Sub(left, Box::new(right.merge(other)))
            } 
            _ => {
                other
            }
        }
    }

    //Swap the left and right expressions of an expression
    fn inverse(self) -> Expr{
        use Expr::*;
        match self{
            Mul(left, right) => {
                Mul(right, left)
            } 
            Div(left, right) => {
                Div(right, left)
            } 
            Sub(left, right) => {
                Sub(right, left)
            } 
            Add(left, right) => {
                Add(right, left)
            }
            _ => {
                self
            }
        }
    }
}

pub enum ParseError{
    UnexpectedToken,
}

#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;

    #[test]
    fn make_num_literal(){
        assert_eq!(Expr::Literal(Literal::Number(8)), Expr::new_num_literal(8));
    }

    #[test]
    fn make_basic_exprs(){

        let literal = Literal::Number(8);

        assert_eq!(
            Expr::Add(
                Box::new(Expr::Literal(literal.clone())), 
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_add(Expr::Literal(literal.clone()), Expr::Literal(literal.clone()))
        ); 
        assert_eq!(
            Expr::Sub(
                Box::new(Expr::Literal(literal.clone())), 
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_sub(Expr::Literal(literal.clone()), Expr::Literal(literal.clone()))
        ); 
        assert_eq!(
            Expr::Mul(
                Box::new(Expr::Literal(literal.clone())), 
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_mul(Expr::Literal(literal.clone()), Expr::Literal(literal.clone()))
        ); 
        assert_eq!(
            Expr::Div(
                Box::new(Expr::Literal(literal.clone())), 
                Box::new(Expr::Literal(literal.clone()))
            ),
            Expr::new_div(Expr::Literal(literal.clone()), Expr::Literal(literal.clone()))
        ); 
    }

    #[test]
    fn merge_exprs(){
        // 5 + 3 * 8
        let left = Expr::new_add(
            Expr::new_num_literal(5), 
            Expr::new_num_literal(3)
        );
        let right = Expr::new_mul(
            Expr::new_num_literal(3), 
            Expr::new_num_literal(8)
        );

        assert_eq!(
            left.merge(right.clone()), 
            // 5+(3*8)
            Expr::Add(
                Box::new(Expr::new_num_literal(5)),
                Box::new(right)
            )
        );
        
        // 5 * 3 + 8
        let left = Expr::new_mul(
            Expr::new_num_literal(5), 
            Expr::new_num_literal(3)
        );
        let right = Expr::new_add(
            Expr::new_num_literal(3), 
            Expr::new_num_literal(8)
        );
        
        assert_eq!(
            left.clone().merge(right),
            //(5*3)+8
            Expr::Add(
                Box::new(left),
                Box::new(Expr::new_num_literal(8))
            )
        );

        //5*5/6*6-2
        let exprs = vec![
            Expr::new_mul(
                Expr::new_num_literal(5),
                Expr::new_num_literal(5)
            ),
            Expr::new_div(
                Expr::new_num_literal(5),
                Expr::new_num_literal(6)
            ),
            Expr::new_mul(
                Expr::new_num_literal(6),
                Expr::new_num_literal(6)
            ),
            Expr::new_sub(
                Expr::new_num_literal(6),
                Expr::new_num_literal(2)
            )
        ];

        assert_eq!(
            exprs[0].clone().merge(exprs[1].clone()).merge(exprs[2].clone()).merge(exprs[3].clone()),
            Expr::new_sub(
                Expr::new_mul(
                    Expr::new_mul(
                        Expr::new_num_literal(5),
                        Expr::new_div(
                            Expr::new_num_literal(5),
                            Expr::new_num_literal(6)
                        )
                    ),
                    Expr::new_num_literal(6)
                ),
                Expr::new_num_literal(2)
            )
        );
    }

    #[test]
    fn inverse_exprs(){
        let expr = Expr::new_mul(
            Expr::new_num_literal(3), 
            Expr::new_mul(Expr::new_num_literal(4), Expr::new_num_literal(8))
        );

        assert_eq!(
            expr.inverse(),
            Expr::new_mul(
                Expr::new_mul(Expr::new_num_literal(4), Expr::new_num_literal(8)),
                Expr::new_num_literal(3), 
            )
        )
    }
}