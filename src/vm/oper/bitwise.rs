use std::marker::PhantomData;

use crate::vm::Value;

use super::{OperatorResult, BinaryOperator};

pub trait BitwiseType {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult;
}

pub struct ModBitwise;

impl BitwiseType for ModBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs % rhs))
    }
}

pub struct AndBitwise;

impl BitwiseType for AndBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs & rhs))
    }
}

pub struct OrBitwise;

impl BitwiseType for OrBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs | rhs))
    }
}

pub struct XorBitwise;

impl BitwiseType for XorBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Int(lhs ^ rhs))
    }
}

pub struct ShlBitwise;

impl BitwiseType for ShlBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs < 0 {
            Err(format!("Cant to shifl left by {}.", rhs))
        } else {
            let rhs = rhs as u32;
            Ok(Value::Int(lhs.wrapping_shl(rhs)))
        }
    }
}

pub struct ShrBitwise;

impl BitwiseType for ShrBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs < 0 {
            Err(format!("Cant to shifl right by {}.", rhs))
        } else {
            let rhs = rhs as u32;
            Ok(Value::Int(lhs.wrapping_shr(rhs)))
        }
    }
}

pub struct BitwiseOperator<T: BitwiseType> {
    phantom: PhantomData<T>,
}

impl<T: BitwiseType> BinaryOperator for BitwiseOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => T::eval(lhs, rhs),
            _ => super::unsupported(lhs, rhs),
        }
    }
}
