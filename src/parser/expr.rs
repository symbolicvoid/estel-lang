use super::token::*;

#[derive(PartialEq, Debug)]
pub struct Expr{
    pub expr: ExprType,
}

#[derive(PartialEq, Debug)]
pub enum ExprType{
    Mul(Box<ExprType>, Box<ExprType>),
    Div(Box<ExprType>, Box<ExprType>),
    Add(Box<ExprType>, Box<ExprType>),
    Sub(Box<ExprType>, Box<ExprType>),
    Literal(Literal)
}

impl ExprType{
    pub fn new_add(left: ExprType, right: ExprType) -> ExprType{
        ExprType::Add(Box::new(left), Box::new(right))
    }
    pub fn new_sub(left: ExprType, right: ExprType) -> ExprType{
        ExprType::Sub(Box::new(left), Box::new(right))
    }
    pub fn new_mul(left: ExprType, right: ExprType) -> ExprType{
        ExprType::Mul(Box::new(left), Box::new(right))
    }
    pub fn new_div(left: ExprType, right: ExprType) -> ExprType{
        ExprType::Div(Box::new(left), Box::new(right))
    }
    pub fn new_num_literal(num: i32) -> ExprType{
        ExprType::Literal(Literal::Number(num))
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn make_num_literal(){
        assert_eq!(ExprType::Literal(Literal::Number(8)), ExprType::new_num_literal(8));
    }

    #[test]
    fn make_basic_exprs(){

        let literal = Literal::Number(8);

        assert_eq!(
            ExprType::Add(
                Box::new(ExprType::Literal(literal.clone())), 
                Box::new(ExprType::Literal(literal.clone()))
            ),
            ExprType::new_add(ExprType::Literal(literal.clone()), ExprType::Literal(literal.clone()))
        ); 
        assert_eq!(
            ExprType::Sub(
                Box::new(ExprType::Literal(literal.clone())), 
                Box::new(ExprType::Literal(literal.clone()))
            ),
            ExprType::new_sub(ExprType::Literal(literal.clone()), ExprType::Literal(literal.clone()))
        ); 
        assert_eq!(
            ExprType::Mul(
                Box::new(ExprType::Literal(literal.clone())), 
                Box::new(ExprType::Literal(literal.clone()))
            ),
            ExprType::new_mul(ExprType::Literal(literal.clone()), ExprType::Literal(literal.clone()))
        ); 
        assert_eq!(
            ExprType::Div(
                Box::new(ExprType::Literal(literal.clone())), 
                Box::new(ExprType::Literal(literal.clone()))
            ),
            ExprType::new_div(ExprType::Literal(literal.clone()), ExprType::Literal(literal.clone()))
        ); 
    }
}