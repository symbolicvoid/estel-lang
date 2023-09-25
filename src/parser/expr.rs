use super::token::*;

//using PartialOrd so we can compare the enum to get its precedence
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expr{
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Literal(Literal)
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

    //Merges two expressions together on the order of their precedence
    //For example: Merge(Add(8, 4), Mul(4, 6)) becomes:
    // Add(8, Mul(4, 6))
    pub fn merge(self, other: Expr) -> Expr{
        if self < other{
            return other.inverse().merge(self).inverse();
        }
        //Bring the enums into scope for ease in reading
        use Expr::*;
        //Compare the enums based on their order using PartialOrd
        match self.clone(){
            Mul(left, _ ) => {
                Mul(left, Box::new(other))
            }
            Div(left, _ ) => {
                Div(left, Box::new(other))
            } 
            Add(left, _ ) => {
                Add(left, Box::new(other))
            } 
            Sub(left, _ ) => {
                Sub(left, Box::new(other))
            } 
            _ => {
                self
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
            Expr::Add(
                Box::new(left),
                Box::new(Expr::new_num_literal(8))
            )
        )
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