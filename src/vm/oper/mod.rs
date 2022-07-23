use super::Value;

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub mod arithmetic;
