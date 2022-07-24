use std::marker::PhantomData;

use crate::vm::{Value, Real};

use super::{OperatorResult, BinaryOperator};

pub trait ComparisonType {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult;
}

pub struct EqualComparison;

impl ComparisonType for EqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs == rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs == rhs))
    }
}

pub struct NotEqualComparison;

impl ComparisonType for NotEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs != rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs != rhs))
    }
}

pub struct LessComparison;

impl ComparisonType for LessComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs < rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs.data < rhs.data))
    }
}

pub struct GreaterComparison;

impl ComparisonType for GreaterComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs > rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs.data > rhs.data))
    }
}

pub struct LessEqualComparison;

impl ComparisonType for LessEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs <= rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs.data <= rhs.data))
    }
}

pub struct GreaterEqualComparison;

impl ComparisonType for GreaterEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs >= rhs))
    }

    fn eval_real(lhs: Real, rhs: Real) -> OperatorResult {
        Ok(Value::Bool(lhs.data >= rhs.data))
    }
}

pub struct ComparisonOperator<T: ComparisonType> {
    phantom: PhantomData<T>,
}

impl<T: ComparisonType> BinaryOperator for ComparisonOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => T::eval_int(lhs, rhs),
            (Value::Int(lhs), Value::Real(rhs)) => T::eval_real(Real::new(lhs as f64), rhs),
            (Value::Real(lhs), Value::Int(rhs)) => T::eval_real(lhs, Real::new(rhs as f64)),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs, rhs),
            _ => super::unsupported(lhs, rhs),
        }
    }
}
