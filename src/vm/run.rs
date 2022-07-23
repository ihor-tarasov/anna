use super::{State, VMResult, Opcode};
use super::oper::arithmetic::*;

pub fn run(mut state: State) -> VMResult {
    loop {
        match state.next_opcode()? {
            Opcode::Const(index) => state.push_constant(index)?,
            Opcode::Addict => state.binary::<ArithmeticOperator<AddictArithmetic>>()?,
            Opcode::Subtract => state.binary::<ArithmeticOperator<SubtractArithmetic>>()?,
            Opcode::Multiply => state.binary::<ArithmeticOperator<MultiplyArithmetic>>()?,
            Opcode::Divide => state.binary::<ArithmeticOperator<DivideArithmetic>>()?,
            Opcode::End => return Ok(state.pop().unwrap()),
        }
    }
}
