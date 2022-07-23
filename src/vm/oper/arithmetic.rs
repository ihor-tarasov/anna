use crate::vm::Real;

use super::{OperatorResult, Value, BinaryOperator};

pub trait ArithmeticType {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult;
}

pub struct AddictArithmetic;

impl ArithmeticType for AddictArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs.wrapping_add(rhs)))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Real(Real::new(lhs.data + rhs.data )))
    }
}

pub struct SubtractArithmetic;

impl ArithmeticType for SubtractArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs.wrapping_sub(rhs)))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Real(Real::new(lhs.data - rhs.data )))
    }
}

pub struct MultiplyArithmetic;

impl ArithmeticType for MultiplyArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs.wrapping_mul(rhs)))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Real(Real::new(lhs.data * rhs.data )))
    }
}

pub struct DivideArithmetic;

impl ArithmeticType for DivideArithmetic {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(format!("Dividing by zero."))
        } else {
            Ok(Value::Int(lhs.wrapping_div(rhs)))
        }
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Real(Real::new(lhs.data / rhs.data )))
    }
}

pub struct ArithmeticOperator<T: ArithmeticType> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: ArithmeticType> BinaryOperator for ArithmeticOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => T::eval_int(lhs, rhs),
            (Value::Int(lhs), Value::Real(rhs)) => T::eval_real(Real::new(lhs as f64), rhs),
            (Value::Real(lhs), Value::Int(rhs)) => T::eval_real(lhs, Real::new(rhs as f64)),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs, rhs),
        }
    }
}
