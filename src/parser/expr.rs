use super::token::*;

//using PartialOrd so we can compare the enum to get its precedence
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expr{
    //Meant to represent lack of expression
    None,
    Literal(Literal),
    //(expr)
    Paren(Box<Expr>),
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
    pub fn new_paren(expr: Expr) -> Expr{
        Expr::Paren(Box::new(expr))
    }
    pub fn new_literal(literal: &Literal) -> Expr{
        Expr::Literal(literal.to_owned())
    }
    #[allow(dead_code)]
    pub fn new_num_literal(num: i32) -> Expr{
        Expr::Literal(Literal::Number(num))
    }
    pub fn new_binary_op(left: Expr, right: Expr, opr: &Operator) -> Expr{
        match opr{
            Operator::Add => {
                Expr::new_add(left, right)
            }
            Operator::Sub => {
                Expr::new_sub(left, right)
            }
            Operator::Mul => {
                Expr::new_mul(left, right)
            }
            Operator::Div => {
                Expr::new_div(left, right)
            }
        }
    }

    pub fn solve(&self) -> Option<Literal>{
        match self{
            Expr::None => panic!("None expression inside the AST!"),
            Expr::Paren(expr) => {
                expr.solve()
            }
            //Division operation can only be done between two numbers
            Expr::Div(left, right) => {
                let left = match left.solve(){
                    Some(literal) => literal.get_number(),
                    None => return None,
                };
                let right = match right.solve(){
                    Some(literal) => literal.get_number(),
                    None => return None,
                };
                if left == None || right == None{
                    None
                } else {
                    Some(Literal::Number(left.unwrap() / right.unwrap()))
                }
            }
            //Multiplication can be done between two numbers, and a string and a number
            //"Hello" * 2  => "HelloHello" 
            Expr::Mul(left, right) => {
                let left = left.solve();
                let right = match right.solve(){
                    Some(literal) => literal.get_number(),
                    None => return None,
                };
                if left == None || right == None{
                    None
                } else {
                    match left.unwrap(){
                        Literal::Number(num) => {
                            Some(Literal::Number(num * right.unwrap()))
                        }
                        Literal::String(string) => {
                            let mut result = String::new();
                            for _ in 0..right.unwrap(){
                                result.push_str(&string);
                            }
                            Some(Literal::String(result))
                        }
                    }
                }
            }
            //Can add both Strings and Numbers
            Expr::Add(left, right) => {
                let left = left.solve();
                let right = right.solve();
                if left == None || right == None{
                    None
                } else {
                    match (left.unwrap(), right.unwrap()){
                        (Literal::Number(left), Literal::Number(right)) => {
                            Some(Literal::Number(left + right))
                        }
                        (Literal::String(left), Literal::String(right)) => {
                            Some(Literal::String(left + &right))
                        }
                        (Literal::String(left), Literal::Number(right)) => {
                            let mut result = String::new();
                            for _ in 0..right{
                                result.push_str(&left);
                            }
                            Some(Literal::String(result))
                        }
                        (Literal::Number(left), Literal::String(right)) => {
                            let mut result = String::new();
                            for _ in 0..left{
                                result.push_str(&right);
                            }
                            Some(Literal::String(result))
                        }
                    }
                }
            }
            //Can only subtract numbers
            Expr::Sub(left, right) => {
                let left = match left.solve(){
                    Some(literal) => literal.get_number(),
                    None => return None,
                };
                let right = match right.solve(){
                    Some(literal) => literal.get_number(),
                    None => return None,
                };
                if left == None || right == None{
                    None
                } else {
                    Some(Literal::Number(left.unwrap() - right.unwrap()))
                }
            }
            Expr::Literal(literal) => {
                Some(literal.clone())
            }
        }
    }

    //Merges two expressions together based on the order of their precedence
    //For example: Merge(Add(8, 4), Mul(4, 6)) becomes:
    // Add(8, Mul(4, 6))
    pub fn merge(self, other: Expr) -> Expr{
        //Bring the enums into scope for ease in reading
        use Expr::*;
        //return other if self is a literal
        if let Literal(_) = self{
            return other;
        } else if self < other { //Compare the enums based on their order using PartialOrd
            //If the other expr has a higher precedence, we want to put self at the left of the expr
            match other{
                Div(left, right ) => {
                    Div(Box::new(self.merge(*left)), right)
                } 
                Mul(left, right ) => {
                    Mul(Box::new(self.merge(*left)), right)
                }
                Add(left, right ) => {
                    Add(Box::new(self.merge(*left)), right)
                }
                Sub(left, right ) => {
                    Sub(Box::new(self.merge(*left)), right)
                }
                _ => {
                    other
                }
            }
        } else {
            //Place the other expr to the right of the expr
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
                Paren(_) => self,
                _ => {
                    other
                }
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

        //5*5+2-8
        let exprs = vec![
            Expr::new_mul(
                Expr::new_num_literal(5),
                Expr::new_num_literal(5)
            ),
            Expr::new_add(
                Expr::new_num_literal(5),
                Expr::new_num_literal(2)
            ),
            Expr::new_sub(
                Expr::new_num_literal(2),
                Expr::new_num_literal(8)
            )
        ];
        assert_eq!(
            exprs[0].clone().merge(exprs[1].clone()).merge(exprs[2].clone()),
            Expr::new_sub(
                Expr::new_add(
                    Expr::new_mul(
                        Expr::new_num_literal(5),
                        Expr::new_num_literal(5)
                    ),
                    Expr::new_num_literal(2)
                ),
                Expr::new_num_literal(8)
            )
        );
    }

    #[test]
    fn solve_numeric_exprs(){
        let exprs = vec!(
            //5*5+3
            Expr::new_add(
                Expr::new_mul(Expr::new_num_literal(5), Expr::new_num_literal(5)), 
                Expr::new_num_literal(3)
            ),
            //(4)*(5)
            Expr::new_mul(
                Expr::new_paren(Expr::new_num_literal(4)), 
                Expr::new_paren(Expr::new_num_literal(5))
            ),
            //5*(5+3)
            Expr::new_mul(
                Expr::new_num_literal(5), 
                Expr::new_paren(
                    Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3))
                )
            ),
            //5*(5+3)*2
            Expr::new_mul(
                Expr::new_mul(
                    Expr::new_num_literal(5), 
                    Expr::new_paren(
                        Expr::new_add(Expr::new_num_literal(5), Expr::new_num_literal(3))
                    )
                ),
                Expr::new_num_literal(2)
            ),
            //(4-2)*7/(4+3)
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
            //(5-5)*(2+8)
            Expr::new_mul(
                Expr::new_paren(
                    Expr::new_sub(Expr::new_num_literal(5), Expr::new_num_literal(5))
                ),
                Expr::new_paren(
                    Expr::new_add(Expr::new_num_literal(2), Expr::new_num_literal(8))
                )
            ),
        );
        let solns = vec!(
            Some(Literal::Number(28)),
            Some(Literal::Number(20)),
            Some(Literal::Number(40)),
            Some(Literal::Number(80)),
            Some(Literal::Number(2)),
            Some(Literal::Number(0)),
        );
        for (expr, soln) in exprs.iter().zip(solns.iter()){
            assert_eq!(expr.solve(), *soln);
        }
    }
}