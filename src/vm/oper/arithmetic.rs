use super::{OperatorResult, Value, BinaryOperator};

pub trait ArithmeticType {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
}

pub struct AddictArithmetic;

impl ArithmeticType for AddictArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs.wrapping_add(rhs)))
    }
}

pub struct MultiplyArithmetic;

impl ArithmeticType for MultiplyArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs.wrapping_mul(rhs)))
    }
}

pub struct ArithmeticOperator<T: ArithmeticType> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: ArithmeticType> BinaryOperator for ArithmeticOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => T::eval_int(lhs, rhs),
        }
    }
}
