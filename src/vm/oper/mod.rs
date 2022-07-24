use super::{Value, value};

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub fn unsupported(lhs: Value, rhs: Value) -> OperatorResult {
    Err(format!("Unsupported operator for types {} and {}", value::type_name(lhs), value::type_name(rhs)))
}

pub mod arithmetic;
pub mod bitwise;
pub mod comparison;
