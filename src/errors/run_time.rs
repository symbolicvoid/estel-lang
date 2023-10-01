#[derive(Debug, PartialEq, Clone)]
pub enum LiteralOpError {
    InvalidTypeError,
    DivByZeroError,
    UndefinedVariableError,
}
